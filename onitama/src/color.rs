#[derive(Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    pub fn next(self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }
}
