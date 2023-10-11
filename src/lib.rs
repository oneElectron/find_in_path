//! This crate allows you to find a file in the PATH
//!
//! # Quickstart
//! Using this crate is very simple.
//! All you need to do is ```use find_in_path::prelude::*;``` or ```use find_in_path::FindInPath;``` to be able to find a ```String```, ```&str```, ```PathBuf```, ```&Path``` in the user's PATH.
//! ```
//! use find_in_path::prelude::*;
//! use std::path::PathBuf;
//!
//! let s: String = "sysctl".to_string();
//! let s_in_path: Option<PathBuf> = s.find_in_path();
//!
//! // The resultant path will be os and distribution specific
//! #[cfg(unix)]
//! assert_eq!(s_in_path, Some(PathBuf::from("/usr/sbin/sysctl")));
//! 
//! #[cfg(windows)]
//! assert_eq!(s_in_path, None);
//! ```
//! 
//! # Non-unicode files
//! find_in_path will support non-unicode paths through the OsString and &OsStr methods, however this remains unimplemented until a future release.
//!
//! # Security
//! By default ```FindInPath``` delays looking in relative directories found in the PATH until the end, because this may lead to unwanted executables being run.
//! You can stop FindInPath from checking relative directories altogether by setting the "skip_relative_directories" cargo feature.
//! Additionally, if the path to the file found in a relative path, the returned ```PathBuf``` will always be relative.
//!
//! # Other behavers
//! By default this crate assumes you are looking for an executable, which means that it will skip anything that is not an executable.
//! 
//! # Windows
//! On Windows you can activate the non-default ```add_exe_ext``` cargo feature in order to automatically add .exe to the end of filenames if there isn't one there already.
//! The feature will also not append ```.exe``` if the filename ends in ```.dll```.
//! ```toml
//! find_in_path = { version = "VERSION_NUMBER", features = ["add_exe_ext"] }
//! ```
//! Setting this feature on unix does nothing.
//!

pub mod prelude;

mod str;
mod string;
mod path;
mod pathbuf;

/// Allows finding in the PATH
///
/// Any type that implements this trait can be turned into a PathBuf to a valid executable found with the PATH environment variable.
pub trait FindInPath {
    fn find_in_path(&self) -> Option<std::path::PathBuf>;
}
