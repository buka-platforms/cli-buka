use clap::{Parser, Subcommand};
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(name = "plugin", about = "Plugin management commands.")]
pub struct PluginCli {
    #[command(subcommand)]
    pub command: PluginCommand,
}

#[derive(Subcommand)]
pub enum PluginCommand {
    /// List all available plugins
    List,
    /// Show info about a specific plugin
    Info {
        /// Plugin name
        name: String,
    },
}

/// # Safety
/// The caller must ensure that `args` is a valid pointer to a null-terminated C string.
/// Passing an invalid pointer or a non-null-terminated string will result in undefined behavior.
#[no_mangle]
pub unsafe extern "C" fn run_plugin(args_json: *const std::os::raw::c_char) {
    use std::ffi::CStr;
    let c_str = unsafe { CStr::from_ptr(args_json) };
    let args: Vec<String> = serde_json::from_str(c_str.to_str().unwrap()).unwrap();
    let cli =
        PluginCli::parse_from(std::iter::once("plugin").chain(args.iter().map(|s| s.as_str())));
    match &cli.command {
        PluginCommand::List => list_plugins(),
        PluginCommand::Info { name } => info_plugin(name),
    }
}

fn list_plugins() {
    use libloading::Library;
    use std::env;
    use std::ffi::CStr;

    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let plugins_dir = exe_dir.join("dist_plugins");
    if let Ok(entries) = fs::read_dir(&plugins_dir) {
        println!("Available plugins:");
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_stem().and_then(|n| n.to_str()) {
                if path.extension().and_then(|e| e.to_str()) == Some("dll") {
                    let description = unsafe {
                        match Library::new(&path) {
                            Ok(lib) => match lib
                                .get::<unsafe extern "C" fn() -> *const std::os::raw::c_char>(
                                    b"get_plugin_description",
                                ) {
                                Ok(f) => {
                                    let c_str = CStr::from_ptr(f());
                                    c_str.to_string_lossy().into_owned()
                                }
                                Err(e) => {
                                    eprintln!("Failed to load symbol from {}: {}", name, e);
                                    "[no description]".to_string()
                                }
                            },
                            Err(e) => {
                                eprintln!("Failed to load library {}: {}", name, e);
                                "[no description]".to_string()
                            }
                        }
                    };
                    println!("- {}, {}", name, description);
                }
            }
        }
    } else {
        println!("No plugins directory found at {:?}", plugins_dir);
    }
}

fn info_plugin(name: &str) {
    use libloading::{Library, Symbol};
    use std::env;
    use std::ffi::CStr;

    let exe_dir = env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let plugin_path = exe_dir.join("dist_plugins").join(format!("{}.dll", name));
    if plugin_path.exists() {
        let description = unsafe {
            Library::new(&plugin_path)
                .ok()
                .and_then(|lib| {
                    let func: Result<
                        Symbol<unsafe extern "C" fn() -> *const std::os::raw::c_char>,
                        libloading::Error,
                    > = lib.get(b"get_plugin_description");
                    func.ok().map(|f| {
                        let c_str = CStr::from_ptr(f());
                        c_str.to_string_lossy().into_owned()
                    })
                })
                .unwrap_or_else(|| "[no description]".to_string())
        };
        println!("Info for plugin: {}\nDescription: {}", name, description);
    } else {
        println!("Plugin '{}' not found.", name);
    }
}

#[no_mangle]
pub extern "C" fn get_plugin_description() -> *const std::os::raw::c_char {
    std::ffi::CString::new("Manages and displays information about plugins.")
        .unwrap()
        .into_raw()
}
