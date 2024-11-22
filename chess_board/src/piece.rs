use std::any::Any;

/// A trait that represents a generic piece in a game or system.
///
/// Any type that implements this trait will be able to provide
/// a character representation of the piece, which can be useful
/// for displaying the piece on a text-based interface, logging,
/// or any other situation where a symbolic representation is helpful.
pub trait Piece {
    /// Provides the character representation of the piece.
    ///
    /// # Returns
    ///
    /// * `char` - A character that symbolically represents the piece.
    fn get_char_representation(&self) -> char;

    
    /// Provides the ability to cast the piece to a trait object of `Any`.
    ///
    /// # Returns
    ///
    /// * `&dyn Any` - A reference to the piece as a trait object of `Any`. This is useful
    /// for downcasting the piece to its concrete type.
    fn as_any(&self) -> &dyn Any;
}
