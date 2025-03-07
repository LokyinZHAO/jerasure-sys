pub mod gf_complete;
pub mod jerasure;

#[cfg(test)]
mod test {
    use crate::jerasure;

    #[test]
    fn test_multiply() {
        unsafe {
            assert_eq!(jerasure::galois_single_multiply(48, 18, 8), 71_i32);
        }
    }
}
