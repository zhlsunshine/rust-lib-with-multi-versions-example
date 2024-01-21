#[no_mangle]
pub fn my_rust_lib_v1(left: usize, right: usize) -> usize {
    println!("my_rust_lib_v1: {}", left + right);
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = my_rust_lib_v1(2, 2);
        assert_eq!(result, 4);
    }
}
