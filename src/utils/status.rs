// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

use utils::PathMod;
use utils::Styled;
use utils::Color;

use std::process;
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

pub trait Exit {
    /// Wrapper for `process::exit`, immediately exits the process with the set
    /// exit status.
    ///
    /// # Example
    // Ignored here as doc-tests are ended because of `std::process::exit`
    /// ```ignore
    /// use rpf::{Prog,Exit,ExitStatus};
    ///
    /// let prog = Prog { name: "test", vers: "0.1.0", yr: "2015" };
    /// prog.exit(ExitStatus::Ok);
    /// ```
    fn exit(&self, status: ExitStatus);

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
    fn error(&self, mesg: String, status: ExitStatus);

    /// Used for errors when working with paths, works similar to `error`
    fn path_error(&self, mesg: String, item: PathBuf);
}

impl Exit for Prog {
    fn exit(&self, status: ExitStatus) {
        process::exit(status as i32);
    }

    fn error(&self, mesg: String, status: ExitStatus) {
        println!("{}{} {}", self.name.paint(Color::Red), ":".paint(Color::Red),
        mesg.paint(Color::Red));
        &self.exit(status);
    }

     fn path_error(&self, mesg: String, item: PathBuf) {
        println!("{}{} {}", item.as_str().paint(Color::Red),
        ":".paint(Color::Red), mesg.paint(Color::Red));
        &self.exit(ExitStatus::Error);
    }
}
