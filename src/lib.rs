pub mod gf_complete;
pub mod jerasure;

#[cfg(test)]
mod test {
    use crate::{gf_complete, jerasure};

    #[test]
    fn test_jerasure() {
        unsafe {
            assert_eq!(jerasure::galois_single_multiply(48, 18, 8), 71_i32);
        }
    }

    #[test]
    fn test_gf() {
        let mut gf_8: std::mem::MaybeUninit<gf_complete::gf_t> = std::mem::MaybeUninit::uninit();
        unsafe {
            if gf_complete::gf_init_easy(gf_8.as_mut_ptr(), 8) == 0 {
                panic!("gf_init_easy failed");
            }
            let mut gf_8 = gf_8.assume_init();
            let gf_8_ptr = &mut gf_8 as *mut gf_complete::gf_t;
            let mult = &gf_8.multiply.w32.unwrap();
            let res = mult(gf_8_ptr, 48, 18);
            assert_eq!(res, 71_u32);
        }
    }

    #[test]
    fn test_transmute() {
        // is it safe to transmute between mudule types
        let mut gf_8: std::mem::MaybeUninit<gf_complete::gf_t> = std::mem::MaybeUninit::uninit();
        unsafe {
            // jerasure use gf_t from gf_complete
            if gf_complete::gf_init_easy(gf_8.as_mut_ptr(), 8) == 0 {
                panic!("gf_init_easy failed");
            }
            let gf_8 = gf_8.assume_init();
            let mut jr_8_from_gf: jerasure::gf = std::mem::transmute(gf_8);
            let mult = &jr_8_from_gf.multiply.w32.unwrap();
            let ptr = &mut jr_8_from_gf as *mut jerasure::gf;
            let res = mult(ptr, 48, 18);
            assert_eq!(res, 71_u32);
        }
    }
}
