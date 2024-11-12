use crate::Color;

pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
    InsufficientMaterial,
    FiftyMoveRule,
}
