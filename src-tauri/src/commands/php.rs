use serde::{Deserialize, Serialize};
use std::process::Command;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct PhpVersion {
    pub version: String,
    pub installed: bool,
    pub active: bool,
    pub path: String,
}

#[tauri::command]
pub fn get_php_versions() -> Result<Vec<PhpVersion>, String> {
    let versions = vec!["8.3", "8.2", "8.1", "8.0", "7.4"];
    let mut result = Vec::new();

    // Get current PHP version
    let current_version = Command::new("php")
        .arg("-v")
        .output()
        .ok()
        .and_then(|o| {
            let output = String::from_utf8_lossy(&o.stdout);
            // Parse "PHP 8.3.0" to get "8.3"
            output.lines().next().and_then(|line| {
                line.split_whitespace().nth(1).map(|v| {
                    let parts: Vec<&str> = v.split('.').collect();
                    if parts.len() >= 2 {
                        format!("{}.{}", parts[0], parts[1])
                    } else {
                        v.to_string()
                    }
                })
            })
        });

    for version in versions {
        let php_path = format!("/usr/bin/php{}", version);
        let installed = Path::new(&php_path).exists();
        let active = current_version.as_deref() == Some(version);

        result.push(PhpVersion {
            version: version.to_string(),
            installed,
            active,
            path: if installed { php_path } else { String::new() },
        });
    }

    Ok(result)
}

#[tauri::command]
pub fn switch_php_version(version: String) -> Result<(), String> {
    // This uses update-alternatives on Debian/Ubuntu systems
    // For other distros, different methods would be needed

    let output = Command::new("pkexec")
        .args(["update-alternatives", "--set", "php", &format!("/usr/bin/php{}", version)])
        .output()
        .map_err(|e| format!("Failed to switch PHP version: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
