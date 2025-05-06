use std::time::Instant;

use iced::advanced::widget::operation::focusable::focus;
use iced::widget::{
    button, center, column as col, container, mouse_area, opaque, pane_grid, row, scrollable,
    stack, text, text_editor, text_input,
};
use iced::Length::Shrink;
use iced::{Border, Color, Element, Length};

use crate::capture::capture_models::Capture;
use crate::capture::capture_pane::CapturePane;
use crate::capture::capture_sidebar::CaptureSidebar;
use crate::editor::editor_models::Editor;
use crate::editor::editor_pane::EditorPane;
use crate::editor::editor_sidebar::EditorSidebar;
use crate::enums::message::Message;
use crate::enums::pane::PaneState;

pub struct Yoink {
    pub is_capture: bool,
    pub editor: Editor,
    pub captures: Vec<Vec<String>>,
    pub capture: Capture,
    pub capture_pane: CapturePane,
    pub capture_sidebar: CaptureSidebar,
    pub files: Vec<String>,
    pub editor_pane: EditorPane,
    pub editor_sidebar: EditorSidebar,
    pub opened_file: Vec<String>,
    pub ui_error: String,
    pub show_helper: bool,
    pub panes: pane_grid::State<PaneState>,
    pub last_updated: Instant,
    pub submit_enabled: bool,
    pub is_subselect_capture: bool,
    pub newfile_submit_enabled: bool,
    pub modal_helper: bool,
}

impl Yoink {
    pub fn hide_helper(&mut self) {
        self.show_helper = false;
    }
    pub fn hide_modal_helper(&mut self) {
        self.modal_helper = false;
    }

    // fn hide_subselect_capture(&mut self) {
    //     self.is_subselect_capture = false;
    // }

    pub fn view_capture_sidebar(&self) -> Element<Message> {
        let capture_list = self
            .captures
            .iter()
            .enumerate()
            .map(|(i, capture)| {
                let capture_button = mouse_area(
                    button(row![
                        col(capture
                            .iter()
                            .map(|field| text(field).into())
                            .collect::<Vec<Element<Message>>>()),
                        button("DEL").on_press(Message::DeleteCapture(i))
                    ])
                    .width(750)
                    .on_press(Message::CaptureSelected(i))
                    .style(|_theme, status| match status {
                        button::Status::Hovered => button::Style {
                            background: Some(iced::Background::Color(Color::from_rgb8(25, 19, 19))),
                            text_color: iced::Color::from_rgb8(255, 224, 181),
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        },
                        _ => button::Style {
                            background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                            text_color: iced::Color::from_rgb8(255, 224, 181),
                            border: iced::Border::default(),
                            shadow: iced::Shadow::default(),
                        },
                    }),
                )
                .on_right_press(Message::SubselectCapture);

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
                    .height(50)
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
                        .height(400),
                    row![button("Switch").on_press(Message::Edit)]
                        .width(Shrink)
                        .height(50)
                        .align_y(iced::Alignment::Center)
                ]
                .height(500)
                .spacing(10),
            )
            .width(750)
            .height(520)
            .padding(5)
            .style(|_theme| container::Style {
                text_color: Some(iced::Color::from_rgb8(255, 224, 181)),
                background: Some(iced::Background::Color(Color::from_rgb8(25, 19, 19))),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
        } else {
            container(col![
                text("Sidebar hidden."),
                button("Switch").on_press(Message::Edit)
            ])
            .style(|_theme| container::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                text_color: Some(iced::Color::from_rgb8(255, 224, 181)),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
        };

        col!(capture_sidebar).width(750).max_width(750).into()
    }

    pub fn view_capture_pane(&self) -> Element<Message> {
        let submit_button = self.view_submit_button();
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
                    submit_button,
                ]
                .max_width(400)
                .spacing(10),
            )
            .align_x(iced::Alignment::Center)
            .align_y(iced::Alignment::Center)
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
    }

    pub fn view_editor_sidebar(&self) -> Element<Message> {
        let editor_list = self
            .files
            .iter()
            .map(|file| {
                let file_button = button(row![
                    text(file.clone()),
                    button("DEL").on_press(Message::DeleteFile(file.clone()))
                ])
                .width(Length::Fill)
                // TODO: to FileSelected
                .on_press(Message::FileSelected(file.to_string()))
                .style(|_theme, status| match status {
                    button::Status::Hovered => button::Style {
                        background: Some(iced::Background::Color(Color::from_rgb8(25, 19, 19))),
                        text_color: iced::Color::from_rgb8(255, 224, 181),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    },
                    _ => button::Style {
                        background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                        text_color: iced::Color::from_rgb8(255, 224, 181),
                        border: iced::Border::default(),
                        shadow: iced::Shadow::default(),
                    },
                });

                file_button.into()
            })
            .collect::<Vec<Element<Message>>>();

        let editor_sidebar = if self.editor_sidebar.is_visible {
            container(
                col![
                    row![
                        // TODO: to FileSearchChanged
                        text_input("Editor..", &self.capture.search)
                            .on_input(Message::FileSearchChanged),
                        button("X")
                    ]
                    .height(Length::FillPortion(2))
                    .align_y(iced::Alignment::Center),
                    scrollable(col(editor_list).spacing(5))
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

        col!(editor_sidebar).width(200).max_width(200).into()
    }

    pub fn view_editor_pane(&self) -> Element<Message> {
        let editor_pane = if self.editor_pane.is_visible {
            container(col![
                row![text(self.capture.current_capture.clone()),].align_y(iced::Alignment::Center),
                text_editor(&self.editor.editor_content)
                    .on_action(Message::EditorContentChanged)
                    .height(Length::Fill)
                    .padding(10),
                row![
                    button("submit file").on_press(Message::UpdateFile),
                    button("create file").on_press(Message::CreateFile),
                    button("create file").on_press(Message::ViewModalHelper)
                ]
            ])
            .padding(10)
        } else {
            container(text("editor_pane hidden.."))
        };

        col!(editor_pane).into()
    }

    fn view_submit_button(&self) -> Element<Message> {
        let mut submit_button = button("Submit").style(|_theme, status| match status {
            button::Status::Hovered => button::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(35, 29, 29))),
                text_color: iced::Color::from_rgb8(255, 224, 181),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            button::Status::Active => button::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                text_color: iced::Color::from_rgb8(255, 224, 181),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            button::Status::Disabled => button::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(65, 59, 59))),
                text_color: iced::Color::from_rgb8(255, 224, 181),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            _ => button::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                text_color: iced::Color::from_rgb8(255, 224, 181),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
        });

        if self.submit_enabled {
            submit_button = submit_button.on_press(Message::SubmitCapture);
        } else {
            submit_button = submit_button;
        }

        submit_button.into()
    }

    pub fn view_newfile_submit_button(&self) -> Element<Message> {
        let mut submit_button = button("Submit").style(|_theme, status| match status {
            button::Status::Hovered => button::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(35, 29, 29))),
                text_color: iced::Color::from_rgb8(255, 224, 181),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            button::Status::Active => button::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                text_color: iced::Color::from_rgb8(255, 224, 181),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            button::Status::Disabled => button::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(65, 59, 59))),
                text_color: iced::Color::from_rgb8(255, 224, 181),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
            _ => button::Style {
                background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                text_color: iced::Color::from_rgb8(255, 224, 181),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            },
        });

        if self.newfile_submit_enabled {
            submit_button = submit_button.on_press(Message::CreateNewFile);
        } else {
            submit_button = submit_button;
        }

        submit_button.into()
    }

    pub fn update_submit_enabled(&mut self) {
        self.submit_enabled = !self.capture.form_topic.is_empty()
            && !self.capture.form_subject.is_empty()
            && !self.capture.form_content.text().trim().is_empty()
    }

    pub fn create_new_file_enabled(&mut self) {
        self.newfile_submit_enabled = !self.editor.new_file.is_empty()
    }

    pub fn modal<'a, Message>(
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
}
