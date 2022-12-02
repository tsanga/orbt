use async_graphql::{Enum, SimpleObject};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, SimpleObject, Clone)]
pub struct Color {
    pub name: ColorType,
    pub hex: String,
}

impl From<Color> for ColorType {
    fn from(c: Color) -> Self {
        c.name
    }
}

impl From<ColorType> for Color {
    fn from(t: ColorType) -> Self {
        Self {
            name: t,
            hex: t.to_hex().to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Enum)]
pub enum ColorType {
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

impl ColorType {
    pub fn to_hex(&self) -> &str {
        match self {
            Self::Red => "#FF7676",
            Self::Orange => "#FFC876",
            Self::Yellow => "#FFF176",
            Self::Lime => "#C3FF76",
            Self::Green => "#76FF84",
            Self::Cyan => "#76FFCE",
            Self::Blue => "#76CEFF",
            Self::Indigo => "#7976FF",
            Self::Purple => "#B276FF",
            Self::Pink => "#FF7651",
        }
    }

    pub fn to_name(&self) -> &str {
        match self {
            Self::Red => "red",
            Self::Orange => "orange",
            Self::Yellow => "yellow",
            Self::Lime => "lime",
            Self::Green => "green",
            Self::Cyan => "cyan",
            Self::Blue => "blue",
            Self::Indigo => "indigo",
            Self::Purple => "purple",
            Self::Pink => "pink",
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            Self::Red,
            Self::Orange,
            Self::Yellow,
            Self::Lime,
            Self::Green,
            Self::Cyan,
            Self::Blue,
            Self::Indigo,
            Self::Purple,
            Self::Pink,
        ]
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn has_minimum_colors() {
        assert!(super::ColorType::all().len() >= 8);
    }
}
