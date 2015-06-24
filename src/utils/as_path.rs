// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// BSD 3-Clause license. For full terms please see the LICENSE file.

use std::path::{PathBuf,Path};

/// A trait for treating String and str as `Path` and `PathBuf`
pub trait AsPath {
    /// Returns a borrowed `Path`
    ///
    /// # Example
    /// ```
    /// use rpf::AsPath;
    /// use std::path::Path;
    ///
    /// let path = "/etc/test/string".as_path().clone();
    /// assert_eq!(path, Path::new("/etc/test/string"));
    /// ```
    fn as_path(&self) -> &Path;

    /// Returns a `PathBuf`
    ///
    /// # Example
    /// ```
    /// use rpf::AsPath;
    /// use std::path::PathBuf;
    ///
    /// let pathbuf = "/var/log/test".as_pathbuf();
    /// assert_eq!(pathbuf, PathBuf::from("/var/log/test"));
    /// ```
    fn as_pathbuf(&self) -> PathBuf;
}

impl AsPath for String {
    fn as_path(&self) -> &Path {
        Path::new(self)
    }

    fn as_pathbuf(&self) -> PathBuf {
        PathBuf::from(self)
    }
}

impl AsPath for str {
    fn as_path(&self) -> &Path {
        Path::new(self)
    }

    fn as_pathbuf(&self) -> PathBuf {
        PathBuf::from(self)
    }
}

#[test]
fn test_as_path_string() {
    assert_eq!("/etc/test/dir".to_string().as_path().clone(),
        Path::new("/etc/test/dir"));
}

#[test]
fn test_as_pathbuf_string() {
    assert_eq!("/etc/test/dir".to_string().as_pathbuf(),
        PathBuf::from("/etc/test/dir"));
}

#[test]
fn test_as_path_str() {
    assert_eq!("/etc/test/dir".as_path().clone(),
        Path::new("/etc/test/dir"));
}

#[test]
fn test_as_pathbuf_str() {
    assert_eq!("/etc/test/dir".as_pathbuf(),
        PathBuf::from("/etc/test/dir"));
}
