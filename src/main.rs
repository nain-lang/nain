// SPDX-License-Identifier: GPL-3.0-or-later
// SPDX-FileCopyrightText: Copyright (C) 2025 Mario Rosell

/*
 *      THE NAIN PROGRAMMING LANGUAGE
 *  =====================================
 *  Nain - the post-modern programming language
 *
 *  Copyright (C) 2025 Mario Rosell
 *
 *  Nain is a free and open-source programming language for embedded systems and systems
 *  programming.
 *
 *  Nain is fast, efficient, and allows for low-level programming while providing high-level
 *  abstractions.
 *
 *  Nain is distributed under the terms of the GNU General Public License (GPL) version 3 or
 *  later if applicable.
 */

mod lex;

fn main() {
    let input = "func my_function() { return 42; }";
    let tokens = lex::lex_in(input);
    for token in tokens {
        println!("{:?}", token);
    }
}
