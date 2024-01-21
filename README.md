
# Rust example with multiple versions of a lib

## Directories Description

- `my-rust-bin` generate the binary which links to different versions of a lib named `my_rust_lib`, then call the function `pub fn my_rust_lib(left: usize, right: usize) -> usize`
- `my-rust-lib-v1` generate the lib named `my_rust_lib` in version `v1`
- `my-rust-lib-v2` generate the lib named `my_rust_lib` in version `v2`
- `lib` is the directory which contains the lib of `my_rust_lib` in any version

## Link lib and compile

### build binary `my-rust-bin` with version `v1` library `my_rust_lib`

1. Copy the library created in folder `my-rust-lib-v1` into `lib`
2. Build the binary in folder `my-rust-bin`
```
cd my-rust-bin
cargo build --features="v1"
```
3. Validate the binary
- find out the binary `my-rust-bin` built in folder `my-rust-bin`
- execute the binary via command `./my-rust-bin`，the output should be like below:
```
my_rust_lib_v1: 8
The return value of my_rust_lib is [8]
```

### build binary `my-rust-bin` with version `v2` library `my_rust_lib`

1. Copy the library created in folder `my-rust-lib-v2` into `lib`
2. Build the binary in folder `my-rust-bin`
```
cd my-rust-bin
cargo build --features="v2"
```
3. Validate the binary
- find out the binary `my-rust-bin` built in folder `my-rust-bin`
- execute the binary via command `./my-rust-bin`，the output should be like below:
```
my_rust_lib_v2: 8
The return value of my_rust_lib is [8]
```
