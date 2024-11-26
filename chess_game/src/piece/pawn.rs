use crate::Color;

pub fn as_utf_str(color: Color) -> &'static str {
    match color {
        Color::White => "♙",
        Color::Black => "♟",
    }
}

pub fn as_fen_char(color: Color) -> char {
    match color {
        Color::White => 'P',
        Color::Black => 'p',
    }
}
