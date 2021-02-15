/*!
 * The lexer is the first part of a compiler, and is responsible
 * for converting the input string into a list of
 * tokens recognised by the compiler.
 * These tokens can be found in `token.rs`
 */
use crate::{Error, Token};
use alloc::{format, vec, vec::Vec};

/**
 * Convert a stream of characters into a list of tokens.
 */
pub fn lex(input: &str) -> Result<Vec<Token>, Error> {
    use Token::*;

    // build the initial state
    let mut tokens = vec![];
    let mut number = 0;
    let mut in_number = false;

    // loop over all characters
    for ch in input.chars() {
        // if we had a number and ended it, push it to the token list
        if !('0'..='9').contains(&ch) && in_number {
            tokens.push(Number(number));
            number = 0;
            in_number = false;
        }

        // more specific handling
        match ch {
            // handle whitespace
            ' ' | '\n' | '\t' | '\r' => {}

            // handle first digit of number
            '0'..='9' if !in_number => {
                in_number = true;
                number = (ch as u8) - b'0';
            }

            // handle more digits of the number
            '0'..='9' if in_number => {
                // this can only store <= 255 so handle overflow
                if number > 25 || (number == 25 && ch > '5') {
                    return Err(Error::Lexer(format!(
                        "found too big integer {}{}..",
                        number, ch
                    )));
                }

                // add this digit to the number
                number *= 10;
                number += (ch as u8) - b'0';
            }

            // handle operators
            '+' => tokens.push(Plus),
            '-' => tokens.push(Minus),
            '*' => tokens.push(Asterisk),
            '/' => tokens.push(Slash),
            '%' => tokens.push(Percent),

            // handle parentheses
            '(' => tokens.push(LeftParen),
            ')' => tokens.push(RightParen),

            // handle any other character, which we didn't expect
            _ => {
                return Err(Error::Lexer(format!(
                    "found unsupported character '{}'",
                    ch
                )));
            }
        }
    }

    // if we ended with a number, push it as well
    if in_number {
        tokens.push(Number(number));
    }

    Ok(tokens)
}
