#[derive(Debug)]
pub struct ChessPosition {
    column: i32,
    row: i32,
}

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        if rank >= 0 && rank < 8 && file >= 0 && file < 8 {
            return Some(ChessPosition {
                column: rank,
                row: file,
            });
        }

        None
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position: position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let row_diff = (self.position.row - other.position.row).abs();
        let col_diff = (self.position.column - other.position.column).abs();

        self.position.row == other.position.row ||           // same row
        self.position.column == other.position.column ||    // same column  
        row_diff == col_diff // diagonal
    }
}
