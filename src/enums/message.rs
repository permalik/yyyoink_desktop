use super::error;
use iced::event::Event;
use iced::widget::{pane_grid, text_editor};
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Message {
    CapturesLoaded(Result<Vec<Vec<String>>, error::Error>),
    FilesLoaded(Result<Vec<String>, error::Error>),
    CapturesReloaded(Result<Vec<Vec<String>>, error::Error>),
    Ignore,
    NewFileInput(String),
    RadiusChanged(f32),
    CaptureSearchChanged(String),
    FileSearchChanged(String),
    CaptureTopicChanged(String),
    CaptureSubjectChanged(String),
    CaptureFormContentChanged(text_editor::Action),
    CaptureSelected(usize),
    CaptureOpened(Result<(String, PathBuf, String), error::Error>),
    FileSelected(String),
    FileWritten(Result<PathBuf, error::Error>),
    EditorFileOpened(Result<Vec<String>, error::Error>),
    UpdateCapture,
    SubselectCapture,
    UpdateFile,
    CreateFile,
    ViewModalHelper,
    CreateNewFile,
    CreatingNewFile(Result<bool, error::Error>),
    DeleteFile(String),
    FileDeleted(Result<bool, error::Error>),
    SubmitCapture,
    FileOpened(Result<PathBuf, error::Error>),
    ShowError(Result<String, error::Error>),
    ShowHelper(Result<String, error::Error>),
    ShowModalHelper(Result<String, error::Error>),
    HideHelper,
    HideModalHelper,
    DeleteCapture(usize),
    CaptureDeleted(Result<bool, error::Error>),
    Event(Event),
    PaneResized(pane_grid::ResizeEvent),
    Edit,
    EditorContentChanged(text_editor::Action),
    SetInitialEditorText(
        Result<
            (
                Vec<(String, Vec<String>)>,
                (String, Vec<String>),
                Vec<(String, Vec<String>)>,
            ),
            error::Error,
        >,
    ),
}
