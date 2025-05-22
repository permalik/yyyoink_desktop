use std::path::PathBuf;

use iced::widget::text_editor;

pub struct Capture {
    pub search: String,
    pub form_topic: String,
    pub form_subject: String,
    pub form_content: text_editor::Content,
    pub updated_file: Option<String>,
    pub opened_capture: Option<(String, PathBuf, String)>,
    pub current_capture: String,
    pub current_capture_timestamp: String,
    pub current_capture_file: String,
    pub current_capture_subject: String,
    pub before: String,
    pub after: String,
}

impl Capture {
    pub fn new() -> Self {
        Self {
            search: String::new(),
            form_topic: String::new(),
            form_subject: String::new(),
            form_content: text_editor::Content::new(),
            updated_file: None,
            opened_capture: None,
            current_capture: "Editor..".to_string(),
            current_capture_timestamp: String::new(),
            current_capture_file: String::new(),
            current_capture_subject: String::new(),
            before: String::new(),
            after: String::new(),
        }
    }
}
