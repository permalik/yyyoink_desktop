mod capture;
mod editor;
mod enums;
mod utilities;
mod yoink;
use capture::capture_models::Capture;
use capture::capture_pane::CapturePane;
use capture::capture_sidebar::CaptureSidebar;
use chrono::prelude::*;
pub use editor::editor_models::Editor;
use editor::editor_pane::EditorPane;
use editor::editor_sidebar::EditorSidebar;
use enums::message::Message;
use enums::pane::PaneState;
use iced::event::{self, Event};
use iced::keyboard;
use iced::keyboard::key;
use iced::widget::text_editor::Content;
use iced::widget::{self, button, column as col, container, pane_grid, text, text_input};
use iced::{Color, Element, Font, Length, Subscription, Task};
// use iced_aw::ContextMenu;
use std::time::Instant;
use utilities::file;
use yoink::yoink_models::Yoink;

pub fn main() -> iced::Result {
    iced::application("Yoink Desktop", Yoink::update, Yoink::view)
        .subscription(Yoink::subscription)
        .default_font(Font::MONOSPACE)
        .run_with(Yoink::new)
}

impl Yoink {
    pub fn new() -> (Self, Task<Message>) {
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
                files: Vec::new(),
                editor_pane: EditorPane::new(),
                editor_sidebar: EditorSidebar::new(),
                opened_file: Vec::new(),
                ui_error: String::new(),
                show_helper: false,
                panes,
                last_updated: Instant::now(),
                submit_enabled: false,
                is_subselect_capture: false,
                newfile_submit_enabled: false,
                modal_helper: false,
                radius: 50.0,
            },
            Task::batch([
                Task::perform(file::load_captures(), Message::CapturesLoaded),
                Task::perform(file::load_files(), Message::FilesLoaded),
            ]),
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
            Message::FileWritten(result) => {
                if let Ok(path) = result {
                    println!("Written to {}", path.display());
                    self.capture.before = "".to_string();
                    self.capture.after = "".to_string();
                    self.editor.editor_content = Content::with_text("");
                }

                Task::perform(file::load_captures(), Message::CapturesReloaded)
            }
            Message::SetInitialEditorText(result) => {
                let mut content_input = String::new();
                if let Ok((before, content, after)) = result {
                    self.capture.before = "".to_string();
                    self.capture.after = "".to_string();
                    self.editor.editor_content = Content::with_text("");
                    for (header, lines) in before {
                        self.capture
                            .before
                            .push_str(&format!("{}\n", header.trim()));
                        for line in lines {
                            self.capture.before.push_str(&format!("{}\n", line.trim()));
                        }
                    }
                    for (header, lines) in after {
                        self.capture.after.push_str(&format!("{}\n", header.trim()));
                        for line in lines {
                            self.capture.after.push_str(&format!("{}\n", line.trim()));
                        }
                    }
                    for line in content.1 {
                        content_input.push_str(&format!("{}\n", line.trim()));
                    }
                    self.editor.editor_content = Content::with_text(&content_input.trim());
                }
                Task::none()
            }
            Message::EditorContentChanged(action) => {
                self.editor.editor_content.perform(action);
                self.editor.is_saved = false;

                Task::none()
            }
            Message::CaptureSelected(index) => {
                if let Some(capture_data) = self.captures.get(index) {
                    let capture_input = capture_data.clone();
                    Task::perform(file::capture_opened(capture_input), Message::CaptureOpened)
                } else {
                    Task::none()
                }
            }
            Message::FileSelected(filename) => {
                if let Some(file) = self.files.iter().find(|f| f == &&filename) {
                    let file_input = file.clone();
                    Task::perform(file::file_opened(file_input), Message::EditorFileOpened)
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
                }
                Task::none()
            }
            Message::ShowModalHelper(result) => {
                if let Ok(value) = result {
                    println!("resulting {}", value);
                    self.modal_helper = true;
                }
                Task::none()
                // widget::focus_next()
            }
            Message::ShowHelper(result) => {
                if let Ok(value) = result {
                    println!("resulting {}", value);
                    self.show_helper = true;
                }
                Task::none()
                // widget::focus_next()
            }
            Message::HideModalHelper => {
                self.hide_modal_helper();
                Task::none()
            }
            Message::HideHelper => {
                self.hide_helper();
                Task::none()
            }
            Message::DeleteCapture(index) => {
                if let Some(capture_data) = self.captures.get(index) {
                    println!("Deleting line: {}", index);
                    let capture_input = capture_data.clone();
                    Task::perform(file::delete_capture(capture_input), Message::CaptureDeleted)
                } else {
                    Task::none()
                }
                // self.hide_subselect_capture();
            }
            Message::CaptureDeleted(result) => {
                if let Ok(_) = result {
                    println!("capture has been deleted!");
                    Task::perform(file::load_captures(), Message::CapturesLoaded)
                } else {
                    println!("capture has NOT been deleted!");
                    Task::none()
                }
            }
            Message::CapturesLoaded(result) => {
                if let Ok(value) = result {
                    self.captures = value;
                }
                Task::none()
            }
            Message::FilesLoaded(result) => {
                if let Ok(value) = result {
                    self.files = value;
                }
                Task::none()
            }
            Message::CapturesReloaded(result) => {
                if let Ok(value) = result {
                    self.captures = value;
                }

                let (mut panes, sidebar) = pane_grid::State::new(PaneState::CaptureSidebarPane);
                let _pane = panes.split(
                    pane_grid::Axis::Vertical,
                    sidebar,
                    PaneState::CaptureFormPane,
                );
                self.panes = panes;
                self.is_capture = true;
                self.editor.is_saved = true;
                Task::none()
            }
            Message::NewFileInput(value) => {
                self.editor.new_file = value;
                self.create_new_file_enabled();
                Task::none()
            }
            Message::CaptureSearchChanged(value) => {
                self.capture.search = value;
                Task::none()
            }
            Message::FileSearchChanged(value) => {
                println!("Searching file:: {}", value);
                Task::none()
            }
            Message::CaptureTopicChanged(value) => {
                self.capture.form_topic = value;
                self.update_submit_enabled();
                Task::none()
            }
            Message::CaptureSubjectChanged(value) => {
                self.capture.form_subject = value;
                self.update_submit_enabled();
                Task::none()
            }
            Message::CaptureFormContentChanged(action) => {
                self.capture.form_content.perform(action);
                self.update_submit_enabled();
                Task::none()
            }
            Message::SubselectCapture => {
                println!("SubselectCapture");
                self.is_subselect_capture = true;
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
                        file::append_file(form_topic, capture_string),
                        Message::FileOpened,
                    )
                }
            }
            Message::CreateNewFile => {
                println!("Creating {}", self.editor.new_file);
                Task::perform(
                    file::create_file(self.editor.new_file.clone()),
                    Message::CreatingNewFile,
                )
            }
            Message::CreatingNewFile(result) => {
                if let Ok(_) = result {
                    self.hide_helper();
                    Task::perform(file::load_files(), Message::FilesLoaded)
                } else {
                    println!("Failed CreatingNewFile");
                    Task::none()
                }
            }
            Message::UpdateCapture => {
                if self.editor.editor_content.text().is_empty() {
                    self.ui_error = "Submission failed: Inputs cannot be null.".to_string();
                    Task::perform(file::log(), Message::ShowError)
                } else {
                    let before_content = &self.capture.before;
                    let utc = Utc::now();
                    let local_timestamp = utc.with_timezone(&Local).to_string();
                    let timestamp = &local_timestamp[..19].to_string();
                    let file = self.capture.current_capture_file.clone();
                    let trimmed_file = &file[1..file.len() - 3];
                    let subject = self.capture.current_capture_subject.clone();
                    let editor_header = format!(
                        "<!--yoink::::{}::::{}::::{}-->\n",
                        timestamp, trimmed_file, subject
                    );
                    let editor_content = self.editor.editor_content.text();
                    let after_content = &self.capture.after;
                    let content = format!(
                        "{}{}{}\n{}",
                        before_content, editor_header, editor_content, after_content
                    );

                    Task::perform(file::write_file(file, content), Message::FileWritten)
                }
            }
            Message::UpdateFile => {
                if self.editor.editor_content.text().is_empty() {
                    self.ui_error = "Submission failed: Inputs cannot be null.".to_string();
                    Task::perform(file::log(), Message::ShowError)
                } else {
                    let before_content = &self.capture.before;
                    let utc = Utc::now();
                    let local_timestamp = utc.with_timezone(&Local).to_string();
                    let timestamp = &local_timestamp[..19].to_string();
                    let file = self.capture.current_capture_file.clone();
                    let trimmed_file = &file[1..file.len() - 3];
                    let subject = self.capture.current_capture_subject.clone();
                    let editor_header = format!(
                        "<!--yoink::::{}::::{}::::{}-->\n",
                        timestamp, trimmed_file, subject
                    );
                    let editor_content = self.editor.editor_content.text();
                    let after_content = &self.capture.after;
                    let content = format!(
                        "{}{}{}\n{}",
                        before_content, editor_header, editor_content, after_content
                    );

                    Task::perform(file::write_file(file, content), Message::FileWritten)
                }
            }
            Message::CreateFile => {
                println!("Creating file..");
                Task::perform(file::log(), Message::ShowHelper)
            }
            Message::ViewModalHelper => {
                println!("Creating file..");
                Task::perform(file::log(), Message::ShowModalHelper)
            }
            Message::DeleteFile(file_name) => {
                println!("Deleting {}..", file_name);
                Task::perform(file::delete_file(file_name), Message::FileDeleted)
            }
            Message::FileDeleted(result) => {
                if let Ok(_) = result {
                    println!("file has been deleted");
                    Task::perform(file::load_files(), Message::FilesLoaded)
                } else {
                    println!("file has NOT been deleted");
                    Task::none()
                }
            }
            Message::FileOpened(result) => {
                if let Ok(path) = result {
                    self.capture.updated_file = Some(path.to_string_lossy().to_string());
                    println!("Opened/Written to {}", path.display());
                }
                Task::perform(file::load_captures(), Message::CapturesLoaded)
            }
            Message::CaptureOpened(result) => {
                if let Ok((timestamp, path, subject)) = result {
                    self.capture.opened_capture = Some((timestamp, path, subject));

                    if let Some((ref timestamp_str, ref path_str, ref subject_str)) =
                        self.capture.opened_capture
                    {
                        self.capture.current_capture_timestamp = timestamp_str.to_string();
                        self.capture.current_capture_file = path_str.to_string_lossy().to_string();
                        self.capture.current_capture_subject = subject_str.to_string();
                        self.capture.current_capture = format!(
                            "{} {} {}",
                            self.capture.current_capture_timestamp,
                            self.capture.current_capture_file,
                            self.capture.current_capture_subject,
                        );
                    }
                }
                if self.capture.current_capture != "Editor..".to_string() {
                    let timestamp = self.capture.current_capture_timestamp.clone();
                    let subject = self.capture.current_capture_subject.clone();
                    Task::batch([
                        Task::perform(
                            async move { file::read_capture(&timestamp, "_test.md", &subject).await },
                            Message::SetInitialEditorText,
                        ),
                        Task::perform(async {}, |_| Message::Edit),
                    ])
                } else {
                    Task::none()
                }
            }
            Message::EditorFileOpened(result) => {
                if let Ok(lines) = result {
                    self.opened_file = lines.clone();
                    let mut editor_content: String = String::new();
                    for line in lines {
                        editor_content.push_str(&line);
                    }
                    self.editor.editor_content = Content::with_text(&editor_content);
                    //TODO: add filename and meta to editor header
                }
                Task::none()
            }
            Message::Ignore => Task::none(),
            Message::RadiusChanged(radius) => {
                self.radius = radius;
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
                    self.hide_helper();
                    Task::none()
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key: keyboard::Key::Named(key::Named::Enter),
                    ..
                }) => Task::perform(async {}, |_| Message::SubmitCapture),
                Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                    let now = Instant::now();
                    if now.duration_since(self.last_updated).as_millis() > 300 {
                        if let Some(result) = file::handle_hotkey(key, modifiers) {
                            Task::perform(async move { result }, |msg| msg)
                        } else {
                            Task::none()
                        }
                    } else {
                        Task::none()
                    }
                }
                _ => Task::none(),
            },
        }
    }

    fn view(&self) -> Element<Message> {
        let content = self.view_editor_pane();
        if self.modal_helper {
            let capture = self.view_capture_sidebar();
            let helper = container(capture);
            Yoink::modal(content, helper, Message::ViewModalHelper).into()
        } else {
            content.into()
        }
        // TODO: REVERT
        // let content = pane_grid::PaneGrid::new(&self.panes, |_pane, state, _is_maximized| {
        //     if self.is_capture {
        //         let content: Element<_> = match state {
        //             PaneState::CaptureSidebarPane => self.view_capture_sidebar(),
        //             PaneState::CaptureFormPane => self.view_capture_pane(),
        //             PaneState::EditorSidebarPane => self.view_editor_sidebar(),
        //             PaneState::EditorPane => self.view_editor_pane(),
        //         };
        //
        //         pane_grid::Content::new(content)
        //     } else {
        //         let content: Element<_> = match state {
        //             PaneState::CaptureSidebarPane => self.view_capture_sidebar(),
        //             PaneState::CaptureFormPane => self.view_capture_pane(),
        //             PaneState::EditorSidebarPane => self.view_editor_sidebar(),
        //             PaneState::EditorPane => self.view_editor_pane(),
        //         };
        //
        //         pane_grid::Content::new(content)
        //     }
        // })
        // .width(Length::Fill)
        // .height(Length::Fill)
        // .spacing(0)
        // .on_resize(10, Message::PaneResized);

        // TODO: REVERT
        // let base: Element<_> = if self.show_helper {
        //     let submit_button = self.view_newfile_submit_button();
        //     let helper = container(col![
        //         text("Helper"),
        //         text_input("File name..", &self.editor.new_file).on_input(Message::NewFileInput),
        //         submit_button
        //     ])
        //     .width(Length::Fill)
        //     .height(Length::Shrink)
        //     .center_x(Length::Shrink)
        //     .center_y(Length::Shrink)
        //     .padding(10)
        //     .style(|_theme| container::Style {
        //         text_color: Some(iced::Color::from_rgb8(15, 9, 9)),
        //         background: Some(iced::Background::Color(Color::from_rgb8(255, 182, 182))),
        //         border: iced::Border::default(),
        //         shadow: iced::Shadow::default(),
        //     });
        //
        //     Yoink::modal(content, helper, Message::HideHelper).into()
        // } else if self.is_subselect_capture {
        //     ContextMenu::new(content, || {
        //         container(col![button("Choice 1").width(400)].align_x(iced::Alignment::Center))
        //             .width(Length::Shrink)
        //             .height(Length::Shrink)
        //             .align_x(iced::Alignment::Center)
        //             .align_y(iced::Alignment::Center)
        //             .into()
        //     })
        //     .into()
        // TODO: REVERT
        // } else {
        //     content.into()
        // };

        // TODO: REVERT
        // base
    }
}
