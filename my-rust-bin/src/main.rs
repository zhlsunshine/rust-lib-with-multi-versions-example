#[cfg(feature = "v1")]
mod bindingmylib {
    extern "C" {
        pub fn my_rust_lib_v1(left: usize, right: usize) -> usize;
    }
}

#[cfg(feature = "v2")]
mod bindingmylib {
    extern "C" {
        pub fn my_rust_lib_v2(left: usize, right: usize) -> usize;
    }
}

#[cfg(not(any(feature = "v1", feature = "v2")))]
compile_error!("Please specify either 'v1' or 'v2' feature");

pub fn my_rust_lib(left: usize, right: usize) -> usize {
    #[cfg(feature = "v1")]
    unsafe {
        return bindingmylib::my_rust_lib_v1(left, right);
    }

    #[cfg(feature = "v2")]
    unsafe {
        return bindingmylib::my_rust_lib_v2(left, right);
    }
}

fn main() {
    let r_value: usize = my_rust_lib(3, 5);
    println!("The return value of my_rust_lib is [{}]", r_value);
}
