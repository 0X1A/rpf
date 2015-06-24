// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

use std::path::{PathBuf,Path};
use std::fs::{PathExt};

/// Adds some useful functions for manipulating and retrieving information from
/// paths
pub trait PathMod {
    /// Returns a `PathBuf` of `&self`'s last component
    ///
    /// # Example
    ///
    /// ```
    /// use rpf::PathMod;
    /// use std::path::Path;
    ///
    /// let path = Path::new("/tmp/test/mod");
    /// let last = path.last_component();
    /// ```
    fn last_component(&self) -> PathBuf;

    /// Returns a `PathBuf` of `&self`'s first component
    ///
    /// # Example
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("/tmp/test/mod");
    /// let first = path.first_component();
    /// assert_eq!(first, PathBuf::from("/"));
    /// ```
    fn first_component(&self) -> PathBuf;

    /// Returns a `PathBuf` of `&self` relative from `rel_from`
    ///
    /// # Example
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("mod");
    /// let parent = PathBuf::from("/tmp/test/mod");
    /// let rel = path.rel_to(&parent);
    /// ```
    fn rel_to<T: AsRef<Path>>(&self, rel_from: &T) -> PathBuf;

    /// Returns a `&str` for a path, returns a blank string if unable to
    /// get a string for the path
    ///
    /// # Example
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("/usr/share");
    /// let path_str = &path.as_str();
    /// assert_eq!(path_str, &"/usr/share");
    /// ```
    fn as_str(&self) -> &str;

    /// Returns a `String` for a path, returns an empty string if unable to get
    /// a string for a path
    ///
    /// # Example
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path_string = PathBuf::from("/var/log/test").as_string();
    /// assert_eq!(path_string, "/var/log/test".to_string());
    /// ```
    fn as_string(&self) -> String;

    /// Returns true if a given path is a symbolic link, returns false if the
    /// path is a regular file, a directory, or does not exist
    ///
    /// # Example
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("test-symlink");
    /// assert_eq!(path.is_symlink(), false);
    /// ```
    fn is_symlink(&self) -> bool;
}

impl PathMod for PathBuf {
    fn last_component(&self) -> PathBuf {
        match self.components().last() {
            Some(s) => { PathBuf::from(s.as_os_str()) },
            None => { PathBuf::new() },
        }
    }

    fn first_component(&self) -> PathBuf {
        match self.components().nth(0) {
            Some(s) => { PathBuf::from(s.as_os_str()) },
            None => { PathBuf::new() },
        }
    }

    fn rel_to<T: AsRef<Path>>(&self, rel_from: &T) -> PathBuf {
        self.relative_from(&rel_from).unwrap_or(&PathBuf::new()).to_path_buf()
    }

    fn as_str(&self) -> &str {
        match self.to_str() {
            Some(s) => { s },
            None => { "" },
        }
    }

    fn as_string(&self) -> String {
        self.as_str().to_string()
    }

    fn is_symlink(&self) -> bool {
        if self.exists() {
            if self.is_dir() || self.is_file() {
                return false
            }
            return true
        } else { return false }
    }
}

impl PathMod for Path {
    fn last_component(&self) -> PathBuf {
        match self.components().last() {
            Some(s) => { PathBuf::from(s.as_os_str()) },
            None => { PathBuf::new() },
        }
    }

    fn first_component(&self) -> PathBuf {
        match self.components().nth(0) {
            Some(s) => { PathBuf::from(s.as_os_str()) },
            None => { PathBuf::new() },
        }
    }

    fn rel_to<T: AsRef<Path>>(&self, rel_from: &T) -> PathBuf {
        self.relative_from(&rel_from).unwrap_or(&PathBuf::new()).to_path_buf()
    }

    fn as_str(&self) -> &str {
        match self.to_str() {
            Some(s) => { s },
            None => { "" },
        }
    }

    fn as_string(&self) -> String {
        self.as_str().to_string()
    }

    fn is_symlink(&self) -> bool {
        if self.exists() {
            if self.is_dir() || self.is_file() {
                return false
            }
            return true
        } else { return false }
    }
}

#[test]
fn test_pathmod_first_comp() {
    let comp = Path::new("/etc/test").first_component();
    assert_eq!(comp, PathBuf::from("/"));
}

#[test]
fn test_pathmod_last_comp() {
    let comp = Path::new("/etc/test").last_component();
    assert_eq!(comp, PathBuf::from("test"));
}

#[test]
fn test_pathmod_as_str() {
    let string = Path::new("/var/log/test").as_str();
    assert_eq!(string, "/var/log/test");
}

#[test]
fn test_pathmod_as_string() {
    let string = Path::new("/var/log/test").as_string();
    assert_eq!(string, "/var/log/test".to_string());
}

#[test]
fn test_pathmod_is_symlink() {
    let path = Path::new("/var/log/test");
    assert_eq!(path.is_symlink(), false);
}
