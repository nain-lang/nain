// SPDX-License-Identifier: GPL-3.0-or-later OR MPL-2.0-or-later
// SPDX-FileCopyrightText: © 2025 The Nain Language Contributors. Some rights reserved.

//----------------------------------------- IMPORTS ------------------------------------

use logos::Logos;

//------------------------------------------ CODE --------------------------------------

#[derive(Logos, Debug, PartialEq, Clone, Copy)]
pub enum Token {
    // --- Keywords ---
    #[token("return")]
    Return,
    #[token("func")]
    Func,
    #[token("let")]
    Let,
    #[token("const")]
    Const,
    #[token("globl")]
    Globl,
    #[token("mut")]
    Mutable,
    #[token("if")]
    If,
    #[token("else")]
    Else,
    #[token("while")]
    While,
    #[token("for")]
    For,
    #[token("in")]
    In,
    #[token("true")]
    True,
    #[token("false")]
    False,
    #[token("null")]
    Null,
    #[token("struct")]
    Struct,
    #[token("enum")]
    Enum,
    #[token("union")]
    Union,
    #[token("import")]
    Import,

    // --- Types ---
    #[token("i32")]
    I32,
    #[token("i64")]
    I64,
    #[token("i128")]
    I128,
    #[token("u8")]
    U8,
    #[token("u16")]
    U16,
    #[token("u32")]
    U32,
    #[token("u64")]
    U64,
    #[token("u128")]
    U128,
    #[token("bool")]
    Bool,
    #[token("str")]
    Str,
    #[token("bit")]
    Bit,
    #[token("unit")]
    Unit,

    // --- Symbols ---
    #[token("(")]
    LParen,
    #[token(")")]
    RParen,
    #[token("{")]
    LBrace,
    #[token("}")]
    RBrace,
    #[token("[")]
    LBracket,
    #[token("]")]
    RBracket,
    #[token(";")]
    Semicolon,
    #[token(":")]
    Colon,
    #[token(",")]
    Comma,
    #[token(".")]
    Dot,
    #[token("->")]
    Arrow,
    #[token("=>")]
    FatArrow,

    // --- Operators ---
    #[token("+")]
    Plus,
    #[token("-")]
    Minus,
    #[token("*")]
    Star,
    #[token("%")]
    Mod,
    #[token("^")]
    Pow,
    #[token("/")]
    Slash,

    #[token("==")]
    Eq,
    #[token("!=")]
    NotEq,
    #[token("<=")]
    LessOrEqual,
    #[token(">=")]
    GreaterOrEqual,
    #[token("<")]
    LessThan,
    #[token(">")]
    Greater,

    #[token("=")]
    Assign,
    #[token("+=")]
    AddAssign,
    #[token("-=")]
    MinusAssign,
    #[token("*=")]
    StarAssign,
    #[token("^=")]
    PowAssign,

    #[token("&")]
    Reference,
    #[token("$")]
    DeReference,

    // --- Literals ---
    #[regex(r#""([^"\\]|\\.)*""#)]
    StringLiteral,
    #[regex(r"[0-9]+")]
    NumberLiteral,
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    // --- Skipped ---
    #[regex(r"[ \t\r\n]+", logos::skip)] // whitespace
    #[regex(r"//[^\n]*", logos::skip)] // single-line comment
    #[regex(r"/\*[^*]*\*+([^/*][^*]*\*+)*/", logos::skip)] // multi-line comment

    // --- Catch-all Error ---
    #[regex(r".", priority = 0)]
    Error,
}

//----------------------------------- TESTS ---------------------------------------------

#[cfg(test)]
mod tests {

    use logos::{Lexer, Logos};

    use super::Token;
    // --- Valid Token Tests ---

    #[test]
    fn test_comment_line() {
        let input = "// This is a comment";
        let mut lexer = crate::Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap()); // Unwrap here to handle the Result type
        }

        // No tokens should be produced from a comment
        assert_eq!(tokens, Vec::<crate::Token>::new());
    }

    #[test]
    fn test_edge_case_empty_input() {
        let input = "";
        let mut lexer = crate::Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap()); // Unwrap here to handle the Result type
        }

        // No tokens should be produced from an empty input
        assert_eq!(tokens, Vec::<crate::Token>::new());
    }

    #[test]
    fn test_invalid_token() {
        let input = "ñ";
        let expected: Vec<crate::lexer::Token> = vec![crate::Token::Error];
        let lexer = crate::lexer::Token::lexer(input);

        let tokens: Vec<Token> = lexer
            .map(|res| match res {
                Ok(t) => t,
                Err(()) => crate::Token::Error,
            })
            .collect();

        assert_eq!(expected, tokens);
    }

    #[test]
    fn test_mixed_case_tokens() {
        let input = "Let func IF while";
        let mut lexer = crate::Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap()); // Unwrap here to handle the Result type
        }

        assert_eq!(
            tokens,
            vec![
                crate::Token::Identifier,
                crate::Token::Func,
                crate::Token::Identifier,
                crate::Token::While
            ]
        );
    }

    #[test]
    fn test_let_statement() {
        let input = "let x = 5;";
        let mut lexer = crate::Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap()); // Unwrap here to handle the Result type
        }

        assert_eq!(
            tokens,
            vec![
                crate::Token::Let,
                crate::Token::Identifier,
                crate::Token::Assign,
                crate::Token::NumberLiteral,
                crate::Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_large_input() {
        let input = "let a = 10 + 5;"; // A simple large expression
        let mut lexer = crate::Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap()); // Unwrap here to handle the Result type
        }

        assert_eq!(
            tokens,
            vec![
                crate::Token::Let,
                crate::Token::Identifier,
                crate::Token::Assign,
                crate::Token::NumberLiteral,
                crate::Token::Plus,
                crate::Token::NumberLiteral,
                crate::Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_mixed_content_with_whitespace() {
        let input = "let x = 42; // assign value to x";
        let mut lexer = crate::Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap()); // Unwrap here to handle the Result type
        }

        assert_eq!(
            tokens,
            vec![
                crate::Token::Let,
                crate::Token::Identifier,
                crate::Token::Assign,
                crate::Token::NumberLiteral,
                crate::Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_multiline_input() {
        let input = "
let x = 10;
let y = 20;
";
        let mut lexer = crate::Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap()); // Unwrap here to handle the Result type
        }

        assert_eq!(
            tokens,
            vec![
                crate::Token::Let,
                crate::Token::Identifier,
                crate::Token::Assign,
                crate::Token::NumberLiteral,
                crate::Token::Semicolon,
                crate::Token::Let,
                crate::Token::Identifier,
                crate::Token::Assign,
                crate::Token::NumberLiteral,
                crate::Token::Semicolon
            ]
        );
    }

    #[test]
    fn test_whitespace_handling() {
        let input = "let  x  =  42 ;";
        let mut lexer = crate::Token::lexer(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next() {
            tokens.push(token.unwrap()); // Unwrap here to handle the Result type
        }

        assert_eq!(
            tokens,
            vec![
                crate::Token::Let,
                crate::Token::Identifier,
                crate::Token::Assign,
                crate::Token::NumberLiteral,
                crate::Token::Semicolon
            ]
        );
    }
    #[test]
    fn test_full_code() {
        let source = "
import Std.IO;

func Main(argc: u32, argv: str[]) {
}
        ";

        let expected: Vec<crate::lexer::Token> = vec![
            crate::lexer::Token::Import,
            crate::lexer::Token::Identifier,
            crate::lexer::Token::Dot,
            crate::lexer::Token::Identifier,
            crate::lexer::Token::Semicolon,
            crate::lexer::Token::Func,
            crate::lexer::Token::Identifier,
            crate::lexer::Token::LParen,
            crate::lexer::Token::Identifier,
            crate::lexer::Token::Colon,
            crate::lexer::Token::U32,
            crate::lexer::Token::Comma,
            crate::lexer::Token::Identifier,
            crate::lexer::Token::Colon,
            crate::lexer::Token::Str,
            crate::lexer::Token::LBracket,
            crate::lexer::Token::RBracket,
            crate::lexer::Token::RParen,
            crate::lexer::Token::LBrace,
            crate::lexer::Token::RBrace,
        ];
    }
}
