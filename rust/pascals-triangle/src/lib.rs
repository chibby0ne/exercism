pub struct PascalsTriangle {
    count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut vec: Vec<Vec<u32>> = Vec::with_capacity(self.count as usize);
        for i in 0..self.count {
            let mut inner_vec: Vec<u32> = Vec::with_capacity(i as usize + 1);
            for j in 0..=i {
                if j == 0 || j == inner_vec.capacity() as u32 {
                    inner_vec.push(1)
                } else {
                    let prev_vec = vec.get(i as usize - 1).unwrap();
                    if j as usize == inner_vec.capacity() - 1 {
                        inner_vec.push(prev_vec[j as usize - 1]);
                    } else {
                        inner_vec.push(prev_vec[j as usize - 1] + prev_vec[j as usize]);
                    }
                }
            }
            vec.push(inner_vec);
        }
        vec
    }
}
