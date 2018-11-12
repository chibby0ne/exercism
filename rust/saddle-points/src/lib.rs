pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut vec: Vec<(usize, usize)> = Vec::new();
    let rows = input.len();

    // Early termination condition
    if rows == 0 {
        return vec;
    }

    let cols = input[0].len();
    for i in 0..rows {
        for j in 0..cols {
            if is_saddle_point(input, rows, cols, i, j) {
                vec.push((i, j));
            }
        }
    }
    return vec;
}

fn is_saddle_point(v: &[Vec<u64>], rows: usize, cols: usize, x: usize, y: usize) -> bool {
    // check if it's the larger or equal is its row
    for j in 0..cols {
        if v[x][y] < v[x][j] {
            return false;
        }
    }
    // check if it's the smaller or equal is its col
    for i in 0..rows {
        if v[x][y] > v[i][y] {
            return false;
        }
    }
    return true;
}
