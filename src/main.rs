mod lexer;
mod evaluator;
mod ast;
mod parser;

use std::env;
use lexer::Token;
use chumsky::prelude::*;
use logos::Logos;
fn main() {
    //reads the input expression from the command line
    let input = env::args()
        .nth(1)
        .expect("Expected expression argument (e.g. `1 + 7 * (3 - 4) / 5`)");

    //creates a lexer instance from the input
    let lexer = Token::lexer(&input);

    //splits the input into tokens, using the lexer
    let mut tokens = vec![];
    for (token, span) in lexer.spanned() {
        match token {
            Ok(token) => tokens.push(token),
            Err(e) => {
                println!("lexer error at {:?}: {}", span, e);
                return;
            }
        }
    }

    //parses the tokens to construct an AST
    let ast = match parser::parser().parse(tokens) {
        Ok(expr) => {
            println!("[AST]\n{:#?}", expr);
            expr
        }
        Err(e) => {
            println!("parse error: {:#?}", e);
            return;
        }
    };

    //evaluates the AST to get the result
    println!("\n[result]\n{}", ast.eval());
}
