use crate::enums::error::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

const TEST_PATH: &str = "/Users/tymalik/Docs/Git/markdown/_test.md";

pub async fn load_captures() -> Result<String, Error> {
    match read_file().await {
        Ok(_) => {
            println!("Success: vec_result");
        }
        Err(e) => {
            eprintln!("Failure: vec_result");
            return Err(e);
        }
    }

    Ok("loaded_capture001".to_string())
}

async fn read_file() -> Result<Vec<String>, Error> {
    // TODO:Remove this paranoid file check.
    // Attempt the read and handle error if it occurs due to non-existant file.
    let (is_file, path) = file_exists(TEST_PATH).await;

    if is_file {
        let bytes = tokio::fs::read(&path)
            .await
            .map_err(|e| Error::IoError(e.kind()))?;
        if let Ok(string) = String::from_utf8(bytes.clone()) {
            // TODO: Parse this file output
            println!("{}", string);
            Ok(vec!["return contents".to_string()])
        } else {
            Err(Error::IoError(ErrorKind::InvalidData))
        }
    } else {
        eprintln!("Failed to read file. File does not exist.");
        Err(Error::FileNotFound)
    }
}

pub async fn write_file(capture_string: String) -> Result<PathBuf, Error> {
    let (is_file, path) = file_exists(TEST_PATH).await;

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
