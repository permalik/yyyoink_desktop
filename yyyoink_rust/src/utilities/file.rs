use super::tool;
use crate::enums::error::Error;
use crate::enums::message::Message;
use iced::keyboard;
use std::ffi::OsString;
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::fs;
use tokio::io::AsyncWriteExt;

pub async fn log() -> Result<String, Error> {
    let success = true;

    if success {
        println!("hello");
        Ok("hello".to_string())
    } else {
        Err(Error::IoError(ErrorKind::InvalidData))
    }
}

pub async fn load_captures() -> Result<Vec<Vec<String>>, Error> {
    let mut files: Vec<String> = Vec::new();
    match get_files().await {
        Ok(file_names) => {
            for file in file_names {
                files.push(file);
            }
        }
        Err(e) => {
            eprintln!("Failed to get file_name.\nUnderlying error: {}", e);
        }
    }

    let mut captures: Option<Vec<Vec<String>>> = None;
    for file in files {
        let file_name: &str = file.as_ref();

        match read_file(file_name).await {
            Ok(lines) => {
                for line in &lines {
                    // TODO: Check for various-sized initial input strings
                    if line.len() > 16 && &line[..4] == "<!--" && &line[line.len() - 3..] == "-->" {
                        let chars: Vec<char> = line.chars().collect();
                        let unwrapped_line: String = chars[4..chars.len() - 3].iter().collect();
                        let parts: Vec<String> = unwrapped_line
                            .split("::::")
                            .map(|s| s.to_string())
                            .collect();

                        captures
                            .get_or_insert(vec![])
                            .push(parts.into_iter().skip(1).collect());
                    }
                }
            }
            Err(e) => {
                eprintln!("Failure: vec_result");
                return Err(e);
            }
        }
    }
    if let Some(captures) = captures {
        Ok(captures)
    } else {
        Err(Error::IoError(ErrorKind::InvalidData))
    }
}

async fn get_files() -> Result<Vec<String>, Error> {
    let source_dir = tool::source_dir();
    let source_dir_ref: &str = &source_dir;
    let mut entries = tokio::fs::read_dir(source_dir_ref)
        .await
        .map_err(|e| Error::IoError(e.kind()))?;

    let mut file_names: Vec<String> = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| Error::IoError(e.kind()))?
    {
        let name: OsString = entry.file_name();
        let name_str = name.to_string_lossy().to_string();

        if name_str.starts_with("_") && name_str.ends_with(".md") {
            file_names.push(name_str);
        }
    }

    if file_names.is_empty() {
        return Err(Error::FileNotFound);
    }

    Ok(file_names)
}

pub async fn load_files() -> Result<Vec<String>, Error> {
    let source_dir = tool::source_dir();
    let source_dir_ref: &str = &source_dir;
    let mut entries = tokio::fs::read_dir(source_dir_ref)
        .await
        .map_err(|e| Error::IoError(e.kind()))?;

    let mut file_names: Vec<String> = Vec::new();

    while let Some(entry) = entries
        .next_entry()
        .await
        .map_err(|e| Error::IoError(e.kind()))?
    {
        let path = entry.path();
        match fs::metadata(&path).await {
            Ok(meta) => {
                if !meta.is_dir() {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        file_names.push(file_name.to_string());
                    }
                }
            }
            Err(e) => {
                eprintln!("Unable to get file metadata: {}", e);
            }
        }
    }

    if file_names.is_empty() {
        return Err(Error::FileNotFound);
    }

    Ok(file_names)
}

async fn read_file(file_name: &str) -> Result<Vec<String>, Error> {
    // TODO:Remove this paranoid file check.
    // Attempt the read and handle error if it occurs due to non-existant file.
    let capture_path = tool::source_path(file_name.to_string());
    let capture_path_ref: &str = &capture_path;
    let (is_file, path) = file_exists(capture_path_ref).await;

    if is_file {
        let bytes = tokio::fs::read(&path)
            .await
            .map_err(|e| Error::IoError(e.kind()))?;
        if let Ok(string) = String::from_utf8(bytes.clone()) {
            let lines: Vec<String> = string.lines().map(|s| s.to_string()).collect();
            Ok(lines)
        } else {
            Err(Error::IoError(ErrorKind::InvalidData))
        }
    } else {
        eprintln!("Failed to read file. File does not exist.");
        Err(Error::FileNotFound)
    }
}

pub async fn append_file(capture_file: String, capture_string: String) -> Result<PathBuf, Error> {
    let capture_path = tool::source_path(capture_file);
    let capture_path_ref: &str = &capture_path;
    let (is_file, path) = file_exists(capture_path_ref).await;

    if !is_file {
        let capture_bytes: &[u8] = capture_string.as_bytes();
        match tokio::fs::write(&path, capture_bytes).await {
            Ok(_) => {
                println!("Wrote to file {}.", path.display());
                Ok(path)
            }
            Err(e) => {
                let err = match e.kind() {
                    ErrorKind::PermissionDenied => Error::PermissionDenied,
                    ErrorKind::NotFound => Error::FileNotFound,
                    kind => Error::IoError(kind),
                };
                eprintln!(
                    "Failed to write file {}: {}\nUnderlying error: {}",
                    path.display(),
                    err,
                    e
                );
                Err(err)
            }
        }
    } else {
        let appended_capture_string = format!("{}{}", "\n", capture_string);
        let capture_bytes: &[u8] = appended_capture_string.as_bytes();
        let file = tokio::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .await
            .map_err(|e| {
                let err = match e.kind() {
                    ErrorKind::PermissionDenied => Error::PermissionDenied,
                    ErrorKind::NotFound => Error::FileNotFound,
                    kind => Error::IoError(kind),
                };
                eprintln!(
                    "Failed to open file for append {}: {}\nUnderlying error: {}",
                    path.display(),
                    err,
                    e
                );
                err
            })?;

        let mut writer = tokio::io::BufWriter::new(file);
        let _ = writer.write_all(capture_bytes).await.map_err(|e| {
            let err = match e.kind() {
                ErrorKind::PermissionDenied => Error::PermissionDenied,
                ErrorKind::NotFound => Error::FileNotFound,
                kind => Error::IoError(kind),
            };
            eprintln!(
                "Failed to append to file {}: {}\nUnderlying error: {}",
                path.display(),
                err,
                e
            );
            err
        });

        writer.flush().await.map_err(|e| {
            let err = match e.kind() {
                ErrorKind::PermissionDenied => Error::PermissionDenied,
                ErrorKind::NotFound => Error::FileNotFound,
                kind => Error::IoError(kind),
            };
            eprintln!(
                "Failed to flush writer after writing to file {}: {}\nUnderlying error: {}",
                path.display(),
                err,
                e
            );
            err
        })?;

        Ok(path)
    }
}

pub async fn write_file(capture_file: String, capture_string: String) -> Result<PathBuf, Error> {
    let capture_path = tool::source_path(capture_file);
    let capture_path_ref: &str = &capture_path;
    let (_is_file, path) = file_exists(capture_path_ref).await;

    let capture_bytes: &[u8] = capture_string.as_bytes();
    match tokio::fs::write(&path, capture_bytes).await {
        Ok(_) => {
            println!("Wrote to file {}.", path.display());
            Ok(path)
        }
        Err(e) => {
            let err = match e.kind() {
                ErrorKind::PermissionDenied => Error::PermissionDenied,
                ErrorKind::NotFound => Error::FileNotFound,
                kind => Error::IoError(kind),
            };
            eprintln!(
                "Failed to write file {}: {}\nUnderlying error: {}",
                path.display(),
                err,
                e
            );
            Err(err)
        }
    }
}

pub async fn read_capture(
    timestamp: &str,
    file_name: &str,
    subject: &str,
) -> Result<
    (
        Vec<(String, Vec<String>)>,
        (String, Vec<String>),
        Vec<(String, Vec<String>)>,
    ),
    Error,
> {
    let capture_path = tool::source_path(file_name.to_string());
    println!("{}", capture_path);
    let capture_path_ref: &str = &capture_path;
    let (is_file, path) = file_exists(capture_path_ref).await;

    if is_file {
        println!("{}{}", timestamp, subject);
        let mut file_content: Vec<String> = Vec::new();
        let bytes = tokio::fs::read(&path)
            .await
            .map_err(|e| Error::IoError(e.kind()))?;
        if let Ok(string) = String::from_utf8(bytes.clone()) {
            file_content = string.lines().map(|s| s.to_string()).collect();
        } else {
            println!("Unable to convert bytes to string.");
        }

        let prefix = "<!--yoink";
        let delimiter = "::::";
        let topic = &file_name[1..file_name.len() - 3];
        let suffix = "-->";
        let capture_string = format!(
            "{}{}{}{}{}{}{}{}",
            prefix, delimiter, timestamp, delimiter, topic, delimiter, subject, suffix
        );
        let mut before: Vec<(String, Vec<String>)> = Vec::new();
        let mut content: (String, Vec<String>) = (String::new(), Vec::new());
        let mut after: Vec<(String, Vec<String>)> = Vec::new();
        let mut sections: Vec<(String, Vec<String>)> = Vec::new();
        let mut current_section: Option<(String, Vec<String>)> = None;
        for line in file_content.iter() {
            if line.starts_with("<!--yoink") && line.ends_with("-->") {
                if let Some((header, lines)) = current_section.take() {
                    sections.push((header, lines));
                }

                current_section = Some((line.to_string(), Vec::new()));
            } else if let Some((_, ref mut lines)) = current_section {
                lines.push(line.to_string());
            }
        }

        if let Some((header, lines)) = current_section {
            sections.push((header, lines));
        }

        let mut found_content = false;
        for (header, lines) in sections {
            if header == capture_string {
                content.0 = header;
                content.1.extend(lines);
                found_content = true;
            } else if !found_content {
                before.push((header, lines));
            } else {
                after.push((header, lines));
            }
        }

        Ok((before, content, after))
    } else {
        eprintln!("Failed to read file. File does not exist.");
        Err(Error::FileNotFound)
    }
}

pub async fn delete_capture(capture: Vec<String>) -> Result<bool, Error> {
    match (capture.get(0), capture.get(1), capture.get(2)) {
        (Some(timestamp), Some(file_path), Some(subject)) => {
            let file_name = format!("_{}.md", file_path);
            let capture_path = tool::source_path(file_name.to_string());
            let capture_path_ref: &str = &capture_path;
            let (is_file, path) = file_exists(capture_path_ref).await;

            if is_file {
                let mut file_content: Vec<String> = Vec::new();
                let bytes = tokio::fs::read(&path)
                    .await
                    .map_err(|e| Error::IoError(e.kind()))?;
                if let Ok(string) = String::from_utf8(bytes.clone()) {
                    file_content = string.lines().map(|s| s.to_string()).collect();
                } else {
                    println!("Unable to convert bytes to string.");
                }

                let prefix = "<!--yoink";
                let delimiter = "::::";
                let suffix = "-->";
                let capture_string = format!(
                    "{}{}{}{}{}{}{}{}",
                    prefix, delimiter, timestamp, delimiter, file_path, delimiter, subject, suffix
                );
                let mut before: Vec<(String, Vec<String>)> = Vec::new();
                let mut content: (String, Vec<String>) = (String::new(), Vec::new());
                let mut after: Vec<(String, Vec<String>)> = Vec::new();
                let mut sections: Vec<(String, Vec<String>)> = Vec::new();
                let mut current_section: Option<(String, Vec<String>)> = None;

                for line in file_content.iter() {
                    if line.starts_with("<!--yoink") && line.ends_with("-->") {
                        if let Some((header, lines)) = current_section.take() {
                            sections.push((header, lines));
                        }

                        current_section = Some((line.to_string(), Vec::new()));
                    } else if let Some((_, ref mut lines)) = current_section {
                        lines.push(line.to_string());
                    }
                }

                if let Some((header, lines)) = current_section {
                    sections.push((header, lines));
                }

                let mut found_content = false;
                for (header, lines) in sections {
                    if header == capture_string {
                        content.0 = header;
                        content.1.extend(lines);
                        found_content = true;
                    } else if !found_content {
                        before.push((header, lines));
                    } else {
                        after.push((header, lines));
                    }
                }

                let mut before_string = "".to_string();
                let mut after_string = "".to_string();
                let mut content_string = "".to_string();
                for (header, lines) in before {
                    before_string.push_str(&format!("{}\n", header.trim()));
                    for line in lines {
                        before_string.push_str(&format!("{}\n", line.trim()));
                    }
                }
                for (header, lines) in after {
                    after_string.push_str(&format!("{}\n", header.trim()));
                    for line in lines {
                        after_string.push_str(&format!("{}\n", line.trim()));
                    }
                }
                content_string.push_str(&format!("{}\n", content.0.trim()));
                for line in content.1 {
                    content_string.push_str(&format!("{}\n", line.trim()));
                }

                let update_content = format!("{}{}", before_string, after_string);

                match write_file(file_name, update_content).await {
                    Ok(path_buf) => {
                        println!("Updated: {}", path_buf.to_string_lossy());
                    }
                    Err(e) => {
                        eprintln!("Failed to update file: {}", e);
                    }
                }

                Ok(true)
            } else {
                eprintln!("Failed to read file. File does not exist.");
                Err(Error::FileNotFound)
            }
        }
        _ => return Err(Error::IoError(ErrorKind::Other)),
    }
}

async fn file_exists(current_path: &str) -> (bool, PathBuf) {
    let path = PathBuf::from(current_path);
    let check_path = path.clone();

    let file_exists = tokio::task::spawn_blocking(move || std::fs::metadata(&check_path).is_ok())
        .await
        .map_err(|e| {
            eprintln!("failed: unable to determine if file exists. {}", e);
            e
        })
        .expect("Failed: Unable to determine if file exists.");

    return (file_exists, path);
}

pub async fn capture_opened(capture: Vec<String>) -> Result<(String, PathBuf, String), Error> {
    match (capture.get(0), capture.get(1), capture.get(2)) {
        (Some(timestamp), Some(path), Some(subject)) => {
            let capture_path = format!("_{}.md", path);
            Ok((
                timestamp.to_string(),
                PathBuf::from(capture_path),
                subject.to_string(),
            ))
        }
        _ => return Err(Error::IoError(ErrorKind::NotFound)),
    }
}

pub async fn file_opened(file: String) -> Result<Vec<String>, Error> {
    match read_file(&file).await {
        Ok(contents) => Ok(contents),
        Err(e) => {
            eprintln!("Failure: file_opened");
            return Err(e);
        }
    }
}

pub async fn create_file(file: String) -> Result<bool, Error> {
    let capture_file = format!("_{}.md", file);
    let capture_path = tool::source_path(capture_file);
    // TODO: update to avoid truncating existing files
    match tokio::fs::File::create(&capture_path).await {
        Ok(_) => Ok(true),
        Err(e) => {
            eprintln!("Failure: create_file\n{}", e);
            Err(Error::IoError(ErrorKind::Other))
        }
    }
}

pub async fn delete_file(file: String) -> Result<bool, Error> {
    let capture_path = tool::source_path(file);
    match tokio::fs::remove_file(PathBuf::from(&capture_path)).await {
        Ok(_) => Ok(true),
        Err(e) => {
            eprintln!("Failure: delete_file\n{}", e);
            Err(Error::IoError(ErrorKind::Other))
        }
    }
}

pub fn handle_hotkey(key: keyboard::Key, modifiers: keyboard::Modifiers) -> Option<Message> {
    match (key.as_ref(), modifiers) {
        (keyboard::Key::Character(c), keyboard::Modifiers::ALT) if c == "e" => Some(Message::Edit),
        (keyboard::Key::Character(c), keyboard::Modifiers::ALT) if c == "s" => {
            Some(Message::UpdateCapture)
        }
        _ => None,
    }
}
