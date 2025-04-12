mod capture;
use capture::capture_pane::CapturePane;
use capture::capture_sidebar::CaptureSidebar;

use iced::widget::{column as col, container, row, text, text_editor, text_input};
use iced::{Color, Element, Font, Length, Task};

pub fn main() -> iced::Result {
    iced::application("Yoink Desktop", Yoink::update, Yoink::view)
        .default_font(Font::MONOSPACE)
        .run()
}

#[allow(dead_code)]
struct Yoink {
    capture_pane: CapturePane,
    capture_sidebar: CaptureSidebar,
    capture_sidebar_search_content: String,
    capture_form_content: text_editor::Content,
}

impl Default for Yoink {
    fn default() -> Yoink {
        Yoink {
            capture_pane: CapturePane::new(),
            capture_sidebar: CaptureSidebar::new(),
            capture_sidebar_search_content: String::new(),
            capture_form_content: text_editor::Content::new(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    InputChanged(String),
    CaptureFormContentChanged(text_editor::Action),
}

impl Yoink {
    fn view(&self) -> Element<Message> {
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
        .style(|_theme| container::Style {
            text_color: Some(iced::Color::from_rgb8(255, 224, 181)),
            background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        });
        let capture_pane = container(
            col![
                text("Capture"),
                text_input("Topic..", &self.capture_sidebar_search_content)
                    .on_input(Message::InputChanged),
                text_input("Subject..", &self.capture_sidebar_search_content)
                    .on_input(Message::InputChanged),
                text_editor(&self.capture_form_content)
                    .on_action(Message::CaptureFormContentChanged)
            ]
            .spacing(10),
        )
        .width(Length::FillPortion(6))
        .height(Length::Fill)
        .padding(5)
        .style(|_theme| container::Style {
            text_color: Some(iced::Color::from_rgb8(15, 9, 9)),
            background: Some(iced::Background::Color(Color::from_rgb8(255, 224, 181))),
            border: iced::Border::default(),
            shadow: iced::Shadow::default(),
        });

        let ui = row![capture_sidebar, capture_pane];

        container(ui)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(value) => {
                println!("{}", value);
                self.capture_sidebar_search_content = value;

                Task::none()
            }
            Message::CaptureFormContentChanged(action) => {
                self.capture_form_content.perform(action);

                let editor_text = self.capture_form_content.text();
                println!("{}", editor_text);

                Task::none()
            }
        }
    }
}
