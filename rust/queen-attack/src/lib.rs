pub struct ChessPosition {
    rank: i8,
    file: i8,
}

pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i8, file: i8)
        -> Result<ChessPosition, &'static str>
    {
        match (rank, file) {
            (0...7, 0...7)
                => Ok(ChessPosition{ rank: rank, file: file }),
            _
                => Err("Invalid Position"),
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
        let rdiff = (piece.pos.rank - self.pos.rank).abs();
        let fdiff = (piece.pos.file - self.pos.file).abs();

        rdiff == 0 || fdiff == 0 || rdiff == fdiff
    }
}
