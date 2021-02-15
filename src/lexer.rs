/**!
 * The lexer is the first part of a compiler, and is responsible
 * for converting the input (string, file, etc)  into a list of
 * tokens recognised by the compiler.
 */
use crate::{
    Error,
    Token::{self, *},
};

/**
 * Convert a stream of characters into a list of tokens.
 */
pub fn lex(input: &str) -> Result<Vec<Token>, Error> {
    struct State {
        // the list of tokens which we'll build
        tokens: Vec<Token>,

        // the temporary number counter
        number: u8,
        in_number: bool,
    }

    // build the initial state
    let mut state = State {
        tokens: vec![],
        number: 0,
        in_number: false,
    };

    /*
     * Helper to push the nunmber to the tokens if there is one
     */
    let end_number = |state: &mut State| {
        // if we were inside a number, this is the time
        // to round it off and put it in the tokens list
        if state.in_number {
            state.tokens.push(Number(state.number));
            state.number = 0;
            state.in_number = false;
        }
    };

    // loop over all characters
    for ch in input.chars() {
        match ch {
            // handle whitespace
            ' ' | '\n' | '\t' | '\r' => end_number(&mut state),

            // handle first digit of number
            '0'..='9' if !state.in_number => {
                state.in_number = true;
                state.number = (ch as u8) - b'0';
            }

            // handle more digits of the number
            '0'..='9' if state.in_number => {
                // this can only store <= 255 so handle overflow
                if state.number > 25 || (state.number == 25 && ch > '5') {
                    return Err(Error::Lexer(format!(
                        "found too big integer {}{}..",
                        state.number, ch
                    )));
                }

                // add this digit to the number
                state.number *= 10;
                state.number += (ch as u8) - b'0';
            }

            // handle operators
            '+' => {
                end_number(&mut state);
                state.tokens.push(Plus);
            }
            '-' => {
                end_number(&mut state);
                state.tokens.push(Minus);
            }
            '*' => {
                end_number(&mut state);
                state.tokens.push(Asterisk);
            }
            '/' => {
                end_number(&mut state);
                state.tokens.push(Slash);
            }
            '%' => {
                end_number(&mut state);
                state.tokens.push(Percent);
            }

            // handle parentheses
            '(' => {
                end_number(&mut state);
                state.tokens.push(LeftParen);
            }
            ')' => {
                end_number(&mut state);
                state.tokens.push(RightParen);
            }

            // handle any other character, which we didn't expect
            _ => {
                return Err(Error::Lexer(format!(
                    "found unsupported character '{}'",
                    ch
                )));
            }
        }
    }

    // round off
    end_number(&mut state);

    Ok(state.tokens)
}
