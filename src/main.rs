use iced::widget::{container, row, text};
use iced::{color, executor, Color, Length};
use iced::{Application, Command, Element, Settings, Theme};

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Pane;

impl Pane {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Default)]
pub struct PaneStyle;

impl container::StyleSheet for PaneStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            background: Some(iced::Background::Color(color!(0xF5EBD8))),
            text_color: Some(Color::BLACK),
            ..Default::default()
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Editor {
    sidebar: Sidebar,
    pane: Pane,
}

impl Default for Editor {
    fn default() -> Editor {
        let sidebar = Sidebar::new();
        let pane = Pane::new();
        Editor {
            sidebar: sidebar,
            pane: pane,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Application for Editor {
    type Executor = executor::Default;
    type Flags = ();
    type Message = Message;
    type Theme = Theme;

    fn new(_flags: ()) -> (Editor, Command<Self::Message>) {
        (Editor::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Custom Editor")
    }

    fn update(&mut self, _message: Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let sidebar = container(text("Sidebar."))
            .width(Length::FillPortion(2))
            .height(Length::Fill)
            .padding(5)
            .style(iced::theme::Container::Custom(Box::new(SidebarStyle)));
        let pane = container(text("Editor Pane."))
            .width(Length::FillPortion(6))
            .height(Length::Fill)
            .padding(5)
            .style(iced::theme::Container::Custom(Box::new(PaneStyle)));

        let ui = row![sidebar, pane];

        container(ui)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_y()
            .into()
    }
}

pub fn main() -> iced::Result {
    Editor::run(Settings::default())
}
