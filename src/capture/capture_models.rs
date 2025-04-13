use iced::widget::text_editor;

pub struct Capture {
    pub search: String,
    pub form_topic: String,
    pub form_subject: String,
    pub form_content: text_editor::Content,
    pub updated_file: Option<String>,
}

impl Capture {
    pub fn new() -> Self {
        Self {
            search: String::new(),
            form_topic: String::new(),
            form_subject: String::new(),
            form_content: text_editor::Content::new(),
            updated_file: None,
        }
    }
}
