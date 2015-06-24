// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// GPL v3 license. For full terms please see the LICENSE file.

use utils::Status;

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
    /// Name of rpfram
    pub name: &'static str,
    /// Version of rpfram
    pub vers: &'static str,
    /// Year of copyright for rpfram
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
    /// let rpf = Prog { name: "util", vers: "0.1.0", yr: "2015" };
    /// rpf.copyright("BSD-3-Clause", vec!["Author"]);
    /// ```
    ///
    /// Would print the following:
    ///
    /// ```bash
    /// util (core-utils) 0.1.0
    /// Copyright (C) 2015 core-utils developers
    /// License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.
    /// This is free software: you are free to change and redistribute it.
    /// There is NO WARRANTY, to the extent permitted by law.
    ///
    /// Written by Author
    /// ```
    pub fn copyright(&self, license: &str, auth: Vec<&str>) {
        match license {
            "Apache" => { },
            "BSD-3-Clause" => { },
            "GPL-2.0" => { },
            "GPL-3.0" => {
                print!("{} (core-utils) {}\n\
            Copyright (C) {} core-utils developers\n\
            License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>.\n\
            This is free software: you are free to change and redistribute it.\n\
            There is NO WARRANTY, to the extent permitted by law.\n\n", &self.name, &self.vers, &self.yr);
                print!("Written by ");
            },
            "MIT" => { },
            _ => { panic!("License not specified"); }
        }
        for pers in auth.iter() {
            print!("{} ", pers);
        }
        print!("\n");
    }

    /// Used for when no arguments are given to a util
    ///
    /// # Example
    ///
    /// ```should_panic
    /// use rpf::Prog;
    ///
    /// let rpf = Prog { name: "util", vers: "0.1.0", yr: "2015" };
    /// rpf.rpf_try();
    /// ```
    ///
    /// Would print the following:
    ///
    /// ```bash
    /// util: Missing arguments
    /// Try util --help for more information
    /// ```
    pub fn rpf_try(&self) {
        println!("{}: Missing arguments\n\
             Try '{} --help' for more information", &self.name, &self.name);
        Status::ArgError.exit();
    }
}
