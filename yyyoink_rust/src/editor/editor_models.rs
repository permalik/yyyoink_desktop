use iced::widget::text_editor;

pub struct Editor {
    pub editor_content: text_editor::Content,
    pub is_saved: bool,
    pub new_file: String,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            editor_content: text_editor::Content::new(),
            is_saved: true,
            new_file: String::new(),
        }
    }
}
