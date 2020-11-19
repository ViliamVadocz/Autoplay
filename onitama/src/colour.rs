use std::result::Result;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Colour {
    Red,
    Blue,
}

impl Colour {
    pub fn next(self) -> Self {
        match self {
            Colour::Red => Colour::Blue,
            Colour::Blue => Colour::Red,
        }
    }

    pub fn from(colour: String) -> Result<Colour, String> {
        match colour.as_ref() {
            "red" => Ok(Colour::Red),
            "blue" => Ok(Colour::Blue),
            _ => Err(format!("Unknown colour: {}", colour)),
        }
    }
}
