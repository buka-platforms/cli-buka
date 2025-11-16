use clap::Parser;
use std::ffi::CStr;
use std::os::raw::c_char;

#[derive(Parser)]
#[command(about = "System plugin: prints system info")]
struct InfoCli {
    /// Show OS info
    #[arg(long)]
    os: bool,
}

/// # Safety
/// The caller must ensure that `args` is a valid pointer to a null-terminated C string.
/// Passing an invalid pointer or a non-null-terminated string will result in undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn run_plugin(args: *const c_char) {
    let json_str = CStr::from_ptr(args).to_string_lossy();
    let mut vec: Vec<String> = serde_json::from_str(&json_str).unwrap();
    vec.insert(0, "system".to_string());
    let cli = InfoCli::parse_from(vec);

    if cli.os {
        println!("OS: {}", std::env::consts::OS);
        println!("Arch: {}", std::env::consts::ARCH);
    }
    if !cli.os {
        println!("System plugin: use --os for details.");
    }
}
