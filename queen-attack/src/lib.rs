#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen(ChessPosition);

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        match (rank, file){
            (0..8, 0..8) => Some(ChessPosition(rank, file))
            _ => None
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen(position)
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let Queen(ChessPosition(r1, f1)) = *self;
        let Queen(ChessPosition(r2, f2)) = *other;

        (r1 - r2).abs() == (f1 - f2).abs() || (((r1 - r2) == 0) ^ ((f1 - f2) == 0))
    }
}
