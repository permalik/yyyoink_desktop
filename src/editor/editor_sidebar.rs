#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorSidebar {
    pub is_visible: bool,
}

impl EditorSidebar {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { is_visible: true }
    }
}
