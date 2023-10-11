test:
    cargo test
    cargo test --features exclude_relative_paths
    cargo test --features add_exe_ext
    cargo test --features exclude_relative_paths,add_exe_ext