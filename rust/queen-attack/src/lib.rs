pub struct ChessPosition {
    x: i16,
    y: i16,
}

pub struct Queen {
    pos: ChessPosition,
}

impl ChessPosition {
    pub fn new(x: i16, y: i16) -> Result<ChessPosition, &'static str> {
        match (x, y) {
            (0...7, 0...7) => Ok(ChessPosition{ x: x, y: y }),
            _ => Err("Invalid Position"),
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
        let xdiff = (piece.pos.x - self.pos.x).abs();
        let ydiff = (piece.pos.y - self.pos.y).abs();

        xdiff == 0 || ydiff == 0 || xdiff == ydiff
    }
}
