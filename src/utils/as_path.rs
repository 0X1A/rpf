// Copyright (C) 2015, Alberto Corona <alberto@0x1a.us>
// All rights reserved. This file is part of rpf, distributed under the
// GPL v3 license. For full terms please see the LICENSE file.

use std::path::{PathBuf,Path};

/// A trait for treating String and str as Path and PathBuf
pub trait AsPath {
    /// Returns a borrowed `Path`
    ///
    fn as_path(&self) -> &Path;
    /// Returns a `PathBuf`
    ///
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
