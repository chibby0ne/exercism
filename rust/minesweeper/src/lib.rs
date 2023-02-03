fn is_mine(grid: &[&str], row: usize, col: usize) -> u32 {
    if let Some(row_content) = grid.get(row) {
        if let Some(cell) = row_content.chars().nth(col) {
            if cell == '*' {
                return 1;
            }
        }
    }
    0
}

fn get_mine_count(grid: &[&str], row: usize, col: usize) -> u32 {
    (-1..=1)
        .flat_map(|r| (-1..=1).map(move |c| (r, c)))
        .filter(|&(r, c)| !(r == 0 && c == 0))
        .map(|(r, c)| (row as i32 + r, col as i32 + c))
        .fold(0, |mut acc, (r, c)| {
            acc += is_mine(grid, r as usize, c as usize);
            acc
        })
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    minefield
        .iter()
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .fold(String::new(), |mut acc, (col, cell)| {
                    let mut to_push = cell.to_string();
                    if cell == ' ' {
                        let mines = get_mine_count(minefield, row, col);
                        if mines > 0 {
                            to_push = mines.to_string();
                        }
                    }
                    acc.push_str(&to_push);
                    acc
                })
        })
        .collect()
}
