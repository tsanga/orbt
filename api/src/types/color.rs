use async_graphql::Enum;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Enum)]
pub enum Color {
    Red,
    Orange,
    Yellow,
    Lime,
    Green,
    Cyan,
    Blue,
    Indigo,
    Purple,
    Pink,
}

impl Color {
    pub fn to_hex(&self) -> &str {
        match self {
            Color::Red => "#FF7676",
            Color::Orange => "#FFC876",
            Color::Yellow => "#FFF176",
            Color::Lime => "#C3FF76",
            Color::Green => "#76FF84",
            Color::Cyan => "#76FFCE",
            Color::Blue => "#76CEFF",
            Color::Indigo => "#7976FF",
            Color::Purple => "#B276FF",
            Color::Pink => "#FF7651",
        }
    }

    pub fn all() -> Vec<Color> {
        vec![
            Color::Red, 
            Color::Orange, 
            Color::Yellow, 
            Color::Lime, 
            Color::Green, 
            Color::Cyan, 
            Color::Blue, 
            Color::Indigo, 
            Color::Purple, 
            Color::Pink,
        ]
    }
}
