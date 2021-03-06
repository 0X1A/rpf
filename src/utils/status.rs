// Copyright (C) 2015, Alberto Corona <ac@albertocorona.com>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

use utils::PathMod;
use utils::Styled;
use utils::Color;

use std::path::PathBuf;

use utils::Prog;

/// Enum used for setting exit statuses
#[derive(Copy,Clone)]
pub enum ExitStatus {
    Ok,
    Error,
    OptError,
    ArgError,
}

/// Causes a `Prog` struct to exit using an error and exit status
pub trait Exit<T: AsRef<str>> {
    /// Used for errors, prints error messages in red terminal font and calls
    /// `rpf::Exit::exit`
    ///
    /// # Example
    // Ignored here as doc-tests are ended because of `std::process::exit`
    /// ```ignore
    /// use rpf::{Prog,Exit,ExitStatus};
    ///
    /// let prog = Prog { name: "test", vers: "0.1.0", yr: "2015" };
    /// prog.error("Some kind of error occured!".to_string(), ExitStatus::Error);
    /// ```
    fn error(&self, mesg: T, status: ExitStatus);

    /// Used for errors when working with paths, works similar to `error`
    fn path_error(&self, mesg: T, item: PathBuf);
}

impl <T: AsRef<str>> Exit<T> for Prog {
    fn error(&self, mesg: T, status: ExitStatus) {
        println!("{}{} {}", self.name.paint(Color::Red), ":".paint(Color::Red),
        mesg.as_ref().paint(Color::Red));
        &self.exit(status);
    }

     fn path_error(&self, mesg: T, item: PathBuf) {
        println!("{}{} {}", item.as_str().paint(Color::Red),
        ":".paint(Color::Red), mesg.as_ref().paint(Color::Red));
        &self.exit(ExitStatus::Error);
    }
}
