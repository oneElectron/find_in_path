//! Contains the FindInPath implementation for &Path 

use crate::prelude::*;
use std::path::Path;

impl FindInPath for &Path {
    fn find_in_path(&self) -> Option<std::path::PathBuf> {
        self.to_string_lossy().as_ref().find_in_path()
    }
}