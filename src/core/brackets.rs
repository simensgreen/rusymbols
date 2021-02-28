use crate::core::literals::*;

#[derive(Debug, Copy, Clone)]
pub enum Brackets {
    Round,
    Figure,
    Square,
}

impl Brackets {
    pub fn get_symbols(&self) -> (&str, &str) {
        match self {
            Brackets::Round => BRACKET_ROUND,
            Brackets::Figure => BRACKET_FIGURE,
            Brackets::Square => BRACKET_SQUARE,
        }
    }
}
