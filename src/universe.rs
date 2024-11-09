use std::fmt;

use wasm_bindgen::prelude::*;

use crate::{cells::Cell, utils::{hades, set_panic_hook}};

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    // a one-dimension vec that stored a flatterned grid (i.e. |..row1..|..r2..|..r3..| )
    cells: Vec<Cell>
}

impl Universe {
    pub fn init_cells(&mut self, initial_cells: Vec<[u32; 2]>) {
        let mut cells = self.cells.clone();
        for [row, col] in initial_cells {
            let idx = self.to_index(row, col);
            cells[idx] = Cell::Alive;
        }

        self.cells = cells;
    }

    pub fn next_epoch(&mut self) {
        let cells = self.cells.clone();
        let mut next_cells = self.cells.clone();
        for (index, cell) in cells.into_iter().enumerate() {
            let (row, col) = self.from_index(index);
            let living_neightbour = self.living_neightbour_count(row, col);
            let next_epoch_state = hades(cell, living_neightbour);
            next_cells[index] = next_epoch_state;
        }

        self.cells = next_cells;
    }

    pub fn cells_to_arr(&self) -> Vec<u8> {
        self.cells.clone().into_iter().map(|v| v as u8).collect()
    }

    fn living_neightbour_count(&self, row: u32, col: u32) -> u8 {
        let mut counts = 0u8;
        for row_delta in [self.height - 1, self.height, self.height + 1] {
            let r = (row + row_delta) as u32 % self.height;
            for col_delta in [self.width - 1, self.width, self.width + 1] {
                let c = (col + col_delta) as u32 % self.width;
                if r == row && c == col { continue }
                let idx = self.to_index(r, c);
                println!("[{}, {}] {}", r, c, self.cells[idx]);
                counts += self.cells[idx] as u8;
            }
        }

        counts
    }

    fn to_index(&self, row: u32, col: u32) -> usize {
        (self.width * row + col) as usize
    }

    fn from_index(&self, index: usize) -> (u32, u32) {
        let row = index / self.width as usize;
        let col = index % self.width as usize;

        (row as u32, col as u32)
    }
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width, height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    /**
     * provide a binding for js array.
     * calls `init_cell` before validate the data can be seraialised into Vec[u32; 2]
     */
    pub fn init_single_cell(&mut self, row: u32, col: u32) {
        self.init_cells(vec![[row, col]]);
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        self.next_epoch();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn width(&self) -> u32 {
        self.width
    }
}

impl fmt::Display for Universe { 
    /**
     * convert the one dimension cells vectors into a human-readable string
     * which display ◼ for alive and ◻ for dead in a multi-line format
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        set_panic_hook();

        let chunks = self.cells.chunks(self.width as usize).into_iter().enumerate();
        for (idx, chunk) in chunks {
            for cell in chunk {
                write!(f, "{}", cell.to_string())?;
            }
            if idx < chunk.len() - 1 { write!(f, "\n")? };
        }
        Ok(())
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[cfg(test)]
mod tests {
    use crate::{cells::Cell, universe::Universe};

    #[test]
    fn test_from_index() {
        let universe = Universe::new(3,3);
        assert_eq!(universe.to_index(2, 2), 8);
        assert_eq!(universe.to_index(1, 0), 3);
    }

    #[test]
    fn test_to_index() {
        let universe = Universe::new(3,3);
        assert_eq!(universe.from_index(8), (2, 2));
        assert_eq!(universe.from_index(3), (1, 0));
        assert_eq!(universe.from_index(0), (0, 0));
    }

    #[test]
    fn test_initializer() {
        let mut universe = Universe::new(2,2);
        let mut cells = universe.cells_to_arr();
        assert_eq!(cells.len(), 4);
        
        let initial_cells = [[0,0], [1,1]].to_vec();
        universe.init_cells(initial_cells);
        cells = universe.cells_to_arr();
        assert_eq!(cells, [1,0,0,1]);
    }

    #[test]
    fn test_living_neightbour_count() {
        let mut universe = Universe::new(4,4);
        // expected cells
        // 0000
        // 0010
        // 1000
        // 0000
        let initial_cells = [[1,2], [2,0]];

        universe.init_cells(initial_cells.to_vec());
        assert_eq!(universe.living_neightbour_count(0, 0), 0);
        assert_eq!(universe.living_neightbour_count(1, 2), 0);
        assert_eq!(universe.living_neightbour_count(1, 0), 1);
        assert_eq!(universe.living_neightbour_count(1, 1), 2);
        assert_eq!(universe.living_neightbour_count(2, 1), 2);
        assert_eq!(universe.living_neightbour_count(3, 2), 0);
    }

    #[test]
    fn test_display() {
        let mut universe = Universe::new(2,2);
        assert_eq!(universe.to_string(), format!(
            "{}{}\n{}{}", 
            Cell::Dead.to_string(),
            Cell::Dead.to_string(),
            Cell::Dead.to_string(),
            Cell::Dead.to_string(),
        ));


        let initial_cells = [[0,0], [1,1]];
        universe.init_cells(initial_cells.to_vec());

        let expected_output = format!(
            "{}{}\n{}{}", 
            Cell::Alive.to_string(),
            Cell::Dead.to_string(),
            Cell::Dead.to_string(),
            Cell::Alive.to_string(),
        );
        assert_eq!(universe.to_string(), expected_output);
    }

    #[test]
    fn test_next_epoch() {
        let mut universe = Universe::new(10,10);
        let initial_cells = [
            [3,4],
            [4,5],
            [5,3],
            [5,4],
            [5,5],
        ].to_vec();
        universe.init_cells(initial_cells.clone());

        universe.next_epoch();
 
        let cells = universe.cells.clone();

        let next_living_cells = [
            [4,3],
            [4,5],
            [5,4],
            [5,5],
            [6,4],
        ];
        let next_living_cell_indexs = next_living_cells.map(|cell| universe.to_index(cell[0], cell[1]));

        for (index, cell) in cells.into_iter().enumerate() {
            if next_living_cell_indexs.contains(&index) {
                assert_eq!(cell, Cell::Alive);
            } else {
                assert_eq!(cell, Cell::Dead);
            }
        }
    }
}
