#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CaptureSidebar {
    pub is_visible: bool,
}

impl CaptureSidebar {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { is_visible: true }
    }
}
