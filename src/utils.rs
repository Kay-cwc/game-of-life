pub fn set_panic_hook() {
    // When the `console_error_panic_hook` feature is enabled, we can call the
    // `set_panic_hook` function at least once during initialization, and then
    // we will get better error messages if our code ever panics.
    //
    // For more details see
    // https://github.com/rustwasm/console_error_panic_hook#readme
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

use super::state::Cell;

/**
 * Hades, the god of the underworld. 
 * Only he can determine one to be alive or dead
 *
 * the law of hades
 * 1. Any live cell with fewer than two live neighbours dies, as if caused by underpopulation.
 * 2. Any live cell with two or three live neighbours lives on to the next generation.
 * 3. Any live cell with more than three live neighbours dies, as if by overpopulation.
 * 4. Any dead cell with exactly three live neighbours becomes a live cell, as if by reproduction.
 */
pub fn hades(is_alive: Cell, living_neightbours: u8) -> Cell {
    match (is_alive, living_neightbours) {
        (Cell::Alive, x) if x < 2 => Cell::Dead,
        (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
        (Cell::Alive, x) if x > 3 => Cell::Dead,
        (Cell::Dead, x) if x == 3 => Cell::Alive,
        (otherwise, _) => otherwise
    }
}

#[cfg(test)]
mod hades_test {
    use super::hades;
    use super::super::state::Cell;
    #[test]
    fn test_hades() {
        assert_eq!(hades(Cell::Alive, 1), Cell::Dead);
        assert_eq!(hades(Cell::Alive, 2), Cell::Alive);
        assert_eq!(hades(Cell::Alive, 3), Cell::Alive);
        assert_eq!(hades(Cell::Alive, 4), Cell::Dead);
        assert_eq!(hades(Cell::Alive, 5), Cell::Dead);
        assert_eq!(hades(Cell::Alive, 6), Cell::Dead);
        assert_eq!(hades(Cell::Alive, 7), Cell::Dead);
        assert_eq!(hades(Cell::Alive, 8), Cell::Dead);

        assert_eq!(hades(Cell::Dead, 1), Cell::Dead);
        assert_eq!(hades(Cell::Dead, 2), Cell::Dead);
        assert_eq!(hades(Cell::Dead, 3), Cell::Alive);
        assert_eq!(hades(Cell::Dead, 4), Cell::Dead);
        assert_eq!(hades(Cell::Dead, 5), Cell::Dead);
        assert_eq!(hades(Cell::Dead, 6), Cell::Dead);
        assert_eq!(hades(Cell::Dead, 7), Cell::Dead);
        assert_eq!(hades(Cell::Dead, 8), Cell::Dead);
    }
}