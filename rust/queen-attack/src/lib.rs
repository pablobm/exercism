use std::ops::Sub;

#[derive(Copy)]
#[derive(Clone)]
pub struct ChessPosition {
    rank: i8,
    file: i8,
}

pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i8, file: i8)
        -> Option<ChessPosition>
    {
        match (rank, file) {
            (0..=7, 0..=7)
                => Some(ChessPosition{ rank: rank, file: file }),
            _
                => None,
        }
    }
}

impl Sub for ChessPosition {
    type Output = ChessPosition;
    fn sub(self, rhs: ChessPosition) -> Self::Output {
        ChessPosition {
            rank: self.rank - rhs.rank,
            file: self.file - rhs.file,
        }
    }
}

impl Queen {
    pub fn new(pos: ChessPosition) -> Queen {
        Queen {
            pos: pos,
        }
    }

    pub fn can_attack(&self, piece: &Queen) -> bool {
        let diff = piece.pos - self.pos;
        diff.rank == 0 || diff.file == 0 ||
            diff.rank.abs() == diff.file.abs()
    }
}
