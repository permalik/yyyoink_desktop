#include <QColor>
#include <QString>
#include <QStringList>
#include <QVariant>
#include <cstdlib>
#include <cstring>
#include <dirent.h>
#include <filesystem>
#include <fstream>
#include <future>
#include <mutex>
#include <sys/stat.h>
#include <unistd.h>
#include "designfiles.h"

namespace fs = std::filesystem;

// /Users/tymalik/Documents/git/artchive_data

std::string sys_file_path() {
    std::string current_user;
    if (const char *system_user = std::getenv("USER")) {
        current_user = std::string(system_user);
    }
    return "/Users/" + current_user + "/.config/artchive_directory_path.txt";
}

std::string bak_file_path() {
    std::string current_user;
    if (const char *system_user = std::getenv("USER")) {
        current_user = std::string(system_user);
    }
    return "/Users/" + current_user + "/.config/artchive_directory_path.txt.bak";
}

std::string git_status_path() {
    std::string current_user;
    if (const char *system_user = std::getenv("USER")) {
        current_user = std::string(system_user);
    }
    return "/Users/" + current_user + "/.config/git_status.txt";
}

void chown_chmod(const fs::path &file_path) {
    uid_t current_uid = getuid();
    gid_t current_gid = getgid();

    if (chown(file_path.c_str(), current_uid, current_gid) != 0) {
        qWarning() << "Failed to change ownership: " << strerror(errno);
        return;
    }

    if (chmod(file_path.c_str(), S_IRUSR | S_IWUSR) != 0) {
        qWarning() << "Failed to change permissions: " << strerror(errno);
        return;
    }
}

QString init_directory_path() {
    const std::string dir_file_path = sys_file_path();
    chown_chmod(dir_file_path);
    std::ifstream file(dir_file_path);
    if (!file.is_open()) {
        qWarning() << "Cannot open directory_path.txt: ";
    }

    std::string file_path;
    std::string line;
    while (std::getline(file, line)) {
        file_path = line;
    }

    file.close();
    return QString::fromStdString(file_path);
}

QString DesignFiles::dir_path() const { return m_dir_path; }

// TODO: system error handling
void DesignFiles::set_dir_path(const QString &dir_path) {
    if (m_dir_path != dir_path) {
        m_dir_path = dir_path;

        std::string move_file_path = sys_file_path();
        std::string mv_cmd = "mv " + move_file_path + " " + move_file_path + ".bak";
        chown_chmod(move_file_path);
        int move_res = std::system(mv_cmd.c_str());
        if (move_res != 0) {
            qWarning() << "Failed to move move_file_path: ";
        }

        std::string redirect_file_path = sys_file_path();
        std::string redirect_cmd =
            "echo \"" + dir_path.toStdString() + "\" > " + redirect_file_path;
        int redirect_res = std::system(redirect_cmd.c_str());
        if (redirect_res != 0) {
            qWarning() << "Failed to redirect redirect_file_path: ";
        }

        std::string remove_file_path = bak_file_path();
        chown_chmod(remove_file_path);
        std::string remove_cmd = "rm " + remove_file_path;
        int remove_res = std::system(remove_cmd.c_str());
        if (remove_res != 0) {
            qWarning() << "Failed to remove bak_file_path: ";
        }

        emit dir_path_changed();
        async_design_assets();
    }
}

// TODO: impl more comprehensive statuses
std::string getColorForStatus(const std::string &status) {
     if (status == "??")
         return "magenta";
     if (status == "M ")
         return "yellow";
     if (status == "A ")
         return "green";
     if (status == "D ")
         return "red";
     if (status == "R ")
         return "blue";
     if (status == "C ")
         return "cyan";
     if (status == "U ")
         return "magenta";
     return "black";
}

std::mutex mtx;

void DesignFiles::design_assets() {
    std::vector<std::string> local_files;
    const fs::path path{m_dir_path.toStdString()};

    if (fs::exists(path) && fs::is_directory(path)) {
        for (const std::filesystem::directory_entry &file : fs::directory_iterator(path)) {
            std::string file_name = file.path().filename().string();
            local_files.push_back(file_name);
        }
    } else {
        qWarning() << "Path does not exist or is not a directory: ";
        return;
    }

    qDebug() << "PRINT LOCAL FILES::";
    for (int i = 0; i > local_files.size(); i++) {
        qDebug() << "local_files: " << local_files[i];
    }

    std::string gs_file_path = git_status_path();
    std::string git_status_cmd = "cd " + m_dir_path.toStdString() + " && git status --porcelain > " + gs_file_path;
    int git_status_res = std::system(git_status_cmd.c_str());
    if (git_status_res != 0) {
        qWarning() << "Failed to redirect git_status_path: ";
    }

    chown_chmod(gs_file_path);
    std::ifstream file(gs_file_path);
    if (!file) {
        qWarning() << "Failed to open git status path: ";
    }

    std::vector<std::tuple<std::string, std::string>> git_status;
    std::string line;
    while (std::getline(file, line)) {
        if (line.size() < 3) continue;
        std::string status = line.substr(0, 2);
        std::string fileName = line.substr(3);

        std::string color = getColorForStatus(status);

        // qDebug() << "get_line filename: " << fileName;
        // qDebug() << "get_line color: " << color;

        // TODO: research emplace
        git_status.emplace_back(fileName, color);
    }

    file.close();
    // TODO: remove git_status file
    // TODO: update colors so strings are passed, not true qcolor
    std::lock_guard<std::mutex> lock(mtx);
    assets.resize(local_files.size());
    for (size_t i = 0; i < local_files.size(); i++) {
        for (const auto& status : git_status) {
            if (local_files[i] == std::get<0>(status)) {
                //qDebug() << "stat local_file: " << std::get<0>(status);
                //qDebug() << "stat status: " << std::get<1>(status);
                assets[i] = std::make_tuple(std::get<0>(status), std::get<1>(status));
            } else {
                // qDebug() << "nostat local_file: " << local_files[i];
                // qDebug() << "nostat status: " << "black";
                assets[i] = std::make_tuple(local_files[i], "black");
            }
        }
    }
    emit items_changed();
}

std::future<void> DesignFiles::async_design_assets() {
    return std::async(std::launch::async, &DesignFiles::design_assets, this);
}

// TODO: sort asset_map
// std::sort(all_files.begin(), all_files.end(), [](const QString &a, const QString &b) { return a < b; });

QStringList DesignFiles::items() {
    auto future = async_design_assets();
    future.wait();

    std::lock_guard<std::mutex> lock(mtx);
    for (auto& asset : assets) {
        QString file_name = QString::fromStdString(std::get<0>(asset));
        m_items.append(file_name);
    }
    return m_items;
}

QVector<QColor> DesignFiles::colors() {
    {
        std::lock_guard<std::mutex> lock(mtx);
        m_colors.clear();
        for (const auto& asset : assets) {
            QString color_name = QString::fromStdString(std::get<1>(asset));
            m_colors.append(QColor(color_name));
        }
    }
    return m_colors;
}

DesignFiles::DesignFiles(QObject *parent) : QObject{parent} {
    m_dir_path = init_directory_path();
    async_design_assets();
}
