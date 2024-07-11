mod utils;

use adib_lang::{eval, parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn evaluate_source_code(source_code: &str) -> String {
    let mut lexer = adib_lang::lexer::Lexer::new(source_code);
    let tokens = match lexer.tokenize() {
        Ok(tokens) => tokens,
        Err(e) => {
            return format!("Error: {}", e);
        }
    };

    let exprs = parser::parse(&tokens);
    let mut env = eval::Environment::new();

    for ast in exprs {
        match eval::eval(&ast, &mut env) {
            Ok(_) => {}
            Err(e) => {
                return format!("Error: {:?}", e);
            }
        }
    }

    "Evaluation completed".to_string()
}
