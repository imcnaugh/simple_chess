#[derive(Debug)]
pub enum GameState {
    InProgress,
    Check,
    Checkmate,
    Stalemate,
    InsufficientMaterial,
    FiftyMoveRule,
}
