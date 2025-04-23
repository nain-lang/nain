use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
// Adding Hash derive to TokenType for usage in HashMap
pub enum TokenType {
    // Special
    EndOfFile,
    Identifier,

    // Keywords
    If,
    Else,
    Elif,
    While,
    Loop,
    Eval,
    Import,
    Export,
    Abstract,
    Final,
    Static,
    Inline,
    Virtual,
    Class,
    Struct,
    Enum,
    Extend,
    Implement,
    Override,
    Let,
    Const,
    Macro,
    Func,  // Support for functions

    // Data types
    I8, I16, I32, I64, I128,
    U8, U16, U32, U64, U128, UArch, // 32 or 64 bits
    F32, F64,
    Bool,
    True,
    False,
    Char,
    Str,

    // Operators
    Add, Sub, Mul, Div, Mod, Pow,
    LogicalAnd, LogicalOr, LogicalNot, LogicalXor,
    LogicalEqual, LogicalNotEqual, LogicalLess,
    LogicalLessEqual, LogicalGreater, LogicalGreaterEqual,
    LogicalStrictEqual, LogicalStrictNotEqual,
    BitwiseAnd, BitwiseOr, BitwiseNot, BitwiseXor,
    BitwiseShiftLeft, BitwiseShiftRight,

    // Delimiters
    LeftParen, RightParen, LeftBracket, RightBracket, LeftBrace, RightBrace,
    Comma, Semicolon, Colon, DoubleColon, Arrow, FatArrow, DoubleDot,

    // Literals
    UnsignedIntLiteral,
    SignedIntLiteral,
    FloatLiteral,
    StringLiteral,
    CharLiteral,
    BoolLiteral, // true / false literals
}

/// A Struct that holds a single token
#[derive(Debug, Clone)]
pub struct TokenStruct {
    pub tok_type: TokenType,
    pub val: String,
    pub line: u32,
    pub pos: u16,
}

/// Creates and returns a HashMap with all the tokens and their value
pub fn create_token_map() -> HashMap<TokenType, String> {
    let mut token_map = HashMap::new();

    // Keywords
    token_map.insert(TokenType::If, "if".to_string());
    token_map.insert(TokenType::Else, "else".to_string());
    token_map.insert(TokenType::Elif, "elif".to_string());
    token_map.insert(TokenType::While, "while".to_string());
    token_map.insert(TokenType::Loop, "loop".to_string());
    token_map.insert(TokenType::Eval, "eval".to_string());
    token_map.insert(TokenType::Import, "import".to_string());
    token_map.insert(TokenType::Export, "export".to_string());
    token_map.insert(TokenType::Abstract, "abstract".to_string());
    token_map.insert(TokenType::Final, "final".to_string());
    token_map.insert(TokenType::Static, "static".to_string());
    token_map.insert(TokenType::Inline, "inline".to_string());
    token_map.insert(TokenType::Virtual, "virtual".to_string());
    token_map.insert(TokenType::Class, "class".to_string());
    token_map.insert(TokenType::Struct, "struct".to_string());
    token_map.insert(TokenType::Enum, "enum".to_string());
    token_map.insert(TokenType::Extend, "extend".to_string());
    token_map.insert(TokenType::Implement, "implement".to_string());
    token_map.insert(TokenType::Override, "override".to_string());
    token_map.insert(TokenType::Let, "let".to_string());
    token_map.insert(TokenType::Const, "const".to_string());
    token_map.insert(TokenType::Macro, "macro".to_string());
    token_map.insert(TokenType::Func, "func".to_string()); // Support for `func`

    // Data types
    token_map.insert(TokenType::I8, "i8".to_string());
    token_map.insert(TokenType::I16, "i16".to_string());
    token_map.insert(TokenType::I32, "i32".to_string());
    token_map.insert(TokenType::I64, "i64".to_string());
    token_map.insert(TokenType::I128, "i128".to_string());
    token_map.insert(TokenType::U8, "u8".to_string());
    token_map.insert(TokenType::U16, "u16".to_string());
    token_map.insert(TokenType::U32, "u32".to_string());
    token_map.insert(TokenType::U64, "u64".to_string());
    token_map.insert(TokenType::U128, "u128".to_string());
    token_map.insert(TokenType::UArch, "uarch".to_string());
    token_map.insert(TokenType::F32, "f32".to_string());
    token_map.insert(TokenType::F64, "f64".to_string());
    token_map.insert(TokenType::Bool, "bool".to_string());
    token_map.insert(TokenType::True, "true".to_string());
    token_map.insert(TokenType::False, "false".to_string());
    token_map.insert(TokenType::Char, "char".to_string());
    token_map.insert(TokenType::Str, "str".to_string());

    // Operators
    token_map.insert(TokenType::Add, "+".to_string());
    token_map.insert(TokenType::Sub, "-".to_string());
    token_map.insert(TokenType::Mul, "*".to_string());
    token_map.insert(TokenType::Div, "/".to_string());
    token_map.insert(TokenType::Mod, "%".to_string());
    token_map.insert(TokenType::Pow, "^".to_string());
    token_map.insert(TokenType::LogicalAnd, "&&".to_string());
    token_map.insert(TokenType::LogicalOr, "||".to_string());
    token_map.insert(TokenType::LogicalNot, "!".to_string());
    token_map.insert(TokenType::LogicalXor, "^^".to_string());
    token_map.insert(TokenType::LogicalEqual, "==".to_string());
    token_map.insert(TokenType::LogicalNotEqual, "!=".to_string());
    token_map.insert(TokenType::LogicalLess, "<".to_string());
    token_map.insert(TokenType::LogicalLessEqual, "<=".to_string());
    token_map.insert(TokenType::LogicalGreater, ">".to_string());
    token_map.insert(TokenType::LogicalGreaterEqual, ">=".to_string());
    token_map.insert(TokenType::LogicalStrictEqual, "===".to_string());
    token_map.insert(TokenType::LogicalStrictNotEqual, "!==".to_string());
    token_map.insert(TokenType::BitwiseAnd, "&".to_string());
    token_map.insert(TokenType::BitwiseOr, "|".to_string());
    token_map.insert(TokenType::BitwiseNot, "~".to_string());
    token_map.insert(TokenType::BitwiseXor, "^".to_string());
    token_map.insert(TokenType::BitwiseShiftLeft, "<<".to_string());
    token_map.insert(TokenType::BitwiseShiftRight, ">>".to_string());

    // Delimiters
    token_map.insert(TokenType::LeftParen, "(".to_string());
    token_map.insert(TokenType::RightParen, ")".to_string());
    token_map.insert(TokenType::LeftBracket, "[".to_string());
    token_map.insert(TokenType::RightBracket, "]".to_string());
    token_map.insert(TokenType::LeftBrace, "{".to_string());
    token_map.insert(TokenType::RightBrace, "}".to_string());
    token_map.insert(TokenType::Comma, ",".to_string());
    token_map.insert(TokenType::Semicolon, ";".to_string());
    token_map.insert(TokenType::Colon, ":".to_string());
    token_map.insert(TokenType::DoubleColon, "::".to_string());
    token_map.insert(TokenType::Arrow, "->".to_string());
    token_map.insert(TokenType::FatArrow, "=>".to_string());
    token_map.insert(TokenType::DoubleDot, "..".to_string());

    token_map
}

/// A function that matches a string to a TokenType
fn match_tok(s: &str) -> TokenType {
    let token_map = create_token_map();
    for (key, value) in token_map.iter() {
        if value == s {
            return *key;
        }
    }

    // Matching literal types
    if let Ok(_) = s.parse::<i64>() {
        return TokenType::SignedIntLiteral;
    }
    if let Ok(_) = s.parse::<u64>() {
        return TokenType::UnsignedIntLiteral;
    }
    if let Ok(_) = s.parse::<f64>() {
        return TokenType::FloatLiteral;
    }
    if s.starts_with("\"") && s.ends_with("\"") {
        return TokenType::StringLiteral;
    }
    if s.starts_with("'") && s.ends_with("'") {
        return TokenType::CharLiteral;
    }
    if s == "true" || s == "false" {
        return TokenType::BoolLiteral;
    }

    // Default: Identifier
    TokenType::Identifier
}

/// A function that returns all the tokens from a line
/// A function that returns all the tokens from a line
fn match_line(line: &str) -> Vec<TokenStruct> {
    let mut toks = Vec::new();
    let mut word = String::new();

    let mut i = 0;
    let chars = line.chars().collect::<Vec<char>>();

    while i < chars.len() {
        let ch = chars[i];

        // If the character is a space, treat the current word as a token
        if ch.is_whitespace() {
            if !word.is_empty() {
                toks.push(TokenStruct {
                    tok_type: match_tok(&word),
                    val: word.clone(),
                    line: 1,
                    pos: i as u16,
                });
                word.clear();
            }
        } else if ch == '(' {
            // Handle LeftParen as a separate token
            if !word.is_empty() {
                toks.push(TokenStruct {
                    tok_type: match_tok(&word),
                    val: word.clone(),
                    line: 1,
                    pos: i as u16 - word.len() as u16,
                });
                word.clear();
            }
            toks.push(TokenStruct {
                tok_type: TokenType::LeftParen,
                val: "(".to_string(),
                line: 1,
                pos: i as u16,
            });
        } else if ch == ')' {
            // Handle RightParen as a separate token
            if !word.is_empty() {
                toks.push(TokenStruct {
                    tok_type: match_tok(&word),
                    val: word.clone(),
                    line: 1,
                    pos: i as u16 - word.len() as u16,
                });
                word.clear();
            }
            toks.push(TokenStruct {
                tok_type: TokenType::RightParen,
                val: ")".to_string(),
                line: 1,
                pos: i as u16,
            });
        } else if ch == '{' {
            // Handle LeftBrace as a separate token
            if !word.is_empty() {
                toks.push(TokenStruct {
                    tok_type: match_tok(&word),
                    val: word.clone(),
                    line: 1,
                    pos: i as u16 - word.len() as u16,
                });
                word.clear();
            }
            toks.push(TokenStruct {
                tok_type: TokenType::LeftBrace,
                val: "{".to_string(),
                line: 1,
                pos: i as u16,
            });
        } else if ch == '}' {
            // Handle RightBrace as a separate token
            if !word.is_empty() {
                toks.push(TokenStruct {
                    tok_type: match_tok(&word),
                    val: word.clone(),
                    line: 1,
                    pos: i as u16 - word.len() as u16,
                });
                word.clear();
            }
            toks.push(TokenStruct {
                tok_type: TokenType::RightBrace,
                val: "}".to_string(),
                line: 1,
                pos: i as u16,
            });
        } else if ch == ';' {
            // Handle Semicolon as a separate token
            if !word.is_empty() {
                toks.push(TokenStruct {
                    tok_type: match_tok(&word),
                    val: word.clone(),
                    line: 1,
                    pos: i as u16 - word.len() as u16,
                });
                word.clear();
            }
            toks.push(TokenStruct {
                tok_type: TokenType::Semicolon,
                val: ";".to_string(),
                line: 1,
                pos: i as u16,
            });
        } else {
            // If the character is part of the current word, add it to the word
            word.push(ch);
        }

        i += 1;
    }

    // If a word is left after processing, create a token for it
    if !word.is_empty() {
        toks.push(TokenStruct {
            tok_type: match_tok(&word),
            val: word.clone(),
            line: 1,
            pos: i as u16 - word.len() as u16,
        });
    }

    toks
}


/// A function that returns all the tokens from the input
pub fn lex_in(input: &str) -> Vec<TokenStruct> {
    let mut toks = Vec::new();

    for line in input.lines() {
        for tok in match_line(line) {
            toks.push(tok);
        }
    }

    toks
}

fn main() {
    let input = "func my_function() { return 42; }";
    let tokens = lex_in(input);
    for token in tokens {
        println!("{:?}", token);
    }
}

