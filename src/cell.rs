#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum CellSymb {
    X,
    O,
    Empty,
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Cell {
    symbol: CellSymb,
    pub value: u64,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: CellSymb::Empty,
            value: 1,
        }
    }
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        self.symbol == CellSymb::Empty
    }

    pub fn draw(&self) -> String {
        match self.symbol {
            CellSymb::Empty => String::from("."),
            CellSymb::O => String::from("●"),
            CellSymb::X => String::from("x"),
        }
    }

    pub fn set_to_circle(&mut self) {
        self.symbol = CellSymb::O;
    }

    pub fn set_to_x(&mut self) {
        self.symbol = CellSymb::X;
    }
}

