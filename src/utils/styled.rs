// Copyright (C) 2015, Alberto Corona <ac@albertocorona.com>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

extern crate ansi_term;

use self::ansi_term::Style;
use self::ansi_term::{Colour,ANSIString};
use self::ansi_term::Colour::{Red};

/// Wrapper for `ansi_term::Colour`
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
}

/// Trait for creating stylized console printing using `ansi_term`
pub trait Styled {
    /// Creates a bold `ANSIString`
    ///
    /// # Example
    /// ```
    /// use rpf::Styled;
    ///
    /// println!("{}", "Styled string test".bold());
    /// ```
    fn bold(&self) -> ansi_term::ANSIString;

    /// Creates an underlined `ANSIString`
    ///
    /// # Example
    /// ```
    /// use rpf::Styled;
    ///
    /// println!("{}", "Styled string test".underline());
    /// ```
    fn underline(&self) -> ansi_term::ANSIString;

    /// Paints a given string with the color given
    ///
    /// # Example
    /// ```
    /// use rpf::Styled;
    /// use rpf::Color;
    ///
    /// println!("{}", "Styled string test".paint(Color::Yellow));
    /// ```
    fn paint(&self, color: Color) -> ansi_term::ANSIString;
}

impl<T: AsRef<str>> Styled for T {
    fn bold(&self) -> ansi_term::ANSIString {
        Style::default().bold().paint(&self.as_ref())
    }

    fn underline(&self) -> ansi_term::ANSIString {
        Style::default().underline().paint(&self.as_ref())
    }

    fn paint(&self, color: Color) -> ansi_term::ANSIString {
        match color {
            Color::Black   => { Colour::Black.paint(&self.as_ref()) },
            Color::Red     => { Colour::Red.paint(&self.as_ref()) },
            Color::Green   => { Colour::Green.paint(&self.as_ref()) },
            Color::Yellow  => { Colour::Yellow.paint(&self.as_ref()) },
            Color::Blue    => { Colour::Blue.paint(&self.as_ref()) },
            Color::Purple  => { Colour::Purple.paint(&self.as_ref()) },
            Color::Cyan    => { Colour::Cyan.paint(&self.as_ref()) },
            Color::White   => { Colour::White.paint(&self.as_ref()) },
        }
    }
}

#[test]
fn test_styled_bold() {
    assert_eq!(String::from("Styled string test").bold().to_string(),
        Style::default().bold().paint("Styled string test").to_string());
}

#[test]
fn test_styled_underline() {
    assert_eq!(String::from("Styled string test").underline().to_string(),
        Style::default().underline().paint("Styled string test").to_string());
}

#[test]
fn test_styled_paint() {
    assert_eq!(String::from("Styled string test").paint(Color::Yellow).to_string(),
        Colour::Yellow.paint("Styled string test").to_string());
}
