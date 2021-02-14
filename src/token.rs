/**
 * A token is a single "unit" in the input string.
 * This can for example be an operator like +
 * or a number like 42.
 *
 * For now this only supports the addition, subtraction,
 * multiplication, division and modulo operators, and
 * decimal numbers.
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    /// +
    Plus,
    /// -
    Minus,
    /// *
    Asterisk,
    /// /
    Slash,
    /// %
    Percent,
    /// any number between 0-255
    Number(u8),
}
