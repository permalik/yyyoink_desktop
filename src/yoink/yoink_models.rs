use std::time::Instant;

use iced::advanced::widget::operation::{focusable, Focusable};
use iced::border::{self, Radius};
use iced::widget::{
    button, center, column as col, container, mouse_area, opaque, pane_grid, row, scrollable,
    stack, text, text_editor, text_input,
};
use iced::Length::Shrink;
use iced::{
    mouse, overlay, touch, Background, Border, Color, Element, Length, Padding, Rectangle, Shadow,
    Size, Theme, Vector,
};

use crate::capture::capture_models::Capture;
use crate::capture::capture_pane::CapturePane;
use crate::capture::capture_sidebar::CaptureSidebar;
use crate::editor::editor_models::Editor;
use crate::editor::editor_pane::EditorPane;
use crate::editor::editor_sidebar::EditorSidebar;
use crate::enums::message::Message;
use crate::enums::pane::PaneState;
use iced::advanced::layout::{self, Layout};
use iced::advanced::widget::tree::{self, Tree};
use iced::advanced::widget::{operation, Id, Operation, Widget};
use iced::advanced::{renderer, Clipboard, Shell};
use iced::event::{self, Event};
use iced::keyboard;
use iced::theme::palette;

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
                let text_fields = capture
                    .iter()
                    .map(|field| {
                        text_input("", field)
                            .on_input(|_| Message::Ignore)
                            .padding(5)
                            .style(|_theme, _status| text_input::Style {
                                background: Background::Color(Color::BLACK),
                                border: Border::default(),
                                icon: iced::Color::from_rgb8(255, 244, 181),
                                placeholder: iced::Color::from_rgb8(255, 244, 181),
                                value: iced::Color::from_rgb8(255, 244, 181),
                                selection: iced::Color::from_rgb8(255, 244, 181),
                            })
                            .into()
                        // let capture_button = mouse_area(
                        //     button(row![
                        //         col(capture
                        //             .iter()
                        //             .map(|field| text(field).into())
                        //             .collect::<Vec<Element<Message>>>()),
                        //         button("DEL").on_press(Message::DeleteCapture(i))
                        //     ])
                        //     .width(750)
                        //     .on_press(Message::CaptureSelected(i))
                        //     .style(|_theme, status| match status {
                        //         button::Status::Hovered => button::Style {
                        //             background: Some(iced::Background::Color(Color::from_rgb8(25, 19, 19))),
                        //             text_color: iced::Color::from_rgb8(255, 224, 181),
                        //             border: iced::Border::default(),
                        //             shadow: iced::Shadow::default(),
                        //         },
                        //         _ => button::Style {
                        //             background: Some(iced::Background::Color(Color::from_rgb8(15, 9, 9))),
                        //             text_color: iced::Color::from_rgb8(255, 224, 181),
                        //             border: iced::Border::default(),
                        //             shadow: iced::Shadow::default(),
                        //         },
                        //     }),
                        // )
                        // .on_right_press(Message::SubselectCapture);
                        //
                        // capture_button.into()
                    })
                    .collect::<Vec<Element<Message>>>();

                let capture_item = mouse_area(row![col(text_fields)].width(750));

                capture_item.into()
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
        let input = text("Placeholder");
        let mybutton = CustomButton::new(input)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .padding(10)
            .clip(false)
            .on_press(Message::Ignore)
            .style(|_theme, _status| CustomStyle::default());
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
                    button("create file").on_press(Message::ViewModalHelper),
                    mybutton,
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

#[allow(missing_debug_implementations)]
pub struct CustomButton<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<OnPress<'a, Message>>,
    width: Length,
    height: Length,
    padding: Padding,
    clip: bool,
    class: Theme::Class<'a>,
}

enum OnPress<'a, Message> {
    Direct(Message),
    Closure(Box<dyn Fn() -> Message + 'a>),
}

impl<'a, Message: Clone> OnPress<'a, Message> {
    fn get(&self) -> Message {
        match self {
            OnPress::Direct(message) => message.clone(),
            OnPress::Closure(f) => f(),
        }
    }
}

impl<'a, Message, Theme, Renderer> CustomButton<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: Catalog,
{
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        let content = content.into();
        let size = content.as_widget().size_hint();

        CustomButton {
            content,
            on_press: None,
            width: size.width.fluid(),
            height: size.height.fluid(),
            padding: DEFAULT_PADDING,
            clip: false,
            class: Theme::default(),
        }
    }

    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn on_press(mut self, on_press: Message) -> Self {
        self.on_press = Some(OnPress::Direct(on_press));
        self
    }

    pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
        self.on_press = Some(OnPress::Closure(Box::new(on_press)));
        self
    }
}

// pub struct CustomButton<'a, Message, Theme = iced::Theme, Renderer = iced::Renderer>
// where
//     Renderer: renderer::Renderer,
//     Theme: Catalog,
// {
//     content: Element<'a, Message, Theme, Renderer>,
//     on_press: Option<OnPress<'a, Message>>,
//     id: Id,
//     width: Length,
//     height: Length,
//     padding: Padding,
//     clip: bool,
//     class: Theme::Class<'a>,
// }
//
// enum OnPress<'a, Message> {
//     Direct(Message),
//     Closure(Box<dyn Fn() -> Message + 'a>),
// }
//
// impl<'a, Message: Clone> OnPress<'a, Message> {
//     fn get(&self) -> Message {
//         match self {
//             OnPress::Direct(message) => message.clone(),
//             OnPress::Closure(f) => f(),
//         }
//     }
// }
//
// impl<'a, Message, Theme, Renderer> CustomButton<'a, Message, Theme, Renderer>
// where
//     Renderer: renderer::Renderer,
//     Theme: Catalog,
// {
//     /// Creates a new [`CustomButton`] with the given content.
//     pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
//         let content = content.into();
//         let size = content.as_widget().size_hint();
//
//         CustomButton {
//             content,
//             id: Id::unique(),
//             on_press: None,
//             width: size.width.fluid(),
//             height: size.height.fluid(),
//             padding: DEFAULT_PADDING,
//             clip: false,
//             class: Theme::default(),
//         }
//     }
//
//     /// Sets the width of the [`CustomButton`].
//     pub fn width(mut self, width: impl Into<Length>) -> Self {
//         self.width = width.into();
//         self
//     }
//
//     /// Sets the height of the [`CustomButton`].
//     pub fn height(mut self, height: impl Into<Length>) -> Self {
//         self.height = height.into();
//         self
//     }
//
//     /// Sets the [`Padding`] of the [`CustomButton`].
//     pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
//         self.padding = padding.into();
//         self
//     }
//
//     /// Sets the message that will be produced when the [`Button`] is pressed.
//     ///
//     /// Unless `on_press` is called, the [`CustomButton`] will be disabled.
//     pub fn on_press(mut self, on_press: Message) -> Self {
//         self.on_press = Some(OnPress::Direct(on_press));
//         self
//     }
//
//     /// Sets the message that will be produced when the [`CustomButton`] is pressed.
//     ///
//     /// This is analogous to [`CustomButton::on_press`], but using a closure to produce
//     /// the message.
//     ///
//     /// This closure will only be called when the [`CustomButton`] is actually pressed and,
//     /// therefore, this method is useful to reduce overhead if creating the resulting
//     /// message is slow.
//     pub fn on_press_with(mut self, on_press: impl Fn() -> Message + 'a) -> Self {
//         self.on_press = Some(OnPress::Closure(Box::new(on_press)));
//         self
//     }
//
//     /// Sets the message that will be produced when the [`CustomButton`] is pressed,
//     /// if `Some`.
//     ///
//     /// If `None`, the [`CustomButton`] will be disabled.
//     pub fn on_press_maybe(mut self, on_press: Option<Message>) -> Self {
//         self.on_press = on_press.map(OnPress::Direct);
//         self
//     }
//
//     /// Sets whether the contents of the [`CustomButton`] should be clipped on
//     /// overflow.
//     pub fn clip(mut self, clip: bool) -> Self {
//         self.clip = clip;
//         self
//     }
//
//     /// Sets the style of the [`CustomButton`].
//     #[must_use]
//     pub fn style(mut self, style: impl Fn(&Theme, Status) -> CustomStyle + 'a) -> Self
//     where
//         Theme::Class<'a>: From<StyleFn<'a, Theme>>,
//     {
//         self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
//         self
//     }
//
//     // TODO: IMPL THIS
//     // /// Sets the style class of the [`Button`].
//     // #[cfg(feature = "advanced")]
//     // #[must_use]
//     // pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
//     //     self.class = class.into();
//     //     self
//     // }
//
//     /// Sets the [`Id`] of the [`CustomButton`].
//     pub fn id(mut self, id: Id) -> Self {
//         self.id = id;
//         self
//     }
// }
//
// // impl<'a, Message, Theme, Renderer> Focusable for CustomButton<'a, Message, Theme, Renderer>
// // where
// //     Renderer: renderer::Renderer,
// //     Theme: Catalog,
// // {
// //     fn is_focused(&self) -> bool {
// //         self.is_focused
// //     }
// //
// //     fn focus(&mut self) {
// //         self.is_focused = true;
// //     }
// //
// //     fn unfocus(&mut self) {
// //         self.is_focused = false;
// //     }
// // }
//
// #[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
// struct State {
//     is_hovered: bool,
//     is_pressed: bool,
//     is_focused: bool,
// }
//
// impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
//     for CustomButton<'a, Message, Theme, Renderer>
// where
//     Message: 'a + Clone,
//     Renderer: 'a + renderer::Renderer,
//     Theme: Catalog,
// {
//     fn tag(&self) -> tree::Tag {
//         tree::Tag::of::<State>()
//     }
//
//     fn state(&self) -> tree::State {
//         tree::State::new(State::default())
//     }
//
//     fn children(&self) -> Vec<Tree> {
//         vec![Tree::new(&self.content)]
//     }
//
//     fn diff(&self, tree: &mut Tree) {
//         tree.diff_children(std::slice::from_ref(&self.content));
//     }
//
//     fn size(&self) -> Size<Length> {
//         Size {
//             width: self.width,
//             height: self.height,
//         }
//     }
//
//     fn layout(
//         &self,
//         tree: &mut Tree,
//         renderer: &Renderer,
//         limits: &layout::Limits,
//     ) -> layout::Node {
//         layout::padded(limits, self.width, self.height, self.padding, |limits| {
//             self.content
//                 .as_widget()
//                 .layout(&mut tree.children[0], renderer, limits)
//         })
//     }
//
//     fn operate(
//         &self,
//         tree: &mut Tree,
//         layout: Layout<'_>,
//         renderer: &Renderer,
//         operation: &mut dyn Operation,
//     ) {
//         operation.container(None, layout.bounds(), &mut |operation| {
//             self.content.as_widget().operate(
//                 &mut tree.children[0],
//                 layout.children().next().unwrap(),
//                 renderer,
//                 operation,
//             );
//         });
//     }
//
//     fn on_event(
//         &mut self,
//         tree: &mut Tree,
//         event: Event,
//         layout: Layout<'_>,
//         cursor: mouse::Cursor,
//         renderer: &Renderer,
//         clipboard: &mut dyn Clipboard,
//         shell: &mut Shell<'_, Message>,
//         viewport: &Rectangle,
//     ) -> event::Status {
//         if let event::Status::Captured = self.content.as_widget_mut().on_event(
//             &mut tree.children[0],
//             event.clone(),
//             layout.children().next().unwrap(),
//             cursor,
//             renderer,
//             clipboard,
//             shell,
//             viewport,
//         ) {
//             return event::Status::Captured;
//         }
//
//         match event {
//             Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left))
//             | Event::Touch(touch::Event::FingerPressed { .. }) => {
//                 if self.on_press.is_some() {
//                     let bounds = layout.bounds();
//
//                     if cursor.is_over(bounds) {
//                         let state = tree.state.downcast_mut::<State>();
//
//                         state.is_pressed = true;
//
//                         return event::Status::Captured;
//                     }
//                 }
//             }
//             Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))
//             | Event::Touch(touch::Event::FingerLifted { .. }) => {
//                 if let Some(on_press) = self.on_press.as_ref().map(OnPress::get) {
//                     let state = tree.state.downcast_mut::<State>();
//
//                     if state.is_pressed {
//                         state.is_pressed = false;
//
//                         let bounds = layout.bounds();
//
//                         if cursor.is_over(bounds) {
//                             shell.publish(on_press);
//                         }
//
//                         return event::Status::Captured;
//                     }
//                 }
//             }
//             Event::Keyboard(keyboard::Event::KeyPressed { key, .. }) => {
//                 if let Some(on_press) = self.on_press.as_ref() {
//                     let state = tree.state.downcast_mut::<State>();
//                     if state.is_focused
//                         && matches!(key, keyboard::Key::Named(keyboard::key::Named::Enter))
//                     {
//                         state.is_pressed = true;
//                         shell.publish(on_press.get());
//                         return event::Status::Captured;
//                     }
//                 }
//             }
//             Event::Touch(touch::Event::FingerLost { .. })
//             | Event::Mouse(mouse::Event::CursorLeft) => {
//                 let state = tree.state.downcast_mut::<State>();
//                 state.is_hovered = false;
//                 state.is_pressed = false;
//             }
//             _ => {}
//         }
//
//         event::Status::Ignored
//     }
//
//     fn draw(
//         &self,
//         tree: &Tree,
//         renderer: &mut Renderer,
//         theme: &Theme,
//         renderer_style: &renderer::Style,
//         layout: Layout<'_>,
//         cursor: mouse::Cursor,
//         viewport: &Rectangle,
//     ) {
//         let bounds = layout.bounds();
//         let content_layout = layout.children().next().unwrap();
//         let is_mouse_over = cursor.is_over(bounds);
//
//         let status = if self.on_press.is_none() {
//             Status::Disabled
//         } else if is_mouse_over {
//             let state = tree.state.downcast_ref::<State>();
//
//             if state.is_pressed {
//                 Status::Pressed
//             } else {
//                 Status::Hovered
//             }
//         } else {
//             Status::Active
//         };
//
//         let style = theme.style(&self.class, status);
//
//         if style.background.is_some() || style.border.width > 0.0 || style.shadow.color.a > 0.0 {
//             renderer.fill_quad(
//                 renderer::Quad {
//                     bounds,
//                     border: style.border,
//                     shadow: style.shadow,
//                 },
//                 style
//                     .background
//                     .unwrap_or(Background::Color(Color::TRANSPARENT)),
//             );
//         }
//
//         let viewport = if self.clip {
//             bounds.intersection(viewport).unwrap_or(*viewport)
//         } else {
//             *viewport
//         };
//
//         self.content.as_widget().draw(
//             &tree.children[0],
//             renderer,
//             theme,
//             &renderer::Style {
//                 text_color: style.text_color,
//                 // TODO: IMPL THIS
//                 // icon_color: style.icon_color.unwrap_or(renderer_style.icon_color),
//                 // scale_factor: renderer_style.scale_factor,
//             },
//             content_layout,
//             cursor,
//             &viewport,
//         );
//     }
//
//     fn mouse_interaction(
//         &self,
//         _tree: &Tree,
//         layout: Layout<'_>,
//         cursor: mouse::Cursor,
//         _viewport: &Rectangle,
//         _renderer: &Renderer,
//     ) -> mouse::Interaction {
//         let is_mouse_over = cursor.is_over(layout.bounds());
//
//         if is_mouse_over && self.on_press.is_some() {
//             mouse::Interaction::Pointer
//         } else {
//             mouse::Interaction::default()
//         }
//     }
//
//     fn overlay<'b>(
//         &'b mut self,
//         tree: &'b mut Tree,
//         layout: Layout<'_>,
//         renderer: &Renderer,
//         translation: Vector,
//     ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
//         self.content.as_widget_mut().overlay(
//             &mut tree.children[0],
//             layout.children().next().unwrap(),
//             renderer,
//             translation,
//         )
//     }
//
//     // fn id(&self) -> Option<Id> {
//     //     Some(self.id.clone())
//     // }
//     //
//     // fn set_id(&mut self, id: Id) {
//     //     self.id = id;
//     // }
// }
//
// impl<'a, Message, Theme, Renderer> From<CustomButton<'a, Message, Theme, Renderer>>
//     for Element<'a, Message, Theme, Renderer>
// where
//     Message: Clone + 'a,
//     Theme: Catalog + 'a,
//     Renderer: renderer::Renderer + 'a,
// {
//     fn from(button: CustomButton<'a, Message, Theme, Renderer>) -> Self {
//         Self::new(button)
//     }
// }
//
// /// The default [`Padding`] of a [`Button`].
// pub const DEFAULT_PADDING: Padding = Padding {
//     top: 5.0,
//     bottom: 5.0,
//     right: 10.0,
//     left: 10.0,
// };
//
// /// The possible status of a [`Button`].
// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
// pub enum Status {
//     /// The [`Button`] can be pressed.
//     Active,
//     /// The [`Button`] can be pressed and it is being hovered.
//     Hovered,
//     /// The [`Button`] is being pressed.
//     Pressed,
//     /// The [`Button`] cannot be pressed.
//     Disabled,
// }
//
// /// The style of a button.
// ///
// /// If not specified with [`Button::style`]
// /// the theme will provide the style.
// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct CustomStyle {
//     /// The [`Background`] of the button.
//     pub background: Option<Background>,
//     /// The border radius of the button.
//     pub border_radius: Radius,
//     /// The border width of the button.
//     pub border_width: f32,
//     /// The border [`Color`] of the button.
//     pub border_color: Color,
//     /// The icon [`Color`] of the button.
//     pub icon_color: Option<Color>,
//     /// The text [`Color`] of the button.
//     pub text_color: Color,
//     /// The [`Border`] of the button.
//     pub border: Border,
//     /// The [`Shadow`] of the button.
//     pub shadow: Shadow,
// }
//
// impl CustomStyle {
//     /// Updates the [`Style`] with the given [`Background`].
//     pub fn with_background(self, background: impl Into<Background>) -> Self {
//         Self {
//             background: Some(background.into()),
//             ..self
//         }
//     }
//
//     // /// Returns whether the [`Button`] is currently focused or not.
//     // pub fn is_focused(&self) -> bool {
//     //     self.is_focused
//     // }
//
//     // /// Returns whether the [`Button`] is currently hovered or not.
//     // pub fn is_hovered(&self) -> bool {
//     //     self.is_hovered
//     // }
//
//     // /// Focuses the [`Button`].
//     // pub fn focus(&mut self) {
//     //     self.is_focused = true;
//     // }
//
//     // /// Unfocuses the [`Button`].
//     // pub fn unfocus(&mut self) {
//     //     self.is_focused = false;
//     // }
// }
//
// impl Default for CustomStyle {
//     fn default() -> Self {
//         Self {
//             background: None,
//             border_radius: 0.0.into(),
//             border_width: 0.0,
//             border_color: Color::TRANSPARENT,
//             icon_color: None,
//             text_color: Color::BLACK,
//             border: Border::default(),
//             shadow: Shadow::default(),
//         }
//     }
// }
//
// pub trait Catalog {
//     /// The item class of the [`Catalog`].
//     type Class<'a>;
//
//     /// The default class produced by the [`Catalog`].
//     fn default<'a>() -> Self::Class<'a>;
//
//     /// The [`Style`] of a class with the given status.
//     fn style(&self, class: &Self::Class<'_>, status: Status) -> CustomStyle;
// }
//
// /// A styling function for a [`Button`].
// pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> CustomStyle + 'a>;
//
// impl Catalog for Theme {
//     type Class<'a> = StyleFn<'a, Self>;
//
//     fn default<'a>() -> Self::Class<'a> {
//         Box::new(primary)
//     }
//
//     fn style(&self, class: &Self::Class<'_>, status: Status) -> CustomStyle {
//         class(self, status)
//     }
// }
//
// /// A primary button; denoting a main action.
// pub fn primary(theme: &Theme, status: Status) -> CustomStyle {
//     let palette = theme.extended_palette();
//     let base = styled(palette.primary.strong);
//
//     match status {
//         Status::Active | Status::Pressed => base,
//         Status::Hovered => CustomStyle {
//             background: Some(Background::Color(palette.primary.base.color)),
//             ..base
//         },
//         Status::Disabled => disabled(base),
//     }
// }
//
// /// A secondary button; denoting a complementary action.
// pub fn secondary(theme: &Theme, status: Status) -> CustomStyle {
//     let palette = theme.extended_palette();
//     let base = styled(palette.secondary.base);
//
//     match status {
//         Status::Active | Status::Pressed => base,
//         Status::Hovered => CustomStyle {
//             background: Some(Background::Color(palette.secondary.strong.color)),
//             ..base
//         },
//         Status::Disabled => disabled(base),
//     }
// }
//
// /// A success button; denoting a good outcome.
// pub fn success(theme: &Theme, status: Status) -> CustomStyle {
//     let palette = theme.extended_palette();
//     let base = styled(palette.success.base);
//
//     match status {
//         Status::Active | Status::Pressed => base,
//         Status::Hovered => CustomStyle {
//             background: Some(Background::Color(palette.success.strong.color)),
//             ..base
//         },
//         Status::Disabled => disabled(base),
//     }
// }
//
// /// A danger button; denoting a destructive action.
// pub fn danger(theme: &Theme, status: Status) -> CustomStyle {
//     let palette = theme.extended_palette();
//     let base = styled(palette.danger.base);
//
//     match status {
//         Status::Active | Status::Pressed => base,
//         Status::Hovered => CustomStyle {
//             background: Some(Background::Color(palette.danger.strong.color)),
//             ..base
//         },
//         Status::Disabled => disabled(base),
//     }
// }
//
// /// A text button; useful for links.
// pub fn custom_text(theme: &Theme, status: Status) -> CustomStyle {
//     let palette = theme.extended_palette();
//
//     let base = CustomStyle {
//         text_color: palette.background.base.text,
//         ..CustomStyle::default()
//     };
//
//     match status {
//         Status::Active | Status::Pressed => base,
//         Status::Hovered => CustomStyle {
//             text_color: palette.background.base.text.scale_alpha(0.8),
//             ..base
//         },
//         Status::Disabled => disabled(base),
//     }
// }
//
// fn styled(pair: palette::Pair) -> CustomStyle {
//     CustomStyle {
//         background: Some(Background::Color(pair.color)),
//         text_color: pair.text,
//         border: border::rounded(2),
//         ..CustomStyle::default()
//     }
// }
//
// fn disabled(style: CustomStyle) -> CustomStyle {
//     CustomStyle {
//         background: style
//             .background
//             .map(|background| background.scale_alpha(0.5)),
//         text_color: style.text_color.scale_alpha(0.5),
//         ..style
//     }
// }
//
// /// Produces a [`Task`] that focuses the [`Button`] with the given [`Id`].
// pub fn focus<Message: 'static + Send>(id: Id) -> impl Operation<Message> {
//     focusable::focus(id)
// }
//
// impl operation::Focusable for State {
//     fn is_focused(&self) -> bool {
//         self.is_focused
//     }
//
//     fn focus(&mut self) {
//         self.is_focused = true;
//     }
//
//     fn unfocus(&mut self) {
//         self.is_focused = false;
//     }
// }
