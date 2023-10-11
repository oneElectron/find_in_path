
fn main() {
    if cfg!(not(unix)) && cfg!(not(windows)) {
        panic!("Unsupported platform");
    }
}