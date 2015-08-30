// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

use std::path::{PathBuf,Path};

/// Adds some useful functions for manipulating and retrieving information from
/// paths
pub trait PathMod {
    /// Returns true if the path's file name starts with a "."
    ///
    /// # Example
    ///
    /// ```
    /// use rpf::PathMod;
    /// use std::path::Path;
    ///
    /// let path = Path::new("/test/dot/.dotfile");
    /// assert_eq!(path.is_dot(), true);
    fn is_dot(&self) -> bool;

    /// Returns a `PathBuf` of `&self`'s last component
    ///
    /// # Example
    ///
    /// ```
    /// use rpf::PathMod;
    /// use std::path::Path;
    ///
    /// let path = Path::new("/tmp/test/mod");
    /// let last = path.last_component().unwrap();
    /// ```
    fn last_component(&self) -> Option<PathBuf>;

    /// Returns a `PathBuf` of `&self`'s first component
    ///
    /// # Example
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("/tmp/test/mod");
    /// let first = path.first_component().unwrap();
    /// assert_eq!(first, PathBuf::from("/"));
    /// ```
    fn first_component(&self) -> Option<PathBuf>;

    /// Returns a `PathBuf` of `&self` relative from its parent
    ///
    /// # Example
    /// ```
    /// use rpf::PathMod;
    /// use std::path::PathBuf;
    ///
    /// let path = PathBuf::from("/tmp/test/mod");
    /// assert_eq!(PathBuf::from("mod"), path.rel_to_parent().unwrap());
    /// ```
    fn rel_to_parent(&self) -> Option<PathBuf>;

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
}

impl PathMod for PathBuf {
    fn is_dot(&self) -> bool {
        let file_name = match self.file_name() {
            Some(s) => {
                match s.to_str() {
                    Some(k) => { k },
                    None => { "" }
                }
            },
            None => { "" }
        };
        if file_name.starts_with(".") { return true; }
        else { return false; }
    }

    fn last_component(&self) -> Option<PathBuf> {
        match self.components().last() {
            Some(s) => { Some(PathBuf::from(s.as_os_str())) },
            None => { None },
        }
    }

    fn first_component(&self) -> Option<PathBuf> {
        match self.components().nth(0) {
            Some(s) => { Some(PathBuf::from(s.as_os_str())) },
            None => { None },
        }
    }

    fn rel_to_parent(&self) -> Option<PathBuf> {
        let parent = match self.parent() {
            Some(p) => { p },
            None => { return None }
        };
        match self.relative_from(parent) {
            Some(s) => { Some(PathBuf::from(s)) },
            None => { None }
        }
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
}

impl PathMod for Path {
    fn is_dot(&self) -> bool {
        let file_name = match self.file_name() {
            Some(s) => {
                match s.to_str() {
                    Some(k) => { k },
                    None => { "" }
                }
            },
            None => { "" }
        };
        if file_name.starts_with(".") { return true; }
        else { return false; }
    }

    fn last_component(&self) -> Option<PathBuf> {
        match self.components().last() {
            Some(s) => { Some(PathBuf::from(s.as_os_str())) },
            None => { None },
        }
    }

    fn first_component(&self) -> Option<PathBuf> {
        match self.components().nth(0) {
            Some(s) => { Some(PathBuf::from(s.as_os_str())) },
            None => { None },
        }
    }

    fn rel_to_parent(&self) -> Option<PathBuf> {
        let parent = match self.parent() {
            Some(p) => { p },
            None => { return None; }
        };
        match self.relative_from(parent) {
            Some(s) => { Some(PathBuf::from(s)) },
            None => { None }
        }
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
}

#[test]
fn test_pathmod_first_comp() {
    let comp = Path::new("/etc/test").first_component().unwrap();
    assert_eq!(comp, PathBuf::from("/"));
}

#[test]
fn test_pathmod_last_comp() {
    let comp = Path::new("/etc/test").last_component().unwrap();
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
fn test_pathmod_is_dot() {
    let path = Path::new("/dir/test/.test");
    assert_eq!(path.is_dot(), true);
}

#[test]
fn test_pathmod_rel_to_parent() {
    let path = PathBuf::from("/var/log/test");
    assert_eq!(PathBuf::from("test"), path.rel_to_parent().unwrap());
}
