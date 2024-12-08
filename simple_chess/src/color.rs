use crate::Color::{Black, White};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Color {
    White,
    Black,
}

impl Color {
    /// Returns the opposite color.
    ///
    /// # Examples
    ///
    /// ```
    /// use simple_chess::Color::{Black, White};
    ///
    /// let color = White;
    /// assert_eq!(color.opposite(), Black);
    ///
    /// let color = Black;
    /// assert_eq!(color.opposite(), White);
    /// ```
    pub fn opposite(&self) -> Color {
        match self {
            White => Black,
            Black => White,
        }
    }
}
