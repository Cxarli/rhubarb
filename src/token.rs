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
    /// (
    LeftParen,
    /// )
    RightParen,
    /// any number between 0-255
    Number(u8),
}

impl Token {
    /**
     * Check if this token is an operator
     */
    pub fn is_operator(&self) -> bool {
        use Token::*;
        match self {
            Number(_) => false,
            LeftParen | RightParen => false,
            Plus | Minus | Asterisk | Slash | Percent => true,
        }
    }

    /**
     * Get the precedence of the token
     * For numbers and parentheses this panics
     */
    pub fn precedence(&self) -> u8 {
        use Token::*;
        match self {
            Plus | Minus => 1,
            Asterisk | Slash | Percent => 2,
            LeftParen | RightParen => panic!("parentheses aren't operators"),
            Number(_) => panic!("numbers aren't operators"),
        }
    }
}
