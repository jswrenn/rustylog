#![feature(globs, macro_rules, phase)]


mod tokenizer;
mod parser;
mod semantics;


fn main() {
    let tokens = "atom(VARIABLE,atom) ← atom".chars();
    let mut tokenizer = tokenizer::Tokenizer::new(tokens);
    for token in tokenizer {
        println!("{}",token);
    }
}
