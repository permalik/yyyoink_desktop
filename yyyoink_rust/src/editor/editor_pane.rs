#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct EditorPane {
    pub is_visible: bool,
}

impl EditorPane {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { is_visible: true }
    }
}
