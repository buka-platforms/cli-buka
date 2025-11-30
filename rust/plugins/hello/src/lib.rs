use clap::Parser;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Parser)]
#[command(name = "hello")]
#[command(about = "Plugin 'hello': prints a greeting message")]
#[command(version = env!("CARGO_PKG_VERSION"))]
struct HelloCli {
    /// Name to greet
    #[arg(short, long, default_value = "World")]
    name: String,
}

/// # Safety
/// The caller must ensure that `args` is a valid pointer to a null-terminated C string.
/// Passing an invalid pointer or a non-null-terminated string will result in undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn run_plugin(args: *const c_char) {
    // Receive JSON array of args
    let json_str = CStr::from_ptr(args).to_string_lossy();
    let mut vec: Vec<String> = serde_json::from_str(&json_str).unwrap();

    // Prepend a dummy binary name for clap
    vec.insert(0, "hello".to_string());

    // Let Clap parse the forwarded arguments
    let cli = HelloCli::parse_from(vec);

    println!("Hello, {}!", cli.name);
}
