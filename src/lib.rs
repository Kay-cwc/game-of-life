mod state;
mod utils;

use wasm_bindgen::prelude::*;

use state::Cell;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    // a one-dimension vec that stored a flatterned grid (i.e. |..row1..|..r2..|..r3..| )
    cells: Vec<Cell>
}

impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        Universe {
            width, height,
            cells: vec![Cell::Dead; (width * height) as usize],
        }
    }

    pub fn init_cells(&mut self, initial_cells: Vec<[u32; 2]>) {
        let mut cells = self.cells.clone();
        for [row, col] in initial_cells {
            let idx = self.get_index(row, col);
            cells[idx] = Cell::Alive;
        }

        self.cells = cells;
    }

    pub fn cells_to_arr(&self) -> Vec<u8> {
        self.cells.clone().into_iter().map(|v| v as u8).collect()
    }

    pub fn living_neightbour_count(&self, row: u32, col: u32) -> u8 {
        let deltas = [-1, 0, 1];
        let mut counts = 0u8;
        for row_delta in deltas {
            let r = (row as i32 + row_delta) as u32 % self.height;
            for col_delta in deltas {
                let c = (col as i32 + col_delta) as u32 % self.width;
                if r == 0 && c == 0 {
                    continue;
                }
                let idx = self.get_index(r, c);
                counts += self.cells[idx] as u8;
            }
        }

        counts
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (self.width * row + col) as usize
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, wasm-game-of-life!");
}

#[cfg(test)]
mod tests {
    use crate::Universe;

    
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
        // 0100
        // 0010
        // 1000
        // 0000
        let initial_cells = [[1,2], [2,0]];

        universe.init_cells(initial_cells.to_vec());
        assert_eq!(universe.living_neightbour_count(0, 0), 0);
        assert_eq!(universe.living_neightbour_count(1, 0), 1);
        assert_eq!(universe.living_neightbour_count(1, 1), 2);
        assert_eq!(universe.living_neightbour_count(2, 1), 2);
        assert_eq!(universe.living_neightbour_count(3, 2), 0);
    }
}
