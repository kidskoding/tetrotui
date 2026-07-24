use std::time::Duration;

pub const WIDTH: usize = 10;
pub const HIDDEN_HEIGHT: usize = 20;
pub const VISIBLE_HEIGHT: usize = 20;
pub const HEIGHT: usize = HIDDEN_HEIGHT + VISIBLE_HEIGHT;

pub struct Board {
    pub cells: Vec<Vec<Option<PieceKind>>>
}

pub struct GameState {
    pub board: Board,
    pub active: ActivePiece,
    pub game_over: bool,
    pub drop_timer: Duration,
}

// the seven variants of the tetronimo
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PieceKind {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

pub struct ActivePiece {
    pub kind: PieceKind,
    pub rotation: u8,
    pub origin: (i32, i32),
}
