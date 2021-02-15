use alloc::string::String;

/**
 * A description of an error somewhere along the compiler chain
 */
#[derive(Debug)]
pub enum Error {
    Lexer(String),
    Parser(String),
    Transformer(String),
    Generator(String),
}

/*

/// Mark it as a general Error
impl std::error::Error for Error {}

/**
 * Convert the error into something displayable
 */
impl ::std::fmt::Display for Error {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        use Error::*;
        match self {
            Lexer(x) => fmt.write_fmt(format_args! {"lexer error: {}", x}),
            Parser(x) => fmt.write_fmt(format_args! {"parser error: {}", x}),
            Transformer(x) => fmt.write_fmt(format_args! {"transformer error: {}", x}),
            Generator(x) => fmt.write_fmt(format_args! {"generator error: {}", x}),
        }
    }
}

*/
