// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// GPL v3 license. For full terms please see the LICENSE file.

use std::path::{PathBuf,Path};
use std::fs::{PathExt};

/// Adds functions for retrieving first and last components as well as getting
/// a relative path
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
    ///
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
    ///
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("mod");
    /// let parent = PathBuf::from("/tmp/test/mod");
    /// let rel = path.rel_to(&parent);
    /// ```
    fn rel_to<T: AsRef<Path>>(&self, rel_from: &T) -> PathBuf;

    /// Returns a string for an path, returns a blank string if not able to
    /// get a string for the path
    ///
    /// # Example
    ///
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("/usr/share");
    /// let path_str = &path.as_str();
    /// assert_eq!(path_str, &"/usr/share");
    /// ```
    fn as_str(&self) -> &str;

    fn as_string(&self) -> String;

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
        if self.is_dir() || self.is_file() {
            return false
        }
        return true
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
        if self.is_dir() || self.is_file() {
            return false
        }
        return true
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
