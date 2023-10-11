use crate::prelude::*;

impl FindInPath for String {
    fn find_in_path(&self) -> Option<std::path::PathBuf> {
        self.as_str().find_in_path()
    }
}
