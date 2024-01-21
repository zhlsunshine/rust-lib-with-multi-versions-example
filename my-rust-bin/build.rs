fn main() {
    println!("cargo:rustc-link-lib=static=my_rust_lib");
    println!("cargo:rustc-link-search=native=./../lib");
}
