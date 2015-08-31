// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
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
    /// let styled = "Styled string test".bold();
    /// println!("{}", styled);
    /// ```
    fn bold(&self) -> ansi_term::ANSIString;

    /// Creates an underlined `ANSIString`
    ///
    /// # Example
    /// ```
    /// use rpf::Styled;
    ///
    /// let styled = "Styled string test".underline();
    /// println!("{}", styled);
    /// ```
    fn underline(&self) -> ansi_term::ANSIString;

    /// Paints a given string with the color given
    ///
    /// # Example
    /// ```
    /// use rpf::Styled;
    /// use rpf::Color;
    ///
    /// let painted = "Styled string test".paint(Color::Yellow);
    /// println!("{}", painted);
    /// ```
    fn paint(&self, color: Color) -> ansi_term::ANSIString;
}

impl Styled for String {
    fn bold(&self) -> ansi_term::ANSIString {
        Style::default().bold().paint(&self)
    }

    fn underline(&self) -> ansi_term::ANSIString {
        Style::default().underline().paint(&self)
    }

    fn paint(&self, color: Color) -> ansi_term::ANSIString {
        match color {
            Color::Black   => { Colour::Black.paint(&self) },
            Color::Red     => { Colour::Red.paint(&self) },
            Color::Green   => { Colour::Green.paint(&self) },
            Color::Yellow  => { Colour::Yellow.paint(&self) },
            Color::Blue    => { Colour::Blue.paint(&self) },
            Color::Purple  => { Colour::Purple.paint(&self) },
            Color::Cyan    => { Colour::Cyan.paint(&self) },
            Color::White   => { Colour::White.paint(&self) },
        }
    }
}

impl Styled for str {
    fn bold(&self) -> ansi_term::ANSIString {
        Style::default().bold().paint(&self)
    }

    fn underline(&self) -> ansi_term::ANSIString {
        Style::default().underline().paint(&self)
    }

    fn paint(&self, color: Color) -> ansi_term::ANSIString {
        match color {
            Color::Black   => { Colour::Black.paint(&self) },
            Color::Red     => { Colour::Red.paint(&self) },
            Color::Green   => { Colour::Green.paint(&self) },
            Color::Yellow  => { Colour::Yellow.paint(&self) },
            Color::Blue    => { Colour::Blue.paint(&self) },
            Color::Purple  => { Colour::Purple.paint(&self) },
            Color::Cyan    => { Colour::Cyan.paint(&self) },
            Color::White   => { Colour::White.paint(&self) },
        }
    }
}

#[test]
fn test_styled_bold() {
    let styled = String::from("Styled string test");
    assert_eq!(styled.bold().to_string(),
        Style::default().bold().paint("Styled string test").to_string());
}

#[test]
fn test_styled_underline() {
    let styled = String::from("Styled string test");
    assert_eq!(styled.underline().to_string(),
        Style::default().underline().paint("Styled string test").to_string());
}

#[test]
fn test_styled_paint() {
    let styled = String::from("Styled string test");
    assert_eq!(styled.paint(Color::Yellow).to_string(),
        Colour::Yellow.paint("Styled string test").to_string());
}
