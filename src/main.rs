use unocalc::lexer::Lexer;
use unocalc::parser::Parser;

fn main() {
    let input = "-5 + 3.12 * (2 - 1) / 6";
    // let input = "-5+3.12*(2-1)";
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize().unwrap();
    let mut parser = Parser::new(&tokens);
    match parser.parse_expression(0) {
        Ok(ast) => println!("Result: {}", ast.evaluate()),
        Err(e) => println!("Error: {}", e),
    }
}

// #[wasm_bindgen]
// pub fn evaluate_expression(input: &str) -> f64 {
//     let mut lexer = Lexer::new(input);
//     match lexer.tokenize() {
//         Ok(tokens) => {
//             let mut parser = Parser::new(&tokens);
//             if let Some(ast) = parser.parse_expression(0) {
//                 ast.evaluate()
//             } else {
//                 panic!("Failed to parse expression.");
//             }
//         }
//         Err(e) => panic!("Lexer error: {}", e),
//     }
// }
