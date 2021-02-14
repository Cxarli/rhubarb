/**
 * A description of an error somewhere along the compiler chain
 */
#[derive(Debug)]
pub enum Error {
    Lexer(String),
    Parser(String),
    Transformer(String),
    Generator(String),

    Io(::std::io::Error),
}

/// Mark it as a general Error
impl std::error::Error for Error {}

/**
 * Since std::io::Error doesn't implement PartialEq directly,
 * we'll have to write our own wrapper which converts it based
 * on display string
 */
impl Eq for Error {}

impl PartialEq for Error {
    fn eq(&self, rhs: &Self) -> bool {
        self.to_string() == rhs.to_string()
    }
}

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

            Io(x) => fmt.write_fmt(format_args! {"i/o error: {}", x}),
        }
    }
}
