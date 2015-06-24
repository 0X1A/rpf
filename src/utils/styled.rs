// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of prog, distributed under the
// GPL v3 license. For full terms please see the LICENSE file.

extern crate ansi_term;

use self::ansi_term::Style;
use self::ansi_term::{Colour,ANSIString};
use self::ansi_term::Colour::{Red};

/// Trait for creating stylized console printing using `ansi_term`

pub trait Styled {
    fn bold(&self) -> ansi_term::ANSIString;
    fn underline(&self) -> ansi_term::ANSIString;
    fn paint(&self, color: ansi_term::Colour) -> ansi_term::ANSIString;
}

impl Styled for String {
    fn bold(&self) -> ansi_term::ANSIString {
        Style::default().bold().paint(&self)
    }

    fn underline(&self) -> ansi_term::ANSIString {
        Style::default().underline().paint(&self)
    }

    fn paint(&self, color: ansi_term::Colour) -> ansi_term::ANSIString {
        match color {
            Colour::Black   => { Colour::Black.paint(&self) },
            Colour::Red     => { Colour::Red.paint(&self) },
            Colour::Green   => { Colour::Green.paint(&self) },
            Colour::Yellow  => { Colour::Yellow.paint(&self) },
            Colour::Blue    => { Colour::Blue.paint(&self) },
            Colour::Purple  => { Colour::Purple.paint(&self) },
            Colour::Cyan    => { Colour::Cyan.paint(&self) },
            Colour::White   => { Colour::White.paint(&self) },
            _   => { Colour::White.paint(&self) },
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

    fn paint(&self, color: ansi_term::Colour) -> ansi_term::ANSIString {
        match color {
            Colour::Black   => { Colour::Black.paint(&self) },
            Colour::Red     => { Colour::Red.paint(&self) },
            Colour::Green   => { Colour::Green.paint(&self) },
            Colour::Yellow  => { Colour::Yellow.paint(&self) },
            Colour::Blue    => { Colour::Blue.paint(&self) },
            Colour::Purple  => { Colour::Purple.paint(&self) },
            Colour::Cyan    => { Colour::Cyan.paint(&self) },
            Colour::White   => { Colour::White.paint(&self) },
            _   => { Colour::White.paint(&self) },
        }
    }
}
