mod capture;
mod editor;
mod enums;
mod utilities;
use capture::capture_models::Capture;
use capture::capture_pane::CapturePane;
use capture::capture_sidebar::CaptureSidebar;
use chrono::prelude::*;
use editor::editor_models::Editor;
use enums::message::Message;
use enums::pane::PaneState;
use iced::event::{self, Event};
use iced::keyboard::key;
use iced::widget::{
    self, button, center, column as col, container, mouse_area, opaque, pane_grid, row, scrollable,
    stack, text, text_editor, text_input,
};
use iced::{keyboard, Border};
use iced::{Color, Element, Font, Length, Subscription, Task};
use utilities::file;

pub fn main() -> iced::Result {
    iced::application("Yoink Desktop", Yoink::update, Yoink::view)
        .subscription(Yoink::subscription)
        .default_font(Font::MONOSPACE)
        .run_with(Yoink::new)
}

struct Yoink {
    is_capture: bool,
    editor: Editor,
    captures: Vec<Vec<String>>,
    capture: Capture,
    capture_pane: CapturePane,
    capture_sidebar: CaptureSidebar,
    ui_error: String,
    show_error: bool,
    panes: pane_grid::State<PaneState>,
}

impl Yoink {
    fn new() -> (Self, Task<Message>) {
        let (mut panes, sidebar) = pane_grid::State::new(PaneState::CaptureSidebarPane);
        let _pane = panes.split(
            pane_grid::Axis::Vertical,
            sidebar,
            PaneState::CaptureFormPane,
        );
        (
            Self {
                is_capture: true,
                editor: Editor::new(),
                captures: Vec::new(),
                capture: Capture::new(),
                capture_pane: CapturePane::new(),
                capture_sidebar: CaptureSidebar::new(),
                ui_error: String::new(),
                show_error: false,
                panes,
            },
            Task::perform(file::load_captures(), Message::CapturesLoaded),
        )
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen().map(Message::Event)
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Edit => {
                if self.is_capture == true {
                    self.is_capture = false;

                    let (mut panes, sidebar) = pane_grid::State::new(PaneState::EditorSidebarPane);
                    let _pane =
                        panes.split(pane_grid::Axis::Vertical, sidebar, PaneState::EditorPane);
                    self.panes = panes;
                } else {
                    self.is_capture = true;

                    let (mut panes, sidebar) = pane_grid::State::new(PaneState::CaptureSidebarPane);
                    let _pane = panes.split(
                        pane_grid::Axis::Vertical,
                        sidebar,
                        PaneState::CaptureFormPane,
                    );
                    self.panes = panes;
                }

                Task::none()
            }
            Message::EditorContentChanged(action) => {
                self.editor.editor_content.perform(action);
                Task::none()
            }
            Message::CaptureSelected(index) => {
                // if let Ok(capture) = result {
                //     println!("{}", capture);
                // }
                if let Some(capture_data) = self.captures.get(index) {
                    //self.capture.open_capture = Some(capture.to_string_lossy().to_string());
                    for capture in capture_data {
                        println!("CaptureSelected: {}", capture);
                    }
                    let capture_input = capture_data.clone();
                    Task::perform(file::capture_opened(capture_input), Message::CaptureOpened)
                } else {
                    Task::none()
                }
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
                Task::none()
            }
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
                if self.capture.form_topic.is_empty()
                    || self.capture.form_subject.is_empty()
                    || self.capture.form_content.text().trim().is_empty()
                {
                    self.ui_error = "Submission failed: Inputs cannot be null.".to_string();
                    Task::perform(file::log(), Message::ShowError)
                } else {
                    println!("Search: {}", self.capture.search);
                    println!("Topic: {}", self.capture.form_topic);
                    println!("Subject: {}", self.capture.form_subject);
                    let form_content = self.capture.form_content.text();
                    println!("{}", form_content);
                    let file_name_prefix = "_";
                    let file_name_ext = ".md";
                    let form_topic = format!(
                        "{}{}{}",
                        file_name_prefix, self.capture.form_topic, file_name_ext
                    );
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

                    Task::perform(
                        file::write_file(form_topic, capture_string),
                        Message::FileOpened,
                    )
                }
            }
            Message::FileOpened(result) => {
                if let Ok(path) = result {
                    self.capture.updated_file = Some(path.to_string_lossy().to_string());
                    println!("Opened/Written to {}", path.display());
                }
                Task::none()
            }
            Message::CaptureOpened(result) => {
                if let Ok((timestamp, path, subject)) = result {
                    self.capture.opened_capture = Some((timestamp, path, subject));

                    if let Some((ref timestamp_str, ref path_str, ref subject_str)) =
                        self.capture.opened_capture
                    {
                        println!(
                            "CaptureOpened {} {} {}",
                            timestamp_str,
                            path_str.to_string_lossy().to_string(),
                            subject_str
                        );
                    }
                }
                Task::none()
            }
            Message::Event(event) => match event {
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Tab),
                    modifiers,
                    ..
                }) => {
                    if modifiers.shift() {
                        widget::focus_previous()
                    } else {
                        widget::focus_next()
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Escape),
                    ..
                }) => {
                    self.hide_error();
                    Task::none()
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Enter),
                    ..
                }) => Task::perform(async {}, |_| Message::SubmitCapture),
                Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                    if let Some(result) = file::handle_hotkey(key, modifiers) {
                        Task::perform(async move { result }, |msg| msg)
                    } else {
                        Task::none()
                    }
                }
                _ => Task::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = pane_grid::PaneGrid::new(&self.panes, |_pane, state, _is_maximized| {
            if self.is_capture {
                let content: Element<_> = match state {
                    PaneState::CaptureSidebarPane => self.view_capture_sidebar(),
                    PaneState::CaptureFormPane => self.view_capture_pane(),
                    PaneState::EditorSidebarPane => self.view_editor_sidebar(),
                    PaneState::EditorPane => self.view_editor_pane(),
                };

                pane_grid::Content::new(content)
            } else {
                let content: Element<_> = match state {
                    PaneState::CaptureSidebarPane => self.view_capture_sidebar(),
                    PaneState::CaptureFormPane => self.view_capture_pane(),
                    PaneState::EditorSidebarPane => self.view_editor_sidebar(),
                    PaneState::EditorPane => self.view_editor_pane(),
                };

                pane_grid::Content::new(content)
            }
        })
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(0)
        .on_resize(10, Message::PaneResized);

        // let capture_ui = row![capture_sidebar, capture_pane];
        //
        // let content = container(capture_ui)
        //     .width(Length::Fill)
        //     .height(Length::Fill);

        if self.show_error {
            let error_overlay = container(text(self.ui_error.to_string()))
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

    fn view_capture_sidebar(&self) -> Element<Message> {
        let capture_list = self
            .captures
            .iter()
            .enumerate()
            .map(|(i, capture)| {
                let capture_button = button(
                    col(capture
                        .iter()
                        .map(|field| text(field).into())
                        .collect::<Vec<Element<Message>>>()),
                    // .padding(Padding {
                    //     top: 10.0,
                    //     right: 25.0,
                    //     bottom: 10.0,
                    //     left: 5.0,
                    // }),
                )
                .width(Length::Fill)
                .on_press(Message::CaptureSelected(i));

                capture_button.into()
            })
            .collect::<Vec<Element<Message>>>();

        let capture_sidebar = if self.capture_sidebar.is_visible {
            container(
                col![
                    row![
                        text_input("Capture..", &self.capture.search)
                            .on_input(Message::CaptureSearchChanged),
                        button("X")
                    ]
                    .height(Length::FillPortion(2))
                    .align_y(iced::Alignment::Center),
                    scrollable(col(capture_list).spacing(5))
                        .style(|_theme, _status| {
                            scrollable::Style {
                                container: container::Style {
                                    text_color: None,
                                    background: None,
                                    border: Border {
                                        color: Color::from_rgb8(0, 0, 0),
                                        width: 0.0,
                                        radius: Default::default(),
                                    },
                                    shadow: Default::default(),
                                },
                                vertical_rail: scrollable::Rail {
                                    background: Some(iced::Background::Color(Color::from_rgb8(
                                        180, 60, 60,
                                    ))),
                                    border: Border {
                                        color: Color::from_rgb8(0, 0, 0),
                                        width: 0.0,
                                        radius: 5.0.into(),
                                    },
                                    scroller: scrollable::Scroller {
                                        color: iced::Color::from_rgb8(120, 30, 30),
                                        border: Border {
                                            color: iced::Color::from_rgba8(0, 0, 0, 0.0),
                                            width: 0.0,
                                            radius: 5.0.into(),
                                        },
                                    },
                                },
                                horizontal_rail: scrollable::Rail {
                                    background: Some(iced::Background::Color(Color::from_rgb8(
                                        180, 60, 60,
                                    ))),
                                    border: Border {
                                        color: Color::from_rgb8(0, 0, 0),
                                        width: 0.0,
                                        radius: 5.0.into(),
                                    },
                                    scroller: scrollable::Scroller {
                                        color: iced::Color::from_rgb8(15, 9, 9),
                                        border: Border {
                                            color: iced::Color::from_rgba8(0, 0, 0, 0.0),
                                            width: 2.0,
                                            radius: 5.0.into(),
                                        },
                                    },
                                },
                                gap: None,
                            }
                        })
                        .height(Length::FillPortion(50)),
                    row![button("Switch").on_press(Message::Edit)]
                        .height(Length::FillPortion(2))
                        .align_y(iced::Alignment::Center)
                ]
                .spacing(10),
            )
            // .width(Length::FillPortion(2))
            //.max_width(100)
            .height(Length::Fill)
            .padding(5)
            .style(|_theme| container::Style {
                text_color: Some(iced::Color::from_rgb8(255, 224, 181)),
                background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
        } else {
            container(col![
                text("Sidebar hidden."),
                button("Switch").on_press(Message::Edit)
            ])
            // .width(Length::FillPortion(2))
            //.max_width(100)
            .height(Length::Fill)
            .padding(5)
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                text_color: Some(iced::Color::from_rgb8(255, 224, 181)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
        };

        col!(capture_sidebar).width(200).max_width(200).into()
        // capture_sidebar.into()
    }

    fn view_capture_pane(&self) -> Element<Message> {
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

        col!(capture_pane).into()
        // capture_pane.into()
    }

    fn view_editor_sidebar(&self) -> Element<Message> {
        let editor_sidebar = if self.capture_sidebar.is_visible {
            container(text("editor_sidebar.."))
        } else {
            container(text("editor_sidebar hidden.."))
        };

        col!(editor_sidebar).width(200).max_width(200).into()
    }

    fn view_editor_pane(&self) -> Element<Message> {
        let editor_pane = if self.capture_pane.is_visible {
            container(col![
                text("editor.."),
                text_editor(&self.editor.editor_content)
                    .on_action(Message::EditorContentChanged)
                    .height(Length::Fill)
                    .padding(10)
            ])
            .padding(10)
        } else {
            container(text("editor_pane hidden.."))
        };

        col!(editor_pane).into()
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
