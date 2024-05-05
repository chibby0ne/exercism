pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let mut res : Vec<Vec<u32>> = Vec::new();
    if size == 0 {
        return res;
    }
    for _ in 0..size {
        res.push(vec![0; size as usize]);
    }
    
    // Do the upper row
    for j in 0..size as usize {
        res[0][j] = j as u32;
    }
    // Do the right side column
    for i in 0..size as usize {
        res[i][size as usize - 1 as usize] = i as u32 + size;
    }
    // Do the lower row
    for j in size as usize - 1..=0 {
        res[0][j] = j as u32 * size;
    }
    // Do the left size column
    for i in size as usize -1..=0 as usize {
        res[i][0] = i as u32 + size * 2;
    }


    res
}
