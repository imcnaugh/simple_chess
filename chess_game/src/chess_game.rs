use game_board::Board;
use crate::Color;

pub struct ChessGame {
    board: Board,
    current_players_turn: Color,
    turn_number: usize,
    fifty_move_rule_counter: usize,
    can_white_castle_short: bool,
    can_white_castle_long: bool,
    can_black_castle_short: bool,
    can_black_castle_long: bool,
}