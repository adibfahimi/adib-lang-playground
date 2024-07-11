mod utils;

use adib_lang::interpreter::Interpreter;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn evaluate_source_code(source_code: &str) -> String {
    let mut lexer = adib_lang::lexer::Lexer::new(source_code.chars().collect());

    let mut parser = adib_lang::parser::Parser::new(&mut lexer);

    let program = match parser.parse() {
        Ok(program) => program,
        Err(e) => return e,
    };

    let mut interpreter = Interpreter::new();
    match interpreter.eval(&program) {
        Ok(_) => String::from("completed!"),
        Err(e) => format!("error: {}", e),
    }
}
