// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

use utils::ExitStatus;
use std::process;

/// A structure for storing rpfram information
///
/// # Example
///
/// ```
/// use rpf::Prog;
///
/// static UTIL: Prog = Prog { name: "util", vers: "0.1.0", yr: "2015" };
/// assert_eq!(UTIL.name, "util");
/// assert_eq!(UTIL.vers, "0.1.0");
/// assert_eq!(UTIL.yr, "2015");
/// ```
pub struct Prog {
    /// Name of program
    pub name: &'static str,
    /// Version of program
    pub vers: &'static str,
    /// Year of copyright for program
    pub yr: &'static str,
}

impl Prog {
    /// Prints copyright, version, and author information
    ///
    /// # Example
    ///
    /// ```
    /// use rpf::Prog;
    ///
    /// let prog = Prog { name: "util", vers: "0.1.0", yr: "2015" };
    /// prog.copyright(
    /// "Copyright (C) 2015 util developers\n\
    /// License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\n\
    /// This is free software: you are free to change and redistribute it.\n\
    /// There is NO WARRANTY, to the extent permitted by law.\n", &["Author"]);
    /// ```
    ///
    /// Would print the following:
    ///
    /// ```bash
    /// util 0.1.0
    /// Copyright (C) 2015 util developers
    /// License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
    /// This is free software: you are free to change and redistribute it.
    /// There is NO WARRANTY, to the extent permitted by law.
    ///
    /// Written by Author
    /// ```
    pub fn copyright(&self, license: &str, auth: &[&str]) {
        print!("{} {}\n{}", &self.name, &self.vers, license);
        print!("Written by ");
        for pers in auth.iter() {
            print!("{} ", pers);
        }
        print!("\n");
    }

    /// Used for when no arguments are given to a util
    ///
    /// # Example
    ///
    /// ```should panic
    /// use rpf::Prog;
    ///
    /// let prog = Prog { name: "util", vers: "0.1.0", yr: "2015" };
    /// prog.prog_try();
    /// ```
    ///
    /// Would print the following:
    ///
    /// ```bash
    /// util: Missing arguments
    /// Try util --help for more information
    /// ```
    pub fn prog_try(&self) {
        println!("{}: Missing arguments\n\
             Try '{} --help' for more information", &self.name, &self.name);
        &self.exit(ExitStatus::ArgError);
    }

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
    pub fn exit(&self, status: ExitStatus) {
        process::exit(status as i32);
    }

}

#[test]
fn test_prog() {
    let prog = Prog { name: "util", vers: "0.1.0", yr: "2015" };
    assert_eq!(prog.name, "util");
    assert_eq!(prog.vers, "0.1.0");
    assert_eq!(prog.yr, "2015");
}

#[test]
fn test_prog_copyright() {
    let prog = Prog { name: "util", vers: "0.1.0", yr: "2015" };
    prog.copyright(
    "Copyright (C) 2015 util developers\n\
    License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\n\
    This is free software: you are free to change and redistribute it.\n\
    There is NO WARRANTY, to the extent permitted by law.\n", &["Author"]);
}
