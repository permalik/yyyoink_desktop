#ifndef DESIGNFILES_H
#define DESIGNFILES_H

#include <future>
#include <QColor>
#include <QObject>
#include <QVector>

// TODO: Refactor to snake_case
class DesignFiles : public QObject {
    Q_OBJECT
    Q_PROPERTY(QString dir_path READ dir_path WRITE set_dir_path NOTIFY dir_path_changed)
    Q_PROPERTY(QStringList items READ items NOTIFY items_changed)
    Q_PROPERTY(QVector<QColor> colors READ colors NOTIFY items_changed)

public:
    explicit DesignFiles(QObject *parent = nullptr);

    QString dir_path() const;

    void design_assets();

    std::future<void> async_design_assets();

    QStringList items();

    QVector<QColor> colors();

public slots:
    void set_dir_path(const QString &dir_path);

signals:
    void dir_path_changed();
    void items_changed();

private:
    QString m_dir_path;
    std::vector<std::tuple<std::string, std::string>> assets;
    QStringList m_items;
    QVector<QColor> m_colors;
};

#endif // DESIGNFILES_H
