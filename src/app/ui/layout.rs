use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Copy, Clone, Deserialize, Serialize)]
pub enum HarmonyLayout {
    // [ ][ ]
    // [ ][ ]
    Square,
    // [  ]
    // [  ]
    // [  ]
    // [  ]
    Stacked,
    // ________
    // ||||||||
    // ||||||||
    // --------
    Line,
    // Gradient #TODO
}

impl Default for HarmonyLayout {
    fn default() -> Self {
        HarmonyLayout::Square
    }
}

impl AsRef<str> for HarmonyLayout {
    fn as_ref(&self) -> &str {
        match self {
            Self::Square => "square",
            Self::Stacked => "stacked",
            Self::Line => "line",
        }
    }
}
