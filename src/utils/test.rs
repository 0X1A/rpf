// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// GPL v3 license. For full terms please see the LICENSE file.

extern crate ansi_term;

use self::ansi_term::{Colour};
use utils::PathMod;
use utils::Styled;

use std::fs;
#[cfg(target_family = "unix")]
use std::os;
use std::fs::{PathExt};
use std::path::{Path};
#[cfg(target_family = "windows")]
use std::os::windows;

/// Removes an entire directory and all of its contents. Assumes the path 
/// given exists.
///
/// # Example
/// ```
/// use rpf::test;
///
/// test::remove("does_not_exist");
/// ```
pub fn remove<F: AsRef<Path>>(path: F) {
    if path.as_ref().is_dir() {
        fs::remove_dir_all(&path).ok();
        println!("Removed: {}", path.as_ref().as_str().paint(Colour::Red));
    } else {
        fs::remove_file(&path).ok();
        println!("Removed: {}", path.as_ref().as_str().paint(Colour::Red));
    }
}

/// Creates a file, assumes it has the correct permissions to write to the 
/// `path` given.
///
/// # Example
/// ```
/// use rpf::test;
///
/// test::create_file("file.txt");
/// test::remove("file.txt");
/// ```
pub fn create_file<F: AsRef<Path>>(path: F) {
    fs::File::create(path).ok();
}

/// Creates a directory, assumes it has the correct permissions to create a 
/// directory in the `path` given.
///
/// # Example
/// ```
/// use rpf::test;
///
/// test::create_dir("test_dir");
/// test::remove("test_dir");
/// ```
pub fn create_dir<F: AsRef<Path>>(path: F) {
    fs::create_dir(path).ok();
}

#[cfg(target_family = "unix")]
pub fn test_create_symlink<T: AsRef<Path>, F: AsRef<Path>>(to: &T, from: &F) {
    match os::unix::fs::symlink(from, to) {
        Ok(_) => {
            println!("{} symlinked to {}", from.as_ref().as_str().bold(),
                to.as_ref().as_str().bold());
        },
        Err(e) => {
            panic!("{}", e.to_string().paint(Colour::Red));
        }
    }
}
