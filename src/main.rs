use rhubarb::*;

fn main() {
    // create some mockup input
    let input = "1 + 2 * 3";

    // tokenise
    let tokens = lex(input).unwrap();
    println!("tokens: {:?}", &tokens);

    // parse
    let expr_ast = parse(tokens).unwrap();
    println!("expr_ast: {:?}", &expr_ast);

    // transform
    let bobo_ast = transform(expr_ast).unwrap();
    println!("bobo_ast: {:?}", &bobo_ast);

    // generate
    let output = generate(bobo_ast).unwrap();

    // output
    println!("{} => {}", input, output);
}
