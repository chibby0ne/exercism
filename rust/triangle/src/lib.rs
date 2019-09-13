pub struct Triangle {
    sides: [u64; 3],
}

impl Triangle {
    pub fn build(sides: [u64; 3]) -> Option<Triangle> {
        if sides.iter().any(|&s| s == 0)
            // compare a + b with c
            || sides.iter().take(2).sum::<u64>() < *sides.iter().last().unwrap()
            // compare b + c with a
            || sides.iter().skip(1).sum::<u64>() < *sides.iter().nth(0).unwrap()
        {
            return None;
        }
        Some(Triangle { sides })
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[1..]
            .iter()
            .all(|s| s == self.sides.iter().nth(0).unwrap())
    }

    pub fn is_scalene(&self) -> bool {
        self.sides[1..]
            .iter()
            .all(|s| s != self.sides.iter().nth(0).unwrap())
            && self.sides[1] != self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        self.sides[1..]
            .iter()
            .any(|s| s == self.sides.iter().nth(0).unwrap())
            || self.sides[1] == self.sides[2]
    }

    pub fn is_degenerate(&self) -> bool {
        self.sides.iter().take(2).sum::<u64>() == *self.sides.iter().last().unwrap()
            || self.sides.iter().skip(1).sum::<u64>() == *self.sides.iter().nth(0).unwrap()
    }
}
