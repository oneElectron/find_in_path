#![cfg(windows)]
use find_in_path::FindInPath;
use std::path::PathBuf;
use std::ffi::OsString;

struct CustomEnv {
    path: OsString,
}

impl CustomEnv {
    pub fn new(test_name: String) -> Self {
        let path = std::env::var_os("PATH").unwrap();
        std::env::set_var("PATH", format!(".\\;.\\test_resources\\windows\\{}\\;C:\\Windows\\system32\\", test_name));

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
fn basic() {
    // assume /bin/ls exists 
    let path = "notepad.exe".to_string();
    let path = path.find_in_path();

    assert_eq!(path, Some(PathBuf::from("C:\\Windows\\system32\\notepad.exe")));
}

#[test]
#[cfg(feature = "exclude_relative_paths")]
fn exclude_relative_paths() {
    let _env = CustomEnv::new("exclude_relative_paths".to_string());
    let path = "asdflkjqwerpoiu.exe".to_string();
    let path = path.find_in_path();

    assert_eq!(path, None);
}

#[test]
#[cfg(not(feature = "exclude_relative_paths"))]
fn include_relative_paths() {
    let _env = CustomEnv::new("include_relative_paths".to_string());
    let path = "asdflkjqwerpoiu.exe".to_string();
    let path = path.find_in_path();

    assert_eq!(path, Some(PathBuf::from("./test_resources/windows/include_relative_paths/asdflkjqwerpoiu.exe")));
}