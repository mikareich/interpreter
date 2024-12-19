pub struct Lexicon {
    pub white_space_tokens: Vec<[&'static str; 2]>,
    pub keywords: Vec<&'static str>,
    pub operators: Vec<&'static str>,
}

#[derive(Debug)]
pub enum TokenClass {
    IntLiteral,
    Identifier,
    Operator,
    Keyword,
}

#[derive(Debug)]
pub struct Token {
    class: TokenClass,
    value: String,
}

fn split_by_white_spaces(source_code: &str, lexicon: &Lexicon) -> Vec<String> {
    let mut tokens: Vec<String> = vec![];

    let mut current_token = String::new();
    let mut current_white_space: Option<[&str; 2]> = None;

    for symbol in source_code.chars() {
        current_token.push(symbol);

        // check if current whitespace gets terminated
        if current_white_space.is_some() {
            let white_space = current_white_space.unwrap();

            if current_token.ends_with(white_space[1]) {
                current_white_space = None;
                current_token.clear();
            } else {
                continue;
            }
        }

        // check if code enters a new whitespace
        lexicon
            .white_space_tokens
            .iter()
            .find(|token| current_token.ends_with(token[0]))
            .map(|&white_space| {
                // set current white space if its not "self-closing" like [space], or [new-line]
                current_white_space = if !white_space[1].is_empty() {
                    Some(white_space)
                } else {
                    None
                };

                let white_space_len = white_space[0].len();
                let token_len = current_token.len();

                let trimmed_token = current_token[0..token_len - white_space_len].to_string();
                if !trimmed_token.is_empty() {
                    tokens.push(trimmed_token);
                }

                current_token.clear();
            });
    }

    tokens
}

/** Lexial Analysis: Tokenize source code */
pub fn lexical_analysis(source_code: &String, lexicon: Lexicon) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    for token in split_by_white_spaces(source_code, &lexicon) {
        // match token against int literal
        let integer = token.parse::<i32>();
        if integer.is_ok() {
            tokens.push(Token {
                class: TokenClass::IntLiteral,
                value: token.clone(),
            });

            continue;
        }

        // match token against keywords
        let keyword = lexicon.keywords.iter().find(|keyword| **keyword == token);
        if keyword.is_some() {
            tokens.push(Token {
                class: TokenClass::Keyword,
                value: token.clone(),
            });

            continue;
        }

        // match token against operators
        let operator = lexicon
            .operators
            .iter()
            .find(|operator| **operator == token);
        if operator.is_some() {
            tokens.push(Token {
                class: TokenClass::Operator,
                value: token.clone(),
            });

            continue;
        }

        // token must be an identifier
        tokens.push(Token {
            class: TokenClass::Identifier,
            value: token.clone(),
        })
    }

    tokens
}
