//! This module contains the base implementation of FindInPath

use crate::prelude::*;
use std::path::{Path, PathBuf};

impl FindInPath for &str {
    fn find_in_path(&self) -> Option<std::path::PathBuf> {
        #[cfg(feature = "add_exe_ext")]
        #[cfg(windows)]
        let mut executable_name = self.to_string();

        #[cfg(feature = "add_exe_ext")]
        #[cfg(windows)]
        if !executable_name.ends_with(".exe") && !executable_name.ends_with(".dll") {
            executable_name.push_str(".exe");
        }

        #[cfg(not(feature = "exclude_relative_paths"))]
        let mut relative_paths: Vec<PathBuf> = vec![];

        let env_path = std::env::var_os("PATH");
        if env_path.is_none() {
            return None;
        }
        let env_path = env_path.unwrap();
        let env_path = env_path.to_string_lossy();

        let path_split: Vec<&str> = if cfg!(windows) {
            env_path.split(';')
        } else {
            env_path.split(':')
        }.collect();

        if path_split.is_empty() {
            return None;
        }

        for path in path_split {
            let mut path = PathBuf::from(path);

            // If path is relative save it for later
            if path.is_relative() {
                #[cfg(not(feature = "exclude_relative_paths"))]
                {
                    relative_paths.push(path);
                }
                continue;
            }

            #[cfg(feature = "add_exe_ext")]
            #[cfg(windows)]
            {
                path = path.join(executable_name.clone());
            } 
            
            #[cfg(any( not(feature = "add_exe_ext"), not(windows) ) )]
            {
                path = path.join(self);
            }

            if is_valid_executable(&path) {
                return Some(path);
            }
        }

        // Search the relative paths
        #[cfg(not(feature = "exclude_relative_paths"))]
        for path in relative_paths {
            let path = path.join(self);

            if is_valid_executable(&path) {
                return Some(path);
            }
        }

        None
    }
}

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
#[cfg(unix)]
fn is_valid_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    let metadata = std::fs::metadata(path);
    if metadata.is_err() {
        return false;
    }

    if metadata.unwrap().permissions().mode() & 0o111 == 0 {
        return false;
    }

    true
}

#[cfg(windows)]
fn is_valid_executable(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    true
}