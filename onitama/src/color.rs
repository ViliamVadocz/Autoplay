#[derive(Copy, Clone, Debug)]
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
}
