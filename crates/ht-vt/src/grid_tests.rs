#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(24, 80, 1000);
        let (cols, rows) = grid.size();

        assert_eq!(rows, 24);
        assert_eq!(cols, 80);
    }

    #[test]
    fn test_put_char() {
        let mut grid = Grid::new(24, 80, 1000);

        grid.put_char('H');
        grid.put_char('i');

        let cell1 = grid.get_cell(0, 0).unwrap();
        let cell2 = grid.get_cell(1, 0).unwrap();

        assert_eq!(cell1.ch, 'H');
        assert_eq!(cell2.ch, 'i');
    }

    #[test]
    fn test_carriage_return() {
        let mut grid = Grid::new(24, 80, 1000);

        grid.put_char('A');
        grid.put_char('B');
        assert_eq!(grid.cursor_pos(), (2, 0));

        grid.carriage_return();
        assert_eq!(grid.cursor_pos(), (0, 0));
    }

    #[test]
    fn test_line_feed() {
        let mut grid = Grid::new(24, 80, 1000);

        grid.line_feed();
        assert_eq!(grid.cursor_pos(), (0, 1));

        grid.line_feed();
        assert_eq!(grid.cursor_pos(), (0, 2));
    }

    #[test]
    fn test_scroll_up() {
        let mut grid = Grid::new(3, 10, 1000);

        // Fill first line
        for _ in 0..5 {
            grid.put_char('A');
        }

        grid.carriage_return();
        grid.line_feed();

        // Fill second line
        for _ in 0..5 {
            grid.put_char('B');
        }

        grid.carriage_return();
        grid.line_feed();

        // Fill third line
        for _ in 0..5 {
            grid.put_char('C');
        }

        // Force scroll
        grid.line_feed();

        // First line should now be 'B'
        let cell = grid.get_cell(0, 0).unwrap();
        assert_eq!(cell.ch, 'B');

        // Second line should be 'C'
        let cell = grid.get_cell(0, 1).unwrap();
        assert_eq!(cell.ch, 'C');
    }

    #[test]
    fn test_cursor_move() {
        let mut grid = Grid::new(24, 80, 1000);

        grid.move_cursor(10, 5);
        assert_eq!(grid.cursor_pos(), (10, 5));

        grid.move_cursor_relative(5, 3);
        assert_eq!(grid.cursor_pos(), (15, 8));

        grid.move_cursor_relative(-5, -2);
        assert_eq!(grid.cursor_pos(), (10, 6));
    }

    #[test]
    fn test_clear_screen() {
        let mut grid = Grid::new(24, 80, 1000);

        // Fill some cells
        for _ in 0..10 {
            grid.put_char('X');
        }

        grid.clear_screen();

        // All cells should be empty
        for row in 0..24 {
            for col in 0..80 {
                let cell = grid.get_cell(col, row).unwrap();
                assert_eq!(cell.ch, ' ');
            }
        }
    }

    #[test]
    fn test_attributes() {
        let mut grid = Grid::new(24, 80, 1000);

        let mut attrs = CellAttributes::default();
        attrs.foreground = Color::Rgb(255, 0, 0);
        attrs.bold = true;

        grid.set_attributes(attrs);
        grid.put_char('A');

        let cell = grid.get_cell(0, 0).unwrap();
        assert_eq!(cell.ch, 'A');
        assert_eq!(cell.attrs.foreground, Color::Rgb(255, 0, 0));
        assert!(cell.attrs.bold);
    }

    #[test]
    fn test_resize() {
        let mut grid = Grid::new(24, 80, 1000);

        grid.put_char('A');
        grid.resize(30, 100);

        let (cols, rows) = grid.size();
        assert_eq!(rows, 30);
        assert_eq!(cols, 100);
    }

    #[test]
    fn test_tab() {
        let mut grid = Grid::new(24, 80, 1000);

        grid.put_char('A');
        grid.tab();

        // Should move to next tab stop (multiple of 8)
        let (x, _) = grid.cursor_pos();
        assert_eq!(x, 8);
    }

    #[test]
    fn test_backspace() {
        let mut grid = Grid::new(24, 80, 1000);

        grid.put_char('A');
        grid.put_char('B');

        assert_eq!(grid.cursor_pos(), (2, 0));

        grid.backspace();
        assert_eq!(grid.cursor_pos(), (1, 0));
    }
}
