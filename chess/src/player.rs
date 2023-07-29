use crate::color::Color;

#[derive(Clone, Copy, Default, PartialEq)]
pub enum PlayerKind {
    #[default]
    Local,
    Remote,
    Bot,
}

#[derive(Default, PartialEq, Clone)]
pub struct Player {
    pub kind: PlayerKind,
    pub color: Color,
    pub name: String,
}

impl Player {
    pub fn with_color(color: Color) -> Self {
        Self {
            color,
            ..Self::default()
        }
    }
}
