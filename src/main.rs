mod capture;
use capture::capture_pane::{CapturePane, CapturePaneStyle};
use capture::capture_sidebar::{CaptureSidebar, CaptureSidebarStyle};
use iced::widget::{column as col, container, row, text, text_input};
use iced::{executor, Application, Command, Element, Length, Settings, Theme};

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Editor {
    capture_pane: CapturePane,
    capture_sidebar: CaptureSidebar,
    capture_sidebar_search_content: String,
}

impl Default for Editor {
    fn default() -> Editor {
        let capture_sidebar = CaptureSidebar::new();
        let capture_pane = CapturePane::new();
        Editor {
            capture_pane: capture_pane,
            capture_sidebar: capture_sidebar,
            capture_sidebar_search_content: String::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
}

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

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::InputChanged(value) => {
                println!("{}", value);
                self.capture_sidebar_search_content = value;
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let capture_sidebar = container(
            col![
                text_input("Capture..", &self.capture_sidebar_search_content)
                    .on_input(Message::InputChanged),
                text("Capture001.."),
                text("Capture002.."),
                text("Capture003.."),
            ]
            .spacing(10),
        )
        .width(Length::FillPortion(2))
        .height(Length::Fill)
        .padding(5)
        .style(iced::theme::Container::Custom(Box::new(
            CaptureSidebarStyle,
        )));
        let capture_pane = container(text("Editor Pane."))
            .width(Length::FillPortion(6))
            .height(Length::Fill)
            .padding(5)
            .style(iced::theme::Container::Custom(Box::new(CapturePaneStyle)));

        let ui = row![capture_sidebar, capture_pane];

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
