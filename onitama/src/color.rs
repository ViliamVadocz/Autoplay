use std::result::Result;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    Red,
    Blue,
}

impl Color {
    pub fn next(self) -> Self {
        match self {
            Color::Red => Color::Blue,
            Color::Blue => Color::Red,
        }
    }

    pub fn from(color: String) -> Result<Color, String> {
        match color.as_ref() {
            "red" => Ok(Color::Red),
            "blue" => Ok(Color::Blue),
            _ => Err(format!("Unknown color: {}", color)),
        }
    }
}
