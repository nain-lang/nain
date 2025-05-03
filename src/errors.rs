// SPDX-License-Identifier: GPL-3.0-or-later OR MPL-2.0-or-later
// SPDX-FileCopyrightText: © 2025 The Nain Language Contributors. Some rights reserved.

//------------------------------------------- IMPORTS -----------------------------------

use ariadne::{Color, ColorGenerator, Fmt, Label, Report, Source};
use yansi::Paint;

//------------------------------------------ DEFINITIONS  --------------------------------

/// Struct to handle error reporting with customized color scheme
pub struct ErrorReporter {
    /// A color generator
    col_gen: ColorGenerator,
    /// The source file
    filename: String,
    /// The source code
    source: String,
    /// The stored reports
    reports: Vec<ReportType>,
}

/// An enum to represent the type of report
pub enum ReportKind {
    /// An error that hasn't been catched at compile-time.
    RuntimeError,
    /// An error that has been catched at compile-time.
    CompileTimeError,

    /// An important warning that might cause undefined behavior or really bad
    /// practises which will almost certainly break the program.
    WarningHigh,
    /// A regular warning: includes bad practises, depricated feature or
    /// un-peformant code.
    Warning,
    /// A low warning: Lots of nested Ifs, hard-to-read code, etc.
    WarningLow,

    /// A message which does not require much attention. Includes incorrect
    /// casing, inconsistent indentation/whitespace, etc.
    Message,
    /// Debug message. Not shown by default. Every `debug()` call on the program.
    DebugMessage,
    /// Spelling error on a comment. Not shown by default as it is annoying to the
    /// user.
    Spelling,
}

/// An struct that represents a report.
pub struct ReportType {
    /// The type of report
    pub report_type: ReportKind,
    /// The message
    pub message: String,
    /// The line number
    pub line: usize,
    /// The column number
    pub column: usize,
    /// The title of the report.
    pub title: String,
}

//--------------------------------------- IMPLEMENTATIONS -------------------------------

impl ErrorReporter {
    /// Constructs a new `ErrorReporter`.
    pub fn new(filename: &str, source: &str) -> Self {
        Self {
            col_gen: ColorGenerator::new(),
            filename: filename.to_string(),
            source: source.to_string(),
            reports: Vec::new(),
        }
    }

    /// Adds a new report
    pub fn add(
        &mut self,
        report_type: ReportKind,
        message: &str,
        line: usize,
        column: usize,
        title: &str,
    ) {
        self.reports.push(ReportType {
            report_type,
            message: message.to_string(),
            line,
            column,
            title: title.to_string(),
        });
    }

    /// Prints all the reports to `stderr`
    pub fn put(&self) {
        for report in &self.reports {
            let color = match report.report_type {
                ReportKind::CompileTimeError => Color::Red,
                ReportKind::RuntimeError => Color::Red,
                ReportKind::WarningHigh => Color::Yellow,
                ReportKind::Warning => Color::Yellow,
                ReportKind::WarningLow => Color::Cyan,
                ReportKind::Message => Color::Blue,
                ReportKind::DebugMessage => Color::Magenta,
                ReportKind::Spelling => Color::Green,
            };

            let kind = ariadne::ReportKind::Custom("title", color);

            Report::build(kind, (self.filename.clone(), report.line..report.line + 1))
                .with_message(&report.message)
                .with_label(
                    Label::new((self.filename.clone(), report.line..report.line + 1))
                        .with_message(&report.message)
                        .with_color(color),
                )
                .finish()
                .eprint((self.filename.clone(), Source::from(self.source.clone())))
                .unwrap();
        }
    }
}

//-------------------------------------- TESTS --------------------------------------------

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_error() {
        let input = "
This is a test

Runtime error here
Compile-time error here

Hi-Warning here
Warning here
Lo-warning here

Message here
Debug message here
Speling eror here
            ";
        let mut reporter = ErrorReporter::new("test.txt", input);
        reporter.add(
            ReportKind::RuntimeError,
            "Runtime Error",
            0,
            2,
            "Error Reporter Test",
        );
        reporter.add(
            ReportKind::CompileTimeError,
            "Compile-time error",
            0,
            3,
            "Error Reporter Test",
        );

        reporter.add(
            ReportKind::WarningHigh,
            "High Warning",
            0,
            4,
            "Error Reporter Test",
        );
        reporter.add(ReportKind::Warning, "Warning", 0, 6, "Error Reporter Test");
        reporter.add(
            ReportKind::WarningLow,
            "Low Warning",
            0,
            7,
            "Error Reporter Test",
        );

        reporter.add(ReportKind::Message, "Message", 0, 8, "Error Reporter Test");
        reporter.add(
            ReportKind::DebugMessage,
            "Debug Message",
            0,
            9,
            "Error Reporter Test",
        );
        reporter.add(
            ReportKind::Spelling,
            "Spelling Error",
            0,
            10,
            "Error Reporter Test",
        );
        reporter.put();
    }
}
