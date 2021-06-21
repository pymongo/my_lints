# How to use my_lints to static analysis other rust project

export MY_LINTS_PATH=/home/w/repos/my_repos/my_lints

## method_1: In my_lints dir

> cd $MY_LINTS_PATH
> 
> cargo clean && cargo b && cargo dylint --all -- --manifest-path=/home/w/temp/other_rust_project/Cargo.toml

## method_2: In other Rust project's CARGO_MANIFEST_DIR

> DYLINT_LIBRARY_PATH=$MY_LINTS_PATH/target/debug cargo dylint --all
