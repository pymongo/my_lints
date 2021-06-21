# How to use my_lints to static analysis other rust project

export MY_LINTS_PATH=/home/w/repos/my_repos/my_lints

## method_1: In my_lints dir

> cd $MY_LINTS_PATH
> 
> cargo clean && cargo b && cargo dylint --all -- --manifest-path=/home/w/temp/other_rust_project/Cargo.toml

## method_2: In other Rust project's CARGO_MANIFEST_DIR

> DYLINT_LIBRARY_PATH=$MY_LINTS_PATH/target/debug cargo dylint --all

## Roadmap

### TODO lints

- [ ] check f32 cast to f64 and warn precession lost(clippy::pedantic has `f64 as f32` checking)

### enhance lints

- [ ] my_lint_recursive_code: detect fn a call b() and fn b call a() cause infinite recursive
