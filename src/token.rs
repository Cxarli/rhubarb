/*!
 * A token is a single "unit" in the input string.
 * This can for example be an operator like +
 * or a number like 42.
 *
 * For now this only supports the addition, subtraction,
 * multiplication, division and modulo operators, and
 * decimal numbers.
 */

/**
 * The token
 */
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Token {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `*`
    Asterisk,
    /// `/`
    Slash,
    /// `%`
    Percent,
    /// `(`
    LeftParen,
    /// `)`
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
            // parentheses are sometimes treated like an operator,
            // but they aren't one
            LeftParen | RightParen => false,
            Plus | Minus | Asterisk | Slash | Percent => true,
        }
    }

    /**
     * Get the precedence of the token, if it has any
     */
    pub fn precedence(&self) -> Option<u8> {
        use Token::*;
        match self {
            Plus | Minus => Some(1),
            Asterisk | Slash | Percent => Some(2),
            LeftParen | RightParen => Some(10),
            Number(_) => None,
        }
    }

    // NOTE: if we ever decide to implement unary operators,
    // we'll also have to add some associativity here
}
