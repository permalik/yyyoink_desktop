use iced::widget::{container, text};
use iced::{Element, Sandbox, Settings};

#[derive(Default)]
struct Editor;

#[derive(Debug, Clone, Copy)]
enum Message {}

impl Sandbox for Editor {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Custom Editor")
    }

    fn update(&mut self, message: Message) {}

    fn view(&self) -> Element<Message> {
        let hello = text("Hello Editor.");
        container(hello).into()
    }
}

pub fn main() -> iced::Result {
    Editor::run(Settings::default())
}
