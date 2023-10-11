//! Contains the FindInPath implementation for PathBuf

use crate::prelude::*;
use std::path::PathBuf;

impl FindInPath for PathBuf {
    fn find_in_path(&self) -> Option<std::path::PathBuf> {
        self.as_path().find_in_path()
    }
}