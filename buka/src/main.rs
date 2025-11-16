use clap::{CommandFactory, Parser};
use libloading::{Library, Symbol};
use std::ffi::CString;

#[derive(Parser)]
#[command(
    name = "buka",
    about = "Buka CLI is a general-purpose plugin-based CLI.",
    version = env!("CARGO_PKG_VERSION"),
    disable_help_flag = true,
    disable_help_subcommand = true
)]
struct Cli {
    /// Plugin name (e.g., hello)
    plugin: Option<String>,

    /// Arguments forwarded to plugin
    #[arg(allow_hyphen_values = true, trailing_var_arg = true)]
    args: Vec<String>,
}

fn get_library_filename(plugin: &str) -> String {
    if cfg!(target_os = "windows") {
        format!("{}.dll", plugin)
    } else if cfg!(target_os = "macos") {
        format!("lib{}.dylib", plugin)
    } else {
        format!("lib{}.so", plugin)
    }
}

fn main() {
    // Get the raw arguments, skipping the binary name
    let mut raw_args = std::env::args().skip(1);

    // If the first argument is --help or -h, print main app help
    if let Some(arg) = raw_args.next() {
        if arg == "--help" || arg == "-h" {
            Cli::command().print_help().unwrap();
            println!();
            return;
        }
    } else {
        // No arguments, print main app help
        Cli::command().print_help().unwrap();
        println!();
        return;
    }

    let cli = Cli::parse();

    if let Some(plugin) = cli.plugin {
        // Forward all args (including --help) to the plugin
        let lib_path = format!("dist_plugins/{}", get_library_filename(&plugin));
        unsafe {
            let lib = Library::new(lib_path).expect("Error: Failed to load plugin library.");
            let func: Symbol<unsafe extern "C" fn(*const std::os::raw::c_char)> = lib
                .get(b"run_plugin")
                .expect("Error: Plugin does not export function run_plugin");
            let json = serde_json::to_string(&cli.args).unwrap();
            let c_args = CString::new(json).unwrap();
            func(c_args.as_ptr());
        }
    } else {
        // No plugin specified, print main app help
        Cli::command().print_help().unwrap();
        println!();
    }
}
