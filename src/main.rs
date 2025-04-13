mod capture;
use capture::capture_pane::CapturePane;
use capture::capture_sidebar::CaptureSidebar;
use iced::widget::{button, column as col, container, row, text, text_editor, text_input};
use iced::{Color, Element, Font, Length, Task};
use std::io::ErrorKind;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

pub fn main() -> iced::Result {
    iced::application("Yoink Desktop", Yoink::update, Yoink::view)
        .default_font(Font::MONOSPACE)
        .run()
}

#[derive(Debug, Clone)]
pub enum Error {
    PermissionDenied,
    FileNotFound,
    IoError(ErrorKind),
}

struct Capture {
    search: String,
    form_topic: String,
    form_subject: String,
    form_content: text_editor::Content,
    updated_file: Option<String>,
}

impl Capture {
    fn new() -> Self {
        Self {
            search: String::new(),
            form_topic: String::new(),
            form_subject: String::new(),
            form_content: text_editor::Content::new(),
            updated_file: None,
        }
    }
}

struct Yoink {
    capture: Capture,
    capture_pane: CapturePane,
    capture_sidebar: CaptureSidebar,
}

impl Default for Yoink {
    fn default() -> Yoink {
        Yoink {
            capture: Capture::new(),
            capture_pane: CapturePane::new(),
            capture_sidebar: CaptureSidebar::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    CaptureSearchChanged(String),
    CaptureTopicChanged(String),
    CaptureSubjectChanged(String),
    CaptureFormContentChanged(text_editor::Action),
    SubmitCapture,
    FileOpened(Result<PathBuf, Error>),
}

impl Yoink {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::CaptureSearchChanged(value) => {
                self.capture.search = value;

                Task::none()
            }
            Message::CaptureTopicChanged(value) => {
                self.capture.form_topic = value;

                Task::none()
            }
            Message::CaptureSubjectChanged(value) => {
                self.capture.form_subject = value;

                Task::none()
            }
            Message::CaptureFormContentChanged(action) => {
                self.capture.form_content.perform(action);

                Task::none()
            }
            Message::SubmitCapture => {
                println!("Search: {}", self.capture.search);
                println!("Topic: {}", self.capture.form_topic);
                println!("Subject: {}", self.capture.form_subject);
                let form_content = self.capture.form_content.text();
                println!("{}", form_content);

                Task::perform(write_file(), Message::FileOpened)
            }
            Message::FileOpened(result) => {
                if let Ok(path) = result {
                    self.capture.updated_file = Some(path.to_string_lossy().to_string());
                    println!("Opened/Written to {}", path.display());
                }

                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let capture_sidebar = if self.capture_sidebar.is_visible {
            container(
                col![
                    text_input("Capture..", &self.capture.search)
                        .on_input(Message::CaptureSearchChanged),
                    text("Capture001.."),
                    text("Capture002.."),
                    text("Capture003.."),
                ]
                .spacing(10),
            )
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(5)
            .style(|_theme| container::Style {
                text_color: Some(iced::Color::from_rgb8(255, 224, 181)),
                background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
        } else {
            container(text("Sidebar hidden."))
                .width(Length::FillPortion(2))
                .height(Length::Fill)
                .padding(5)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                    text_color: Some(iced::Color::from_rgb8(255, 224, 181)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                })
        };
        let capture_pane = if self.capture_pane.is_visible {
            container(
                col![
                    text("Capture"),
                    text_input("Topic..", &self.capture.form_topic)
                        .on_input(Message::CaptureTopicChanged),
                    text_input("Subject..", &self.capture.form_subject)
                        .on_input(Message::CaptureSubjectChanged),
                    text_editor(&self.capture.form_content)
                        .on_action(Message::CaptureFormContentChanged),
                    button("Submit")
                        .on_press(Message::SubmitCapture)
                        .style(|_theme, _status| button::Style {
                            background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                            text_color: iced::Color::from_rgb8(255, 224, 181),
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        }),
                ]
                .spacing(10),
            )
            .width(Length::FillPortion(6))
            .height(Length::Fill)
            .padding(5)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(255, 224, 181))),
                text_color: Some(iced::Color::from_rgb8(15, 9, 9)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
        } else {
            container(text("Pane hidden."))
                .width(Length::FillPortion(6))
                .height(Length::Fill)
                .padding(5)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(255, 224, 181))),
                    text_color: Some(iced::Color::from_rgb8(15, 9, 9)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                })
        };

        let ui = row![capture_sidebar, capture_pane];

        container(ui)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

async fn write_file() -> Result<PathBuf, Error> {
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
        match tokio::fs::write(&path, b"UPDATED").await {
            Ok(_) => {
                println!("Wrote to file {}.", path.display());
                Ok(path)
            }
            Err(e) => match e.kind() {
                ErrorKind::PermissionDenied => {
                    eprintln!("Permission denied: Failed to write file.");
                    Err(Error::PermissionDenied)
                }
                ErrorKind::NotFound => {
                    eprintln!("File Not Found: Failed to write file {}.", path.display());
                    Err(Error::FileNotFound)
                }
                kind => {
                    eprintln!("Failed to write file: {}", e);
                    Err(Error::IoError(kind))
                }
            },
        }
    } else {
        let file = tokio::fs::OpenOptions::new()
            .append(true)
            .open(&path)
            .await
            .map_err(|e| {
                eprintln!("Failed: Cannot open file for append. {}", e);
                match e.kind() {
                    ErrorKind::PermissionDenied => Error::PermissionDenied,
                    ErrorKind::NotFound => Error::FileNotFound,
                    kind => Error::IoError(kind),
                }
            })?;

        let mut writer = tokio::io::BufWriter::new(file);
        let _ = writer.write_all(b"NEWNEW").await.map_err(|e| {
            eprintln!("Failed: Cannot append to file. {}", e);
            match e.kind() {
                ErrorKind::PermissionDenied => Error::PermissionDenied,
                ErrorKind::NotFound => Error::FileNotFound,
                kind => Error::IoError(kind),
            }
        });

        writer.flush().await.map_err(|e| {
            eprintln!("Failed: Cannot flush writer. {}", e);
            match e.kind() {
                ErrorKind::PermissionDenied => Error::PermissionDenied,
                ErrorKind::NotFound => Error::FileNotFound,
                kind => Error::IoError(kind),
            }
        })?;

        Ok(path)
    }
}
