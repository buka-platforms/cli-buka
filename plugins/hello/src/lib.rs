use clap::Parser;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Parser)]
#[command(name = "hello")]
#[command(about = "Hello plugin")]
struct HelloCli {
    #[arg(short, long, default_value = "World")]
    name: String,
}

/// # Safety
///
/// The caller must ensure that `args` is a valid pointer to a null-terminated C string.
/// Passing an invalid pointer or a non-null-terminated string will result in undefined behavior.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn run_plugin(args: *const c_char) {
    // Receive JSON array of args
    let json_str = unsafe { CStr::from_ptr(args).to_string_lossy() };
    let vec: Vec<String> = serde_json::from_str(&json_str).unwrap();

    // Let Clap parse the forwarded arguments
    let cli = HelloCli::parse_from(vec);

    println!("Hello, {}!", cli.name);
}
