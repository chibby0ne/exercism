fn is_mine(grid: &[Vec<u8>], row: usize, col: usize) -> u32 {
    if let Some(row_content) = grid.get(row) {
        if let Some(&cell) = row_content.get(col) {
            if cell == b'*' {
                return 1;
            }
        }
    }
    0
}

fn get_mine_count(grid: &[Vec<u8>], row: usize, col: usize) -> String {
    let mut mines: u32 = 0;
    for row_d in -1..=1 {
        for col_d in -1..=1 {
            let actual_row = row as i32 + row_d;
            let actual_col = col as i32 + col_d;
            if row_d == 0 && row_d == col_d || actual_row < 0 || actual_col < 0 {
                continue;
            }
            mines += is_mine(grid, actual_row as usize, actual_col as usize);
        }
    }
    if mines == 0 {
        " ".to_string()
    } else {
        mines.to_string()
    }
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let field: Vec<Vec<u8>> = minefield.iter().map(|&x| x.as_bytes().to_vec()).collect();
    let mut result = Vec::new();
    for (row, line) in field.iter().enumerate() {
        let mut r = String::new();
        for (col, &cell) in line.iter().enumerate() {
            if cell == b' ' {
                r.push_str(&get_mine_count(&field, row, col));
            } else {
                r.push(cell as char);
            }
        }
        result.push(r);
    }
    result
}
