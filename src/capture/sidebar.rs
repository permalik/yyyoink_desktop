use iced::{color, widget::container, Color};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sidebar;

impl Sidebar {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Default)]
pub struct SidebarStyle;

impl container::StyleSheet for SidebarStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(color!(0x000000))),
            text_color: Some(Color::WHITE),
            ..Default::default()
        }
    }
}
