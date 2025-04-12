use iced::{color, widget::container, Color};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapturePane;

impl CapturePane {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Default)]
pub struct CapturePaneStyle;

impl container::StyleSheet for CapturePaneStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(color!(0xF5EBD8))),
            text_color: Some(Color::BLACK),
            ..Default::default()
        }
    }
}
