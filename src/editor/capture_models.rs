use iced::widget::text;

pub struct Editor {
    pub is_saved: bool,
}

impl Editor {
    pub fn new() -> Self {
        Self { is_saved: true }
    }
}
