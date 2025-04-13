use std::path::PathBuf;

use lsp_servers::lsp_servers::relevant_lsp_servers;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut settings = ::lsp_servers::settings::Settings::defaults();
    let mut target_path = None;

    if let Some(cwd) = match std::env::current_dir() {
        Ok(path) => Some(path),
        Err(e) => {
            println!("Failed to get current directory: {}", e);
            None
        }
    } {
        // Process all arguments
        for arg in &args[1..] {
            if arg.starts_with("-") {
                // This is a flag
                match arg.as_str() {
                    "-i" => settings.installed_only = true,
                    // Add more flag handling here as needed
                    _ => println!("Unknown flag: {}", arg),
                }
            } else {
                // This is likely a path
                match PathBuf::from(&arg).canonicalize() {
                    Ok(absolute_path) => {
                        target_path = Some(absolute_path);
                    }
                    Err(e) => {
                        println!("Failed to resolve path '{}': {}", arg, e);
                    }
                }
            }
        }

        // Use the specified path or default to current directory
        let target_dir = target_path.unwrap_or(cwd);
        let servers = relevant_lsp_servers(target_dir, &settings);
        for s in servers {
            println!("{}", s);
        }
    }
}
