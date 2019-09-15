#[derive(Debug, PartialEq)]
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
            (0..=7, 0..=7) => Some(ChessPosition {
                rank,
                file,
            }),
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
        let mut rank = self.rank;
        let mut file = self.file;

        // going south east
        while rank < 8 && file < 8 {
            rank += 1;
            file += 1;
            let temp_pos = &ChessPosition { rank, file };
            if temp_pos == other {
                return true;
            }
        }

        // going north east
        rank = self.rank;
        file = self.file;
        while rank >= 0 && file < 8 {
            rank -= 1;
            file += 1;
            let temp_pos = &ChessPosition { rank, file };
            if temp_pos == other {
                return true;
            }
        }

        // going north west
        rank = self.rank;
        file = self.file;
        while rank >= 0 && file >= 0 {
            rank -= 1;
            file -= 1;
            let temp_pos = &ChessPosition { rank, file };
            if temp_pos == other {
                return true;
            }
        }

        // going south west
        rank = self.rank;
        file = self.file;
        while rank < 8 && file >= 0 {
            rank += 1;
            file -= 1;
            let temp_pos = &ChessPosition { rank, file };
            if temp_pos == other {
                return true;
            }
        }
        false
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
