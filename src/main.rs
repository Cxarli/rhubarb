/**!
 * Rhubarb is a very simple compiler from a basic math equation
 * containing just numbers and +-/%* into a series of instructions
 * which fit the Intel 8080 manual.
 */
use rhubarb::*;

/**
 * The main function with Result support
 */
fn _main() -> Result<(), Error> {
    // create some mockup input
    let input = "9 - ((4 * (3 + 6)) % 8) % (((2 * (5 - 0)) - (7 / 1)) * 3)";

    // tokenise
    let tokens = lex(input)?;
    println!("tokens: {:?}", &tokens);

    // parse
    let expr_ast = parse(tokens)?;
    println!("expr_ast: {:?}", &expr_ast);

    // transform
    let bobo_ast = transform(expr_ast)?;
    println!("bobo_ast: {:?}", &bobo_ast);

    // generate
    let output = generate(bobo_ast)?;

    // output
    println!("input:\n{}\n\noutput:\n{}", input, output);
    Ok(())
}

/**
 * Wrapper around _main to handle the error
 */
fn main() {
    if let Err(e) = _main() {
        eprintln!("error: {:?}", e);
    }
}
