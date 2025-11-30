use std::ffi::CStr;
use std::os::raw::c_char;

/// # Safety
/// The caller must ensure that `args` is a valid pointer to a null-terminated C string.
/// Passing an invalid pointer or a non-null-terminated string will result in undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn run_plugin(args_json: *const c_char) {
    let c_str = unsafe { CStr::from_ptr(args_json) };
    let args: Vec<String> = serde_json::from_str(c_str.to_str().unwrap()).unwrap_or_default();
    let output = args.join(" ");
    println!("{}", output);
}
