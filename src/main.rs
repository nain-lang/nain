// SPDX-License-Identifier: GPL-3.0-or-later OR MPL-2.0-or-later
// SPDX-FileCopyrightText: © 2025 The Nain Language Contributors. Some rights reserved.

//! # Nain
//!
//! Nain is a free and open-source systems programming language with a focus
//! on explicit syntax, strong static typing, and compile-time enforcement on
//! code correctness.
//!
//! Nain convines low-level control with modern concepts from functional
//! programming paradigms.
//!

use lexer::Token;
use logos::Logos;

mod lexer;
mod parser;

fn main() {
    let source = "
// This is a comment
/* This is also a comment */

import Std.IO
import Std.Rand

func main(argc: u16, argv: str[]) {
	println(\"There are {?} arguments\", argc);
	for [arg in argv.iter()] {
		print(\"Argument number {?} is: {?}\", arg, argv[arg])
	}
}
    ";

    for result in Token::lexer(source) {
        match result {
            Ok(token) => println!("{:#?}", token),
            Err(e) => println!("{:#?}", e),
        }
    }
}
