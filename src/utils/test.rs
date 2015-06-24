// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

use utils::PathMod;
use utils::Styled;
use utils::Color;

use std::fs;
#[cfg(target_family = "unix")]
use std::os;
use std::fs::{PathExt};
use std::path::{Path};
#[cfg(target_family = "windows")]
use std::os::windows;

/// Removes a directory or file and all of its contents.
///
/// # Example
/// ```
/// use rpf::test;
///
/// test::remove("does_not_exist");
/// ```
pub fn remove<F: AsRef<Path>>(path: F) {
    if path.as_ref().is_dir() {
        match fs::remove_dir_all(&path) {
            Ok(_) => { println!("test: removed directory '{}'",
                                path.as_ref().as_str().paint(Color::Green)) },
            Err(_) => { println!("test: unable to remove directory '{}'",
                                 path.as_ref().as_str().paint(Color::Red)) },
        }
    } else {
        match fs::remove_file(&path) {
            Ok(_) => { println!("test: removed file '{}'",
                                path.as_ref().as_str().paint(Color::Green)) },
            Err(_) => { println!("test: unable to remove file '{}'",
                                 path.as_ref().as_str().paint(Color::Red)) },
        }
    }
}

/// Creates a file
///
/// # Example
/// ```
/// use rpf::test;
///
/// test::create_file("file.txt");
/// test::remove("file.txt");
/// ```
pub fn create_file<F: AsRef<Path>>(path: F) {
    match fs::File::create(&path) {
        Ok(_) => { println!("test: created file '{}'",
                            path.as_ref().as_str().paint(Color::Green)) },
        Err(_) => { println!("test: unable to crate file '{}'",
                            path.as_ref().as_str().paint(Color::Red)) },
    }
}

/// Creates a directory
///
/// # Example
/// ```
/// use rpf::test;
///
/// test::create_dir("test_dir");
/// test::remove("test_dir");
/// ```
pub fn create_dir<F: AsRef<Path>>(path: F) {
    match fs::create_dir(&path) {
        Ok(_) => { println!("test: created directory '{}'",
                            path.as_ref().as_str().paint(Color::Green)) },
        Err(_) => { println!("test: unable to crate directory '{}'",
                            path.as_ref().as_str().paint(Color::Red)) },
    }
}

#[cfg(target_family = "unix")]
pub fn test_create_symlink<T: AsRef<Path>, F: AsRef<Path>>(to: &T, from: &F) {
    match os::unix::fs::symlink(from, to) {
        Ok(_) => {
            println!("{} symlinked to {}", from.as_ref().as_str().bold(),
                to.as_ref().as_str().bold());
        },
        Err(e) => {
            panic!("{}", e.to_string().paint(Color::Red));
        }
    }
}

#[test]
fn test_test_remove() {
    remove("does_not_exist");
}

#[test]
fn test_test_create_file() {
    create_file("test-file");
    remove("test-file");
}

#[test]
fn test_test_create_dir() {
    create_dir("test-dir");
    remove("test-dir");
}
