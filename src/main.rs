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
//! ## Quickstart
//!
//! To get started with Nain, follow these steps:
//!
//! ### Installation
//!
//! ```bash
//! cargo install nain
//! ```
//!
//! ### Our first program
//!
//! Before making our first program, we need two things:
//!
//! - **A text editor** (e.g. VSCode with the Nain extension)
//! - **A terminal**
//!
//! Every single OS comes with a terminal, so you can use that.
//!
//! > **Note**
//! > If you use MacOS, the Nain CLI might look strange.
//! > Use [iTerm2](https://iterm2.com/) instead.
//!
//! You can use any text editor. But *real* programmers use special editor that
//! automatically highlights keywords and other things, autocomplete code, etc.
//!
//! For starting out, we recommend [VSCode](https://code.visualstudio.com/)
//! with the Nain extension.
//!
//! > **Note**
//! > You can use any editor. Even Notepad if you wanted to.
//! > But it is better to use these *special* editors.
//!
//! Now, make a file called `hello.na` and open it with the editor you chose.
//!
//! And now, we can proceed to write our first program.
//!
//! First, we need to import the `Std.io` *submodule*, which gives us a nice way
//! to do I/O.
//!
//! > **Explanation**
//! > A *submodule* is just code we can use. These are
//! > inside of a module, which is exactly what you (*probably*)
//! > are thinking of.
//!
//! It includes the `println` function, which allows us to print text to the
//! console.
//!
//! To import modules in Nain, we use this syntax:
//!
//! ```nain
//! import <module>.<submodule>;
//! ```
//!
//! Note the semicolon at the end.
//! For example to import the `Std.io` submodule, we would write:
//!
//! ```nain
//! import Std.io;
//! ```
//!
//! Then, we can use the `println` function, inside the `Main` function:
//!
//! ```nain
//! import Std.io;
//!
//! func Main() { // Note that we use `func` on Nain.
//!   println("Hello world!");
//! }
//! ```
//!
//! Now, to exit, we use the `exit` function:
//!
//! ```nain
//! exit(0); // 0 means "no errors". Everything else mesans an error.
//! ```
//!
//! Putting all together, a *Hello, World!* program looks like this:
//!
//! ```nain
//! import Std.io;
//!
//! func Main() {
//!   println("Hello, World!");
//!   exit(0);
//! }
//! ```
//!
//! And that's it!
//!
//! ### Running programs
//!
//! To run a Nain program, you can use the `nain` command:
//!
//! ```bash
//! nain hello.na
//! ```
//! Now, we can't fit the *whole* language in this page, so we need to stop here.
//! But, you can find a *longer* tutorial [on the wiki](https://wiki.nainlang.xyz/wiki/Beginners_Tutorial).
//!
//! There you will learn tons of cool stuff, so check it out!
//!
//! There is a also a lot of documentation for other things there.
//!

use lexer::Token;
use logos::Logos;

mod errors;
mod lexer;
mod parser;

fn main() {
    let source = "
import Std.io;

func Main() {
    println(\"Hello, World!\");
    exit(0);
}
        ";

    let mut tokens: Vec<Token> = Vec::new();
    let lexer = Token::lexer(source);

    for tok in lexer {
        tokens.push(tok.unwrap());
    }

    for tok in tokens {
        println!("{:?}", tok);
    }
}
