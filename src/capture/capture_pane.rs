#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapturePane {
    pub is_visible: bool,
}

impl CapturePane {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self { is_visible: true }
    }
}
