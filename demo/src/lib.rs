include!(concat!(env!("OUT_DIR"), "/json.rs"));

#[cfg(test)]
mod test {
    use super::is_valid;

    use std::ffi::CString;

    #[test]
    fn it_works() {
        let input_good = CString::new(r#"{ "hello": "world!" }"#).unwrap();
        let input_bad = CString::new(r#"{ "hello": "world!", "oops" }"#).unwrap();

        unsafe {
            assert!(is_valid(input_good.as_ptr()));
            assert!(!is_valid(input_bad.as_ptr()));
        }
    }
}
