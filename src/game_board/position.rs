use std::fmt::{Debug, Formatter};

#[derive(Eq, PartialEq, Clone, Hash)]
pub struct Position {
    pub vertical: u8,
    pub horizontal: u8,
}

impl Position {
    pub fn new(horizontal: u8, vertical: u8) -> Result<Self, String> {
        if horizontal > 7 || vertical > 7 {
            return Err("To large position passed!".to_string())
        }
        Ok(Self { vertical, horizontal })
    }
}

impl Debug for Position {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{:?}{:?}", self.horizontal, self.vertical))
    }
}

/// Used to build an position from certain data
pub struct PositionBuilder;

impl PositionBuilder {
    pub fn clone(position: &Position) -> Position {
        Position {
            vertical: position.vertical,
            horizontal: position.horizontal,
        }
    }
}

