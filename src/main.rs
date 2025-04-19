mod capture;
mod enums;
mod utilities;
use capture::capture_models::Capture;
use capture::capture_pane::CapturePane;
use capture::capture_sidebar::CaptureSidebar;
use chrono::prelude::*;
use enums::message::Message;
use iced::widget::{
    self, button, center, column as col, container, mouse_area, opaque, row, stack, text,
    text_editor, text_input,
};
use iced::{Color, Element, Font, Length, Task};
use utilities::file;

pub fn main() -> iced::Result {
    iced::application("Yoink Desktop", Yoink::update, Yoink::view)
        .default_font(Font::MONOSPACE)
        .run_with(Yoink::new)
}

struct Yoink {
    captures: Vec<Vec<String>>,
    capture: Capture,
    capture_pane: CapturePane,
    capture_sidebar: CaptureSidebar,
    ui_error: String,
    show_error: bool,
}

impl Yoink {
    fn new() -> (Self, Task<Message>) {
        (
            Self {
                captures: Vec::new(),
                capture: Capture::new(),
                capture_pane: CapturePane::new(),
                capture_sidebar: CaptureSidebar::new(),
                ui_error: String::new(),
                show_error: false,
            },
            Task::perform(file::load_captures(), Message::CapturesLoaded),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ShowError(result) => {
                if let Ok(value) = result {
                    println!("resulting {}", value);
                    self.show_error = true;
                }
                widget::focus_next()
            }
            Message::HideError => {
                self.hide_error();
                Task::none()
            }
            Message::CapturesLoaded(result) => {
                if let Ok(value) = result {
                    self.captures = value;
                }
                Task::none()
            }
            Message::CaptureSearchChanged(value) => {
                self.capture.search = value;
                Task::none()
            }
            Message::CaptureTopicChanged(value) => {
                self.capture.form_topic = value;
                Task::none()
            }
            Message::CaptureSubjectChanged(value) => {
                self.capture.form_subject = value;
                Task::none()
            }
            Message::CaptureFormContentChanged(action) => {
                self.capture.form_content.perform(action);
                Task::none()
            }
            Message::SubmitCapture => {
                if !self.capture.form_topic.starts_with('_') {
                    self.ui_error =
                        "Submission failed: Topic must start with underscore.".to_string();
                    Task::perform(file::log(), Message::ShowError)
                } else {
                    println!("Search: {}", self.capture.search);
                    println!("Topic: {}", self.capture.form_topic);
                    println!("Subject: {}", self.capture.form_subject);
                    let form_content = self.capture.form_content.text();
                    println!("{}", form_content);
                    let spec_prefix = "<!--yoink::::";
                    let spec_delimiter = "::::";
                    let spec_suffix = "-->\n";
                    let utc = Utc::now();
                    let local_timestamp = utc.with_timezone(&Local).to_string();
                    let local = &local_timestamp[..19].to_string();
                    let spec_string = format!(
                        "{}{}{}{}{}{}{}",
                        spec_prefix,
                        local,
                        spec_delimiter,
                        self.capture.form_topic,
                        spec_delimiter,
                        self.capture.form_subject,
                        spec_suffix
                    );
                    let content_string = format!("{}{}", self.capture.form_content.text(), "\n");
                    let capture_string = format!("{}{}", spec_string, content_string);

                    Task::perform(file::write_file(capture_string), Message::FileOpened)
                }
            }
            Message::FileOpened(result) => {
                if let Ok(path) = result {
                    self.capture.updated_file = Some(path.to_string_lossy().to_string());
                    println!("Opened/Written to {}", path.display());
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let capture_list = self
            .captures
            .iter()
            .map(|capture| {
                col(capture
                    .iter()
                    .map(|field| text(field).into())
                    .collect::<Vec<Element<Message>>>())
                .into()
            })
            .collect::<Vec<Element<Message>>>();

        let capture_sidebar = if self.capture_sidebar.is_visible {
            container(
                col![
                    text_input("Capture..", &self.capture.search)
                        .on_input(Message::CaptureSearchChanged),
                    col(capture_list).spacing(5),
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
            })
        } else {
            container(text("Sidebar hidden."))
                .width(Length::FillPortion(2))
                .height(Length::Fill)
                .padding(5)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                    text_color: Some(iced::Color::from_rgb8(255, 224, 181)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                })
        };
        let capture_pane = if self.capture_pane.is_visible {
            container(
                col![
                    text("Capture"),
                    text_input("Topic..", &self.capture.form_topic)
                        .on_input(Message::CaptureTopicChanged),
                    text_input("Subject..", &self.capture.form_subject)
                        .on_input(Message::CaptureSubjectChanged),
                    text_editor(&self.capture.form_content)
                        .on_action(Message::CaptureFormContentChanged),
                    button("Submit")
                        .on_press(Message::SubmitCapture)
                        .style(|_theme, _status| button::Style {
                            background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                            text_color: iced::Color::from_rgb8(255, 224, 181),
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        }),
                ]
                .spacing(10),
            )
            .width(Length::FillPortion(6))
            .height(Length::Fill)
            .padding(5)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(255, 224, 181))),
                text_color: Some(iced::Color::from_rgb8(15, 9, 9)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
        } else {
            container(text("Pane hidden."))
                .width(Length::FillPortion(6))
                .height(Length::Fill)
                .padding(5)
                .style(|_theme| container::Style {
                    background: Some(iced::Background::Color(Color::from_rgb8(255, 224, 181))),
                    text_color: Some(iced::Color::from_rgb8(15, 9, 9)),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                })
        };

        let capture_ui = row![capture_sidebar, capture_pane];

        let content = container(capture_ui)
            .width(Length::Fill)
            .height(Length::Fill);

        if self.show_error {
            // let error_overlay = self.ui_error.as_ref().map(|msg| { container(text(msg))
            let error_overlay = container(text("Cannot begin with underscore!"))
                .width(Length::Fill)
                .height(Length::Shrink)
                .center_x(Length::Shrink)
                .center_y(Length::Shrink)
                .padding(10)
                .style(|_theme| container::Style {
                    text_color: Some(iced::Color::from_rgb8(15, 9, 9)),
                    background: Some(iced::Background::Color(Color::from_rgb8(255, 182, 182))),
                    border: iced::Border::default(),
                    shadow: iced::Shadow::default(),
                });

            modal(content, error_overlay, Message::HideError)
        } else {
            content.into()
        }
    }
}

impl Yoink {
    fn hide_error(&mut self) {
        self.show_error = false;
    }
}

fn modal<'a, Message>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_blur: Message,
) -> Element<'a, Message>
where
    Message: Clone + 'a,
{
    stack![
        base.into(),
        opaque(
            mouse_area(center(opaque(content)).style(|_theme| {
                container::Style {
                    background: Some(
                        Color {
                            a: 0.8,
                            ..Color::BLACK
                        }
                        .into(),
                    ),
                    ..container::Style::default()
                }
            }))
            .on_press(on_blur)
        )
    ]
    .into()
}
