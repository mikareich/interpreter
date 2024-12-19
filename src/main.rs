mod lexer;
use lexer::{lexical_analysis, Lexicon};

const FULL_COMMENT: [&str; 2] = ["/*", "*/"];
const INLINE_COMMENT: [&str; 2] = ["//", "\n"];
const WHITE_SPACE: [&str; 2] = [" ", ""];
const NEW_LINE: [&str; 2] = ["\n", ""];
const TAB: [&str; 2] = ["\t", ""];

fn main() {
    let source_code = match std::fs::read("test.humplet") {
        Ok(code) => String::from_utf8(code).unwrap(),
        Err(_) => panic!("Could not read file"),
    };

    let lexicon = Lexicon {
        white_space_tokens: vec![FULL_COMMENT, INLINE_COMMENT, WHITE_SPACE, NEW_LINE, TAB],
        keywords: vec!["let", "if", "then", "else", "be", "."],
        operators: vec!["eq", "lt", "gt"],
    };

    let tokens = lexical_analysis(&source_code, lexicon);

    println!("{:?}", tokens)
}
