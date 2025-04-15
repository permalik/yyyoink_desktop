use crate::enums::error::Error;
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

pub async fn load_captures() -> Result<String, Error> {
    let value = "loaded_capture001".to_string();
    Ok(value)
}

pub async fn write_file(capture_string: String) -> Result<PathBuf, Error> {
    let path = PathBuf::from("/Users/tymalik/Docs/Git/markdown/_test.md");
    let check_path = path.clone();

    let file_exists = tokio::task::spawn_blocking(move || std::fs::metadata(&check_path).is_ok())
        .await
        .map_err(|e| {
            eprintln!("Failed: Unable to determine if file exists. {}", e);
            e
        })
        .expect("Failed: Unable to determine if file exists.");

    if !file_exists {
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
