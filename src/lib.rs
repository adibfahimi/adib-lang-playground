mod utils;

use adib_lang::{eval, lexer, parser};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn evaluate_source_code(source_code: &str) -> String {
    let tokens = match lexer::tokenize(source_code) {
        Ok(tokens) => tokens,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            return String::from("Error during tokenization");
        }
    };

    let asts = parser::parse(&tokens);
    let mut env = eval::Environment::new();

    for ast in asts {
        eval::eval(&ast, &mut env);
    }

    "Evaluation completed".to_string()
}
