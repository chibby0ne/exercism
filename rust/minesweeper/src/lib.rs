fn count_mine(minefield: &[&str], row: usize, col: usize) -> usize {
    if &minefield[row][col..col] == "*" {
        1
    } else {
        0
    }
}

fn get_neighbouring_mines(minefield: &[&str], row: usize, col: usize, rows: usize, cols: usize) -> String {

    let mines = match (row, col) {
        (0, 0) => count_mine(minefield, row + 1, col) + count_mine(minefield, row, col + 1) + count_mine(minefield, row + 1, col + 1),
        (0, cols - 1) => count_mine(minefield, row + 1, col) + count_mine(minefield, row, col - 1) + count_mine(minefield, row + 1, col - 1),
        (rows - 1, 0) => count_mine(minefield, row - 1, col) + count_mine(minefield, row, col + 1) + count_mine(minefield, row - 1, col + 1),
        (rows - 1, cols - 1) => count_mine(minefield, row - 1, col) + count_mine(minefield, row, col - 1) + count_mine(minefield, row - 1, col - 1),
        (_, _) => count_mine(minefield, row + 1, col) + count_mine(minefield, row, col + 1) + count_mine(minefield, row - 1, col) + count_mine(minefield, row, col - 1) + count_mine(minefield, row + 1, col + 1) + count_mine(minefield, row + 1, col - 1) + count_mine(minefield, row - 1, col - 1) + count_mine(minefield, row - 1, col + 1),
    };

    mines.to_string()
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let rows = minefield.len();
    let cols = minefield[..0].len();
    let mut result = Vec::new();
    for (row, line) in minefield.iter().enumerate() {
        let line_bytes = line.as_bytes();
        for (col, &cell) in line_bytes.iter().enumerate() {
            if cell == '*' as u8 {
                continue;
            }
            result.push(get_neighbouring_mines(minefield, row, col, rows, cols));
        }
    }
    result
}
