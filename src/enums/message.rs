use super::error;
use iced::widget::text_editor;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    CapturesLoaded(Result<Vec<Vec<String>>, error::Error>),
    CaptureSearchChanged(String),
    CaptureTopicChanged(String),
    CaptureSubjectChanged(String),
    CaptureFormContentChanged(text_editor::Action),
    SubmitCapture,
    FileOpened(Result<PathBuf, error::Error>),
}
