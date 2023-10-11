#![cfg(unix)]
use find_in_path::*;
use std::path::PathBuf;
use std::ffi::OsString;

struct CustomEnv {
    path: OsString,
}

impl CustomEnv {
    pub fn new(test_name: String) -> Self {
        let path = std::env::var_os("PATH").unwrap();
        std::env::set_var("PATH", format!("./:./test_resources/unix/{}/:/bin/", test_name));

        Self {
            path
        }
    }
}

impl Drop for CustomEnv {
    fn drop(&mut self) {
        std::env::set_var("PATH", self.path.clone());
    }
}

#[test]
#[cfg(feature = "exclude_relative_paths")]
fn exclude_relative_paths() {
    // assume /bin/ls exists 
    let path = "asdflkjqwerpoiu".to_string();
    let path = path.find_in_path();

    assert_eq!(path, None);
}

#[test]
#[cfg(not(feature = "exclude_relative_paths"))]
fn include_relative_paths() {
    let _env = CustomEnv::new("include_relative_paths".to_string());

    let path = "asdflkjqwerpoiu".to_string();
    let path = path.find_in_path();

    assert_eq!(path, Some(PathBuf::from("./test_resources/unix/include_relative_paths/asdflkjqwerpoiu")));
}

#[test]
fn non_executable() {
    let _env = CustomEnv::new("non_executable".to_string());

    let path = "asdflkjqwerpoiu".to_string();
    let path = path.find_in_path();

    assert_eq!(path, None);
}