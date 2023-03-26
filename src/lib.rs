#![allow(non_snake_case)]



#[no_mangle]
pub extern fn add00(left: i32, right: i32) -> i32 {
    //println!("into sideser!!!");
    return left + right;
}

#[no_mangle]
pub extern fn raa_string() -> *const u8{
    //"hell, O!\0".as_ptr()
    let retMsg = "NNNNN let's rust!";
    return retMsg.as_ptr();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add00(2, 2);
        assert_eq!(result, 4);
    }
}
