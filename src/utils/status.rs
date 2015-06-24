// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

use utils::PathMod;
use utils::Styled;
use utils::Color;

use std::process;
use std::path::PathBuf;

/// Enum used for setting exit statuses
#[derive(Copy,Clone)]
pub enum Status {
    Ok,
    Error,
    OptError,
    ArgError,
}

impl Status {
    /// Wrapper for `process::exit`, immediately exits the process with the set
    /// exit status.
    ///
    /// # Example
    ///
    /// ```should_panic
    /// use rpf::Status;
    ///
    /// Status::ArgError.exit();
    /// ```
    pub fn exit(self) {
        process::exit(self as i32);
    }

    /// Used for errors, prints error messages in red terminal font and calls
    /// `rpf::exit`
    ///
    /// # Example
    ///
    /// ```should_panic
    /// use rpf::Status;
    /// use std::path::Path;
    ///
    /// let file = Path::new("file.txt");
    /// Status::Error.err("util", "Hit an error".to_string());
    /// ```
    pub fn err(&self, rpf: &str, mesg: String) {
        println!("{}{} {}", rpf.paint(Color::Red), ":".paint(Color::Red),
        mesg.paint(Color::Red));
        self.exit();
    }

    /// Used for errors when working with paths, works similar to `err`
    pub fn path_err(&self, mesg: String, item: PathBuf) {
        println!("{}{} {}", item.as_str().paint(Color::Red),
        ":".paint(Color::Red), mesg.paint(Color::Red));
        self.exit();
    }

}
