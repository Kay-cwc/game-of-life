
use std::fmt;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl fmt::Display for Cell {
    /** display the cell in a human readable format */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let readable_display = match self {
            Cell::Alive => '◼',
            Cell::Dead => '◻'
        };
        write!(f, "{}", readable_display)
    }
}

#[cfg(test)]
mod test {
    use crate::cells::Cell;

    #[test]
    fn test_display() {
        assert_eq!(Cell::Alive.to_string(), '◼'.to_string());
        assert_eq!(Cell::Dead.to_string(), '◻'.to_string());
    }
}