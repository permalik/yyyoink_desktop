mod capture;
use capture::pane::{Pane, PaneStyle};
use capture::sidebar::{Sidebar, SidebarStyle};
use iced::widget::{container, row, text};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

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
        String::from("Yoink Desktop")
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
