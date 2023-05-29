#[no_mangle]
pub extern "C" fn std_wrapper_exit(code: i32) {
    std::process::exit(code);
}
