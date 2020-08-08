// https://s3.amazonaws.com/temp.michaelfbryan.com/getting-started/index.html
// rust_print

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub unsafe extern "C" fn rust_print(msg: *const c_char) {
    let msg = CStr::from_ptr(msg);
    let as_str = msg.to_str().expect("The message is always valid UTF8");

    println!("{}", as_str);
}