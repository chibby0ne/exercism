#[derive(Debug)]
pub struct ChessPosition {
    rank: i32,
    file: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file) {
            (0..=7, 0..=7) => Some(ChessPosition { rank, file }),
            _ => None,
        }
    }

    pub fn in_same_rank(&self, other: &ChessPosition) -> bool {
        self.rank == other.rank
    }

    pub fn in_same_file(&self, other: &ChessPosition) -> bool {
        self.file == other.file
    }

    pub fn in_same_diagonal(&self, other: &ChessPosition) -> bool {
        (self.file - other.file).abs() == (self.rank - other.rank).abs()
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        self.position.in_same_file(&other.position)
            || self.position.in_same_rank(&other.position)
            || self.position.in_same_diagonal(&other.position)
    }
}
