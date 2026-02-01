use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhpVersion {
    pub version: String,
    pub full_version: String,
    pub installed: bool,
    pub active: bool,
    pub path: String,
}

/// Get all PHP versions installed on the system
#[tauri::command]
pub fn get_php_versions() -> Result<Vec<PhpVersion>, String> {
    let known_versions = vec!["8.4", "8.3", "8.2", "8.1", "8.0", "7.4", "7.3", "7.2"];
    let search_paths = vec!["/usr/bin", "/usr/local/bin", "/opt/php"];

    let mut result = Vec::new();
    let mut found_versions: Vec<String> = Vec::new();

    // Get current active PHP version
    let (active_version, active_full) = get_active_php_version();

    // Method 1: Check known versions in standard paths
    for version in &known_versions {
        for base_path in &search_paths {
            let php_path = format!("{}/php{}", base_path, version);

            if Path::new(&php_path).exists() && !found_versions.contains(&version.to_string()) {
                let full_version = get_php_full_version(&php_path);
                let is_active = active_version.as_deref() == Some(*version);

                result.push(PhpVersion {
                    version: version.to_string(),
                    full_version: full_version.unwrap_or_default(),
                    installed: true,
                    active: is_active,
                    path: php_path,
                });
                found_versions.push(version.to_string());
            }
        }
    }

    // Method 2: Scan /usr/bin for php* binaries we might have missed
    if let Ok(entries) = fs::read_dir("/usr/bin") {
        for entry in entries.filter_map(|e| e.ok()) {
            let filename = entry.file_name().to_string_lossy().to_string();

            // Match php7.x or php8.x patterns
            if filename.starts_with("php") && filename.len() > 3 {
                let version_part = &filename[3..];

                // Check if it looks like a version (e.g., "8.3" or "7.4")
                if version_part.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false)
                   && !found_versions.contains(&version_part.to_string())
                {
                    let php_path = format!("/usr/bin/{}", filename);
                    let full_version = get_php_full_version(&php_path);
                    let is_active = active_version.as_deref() == Some(version_part);

                    result.push(PhpVersion {
                        version: version_part.to_string(),
                        full_version: full_version.unwrap_or_default(),
                        installed: true,
                        active: is_active,
                        path: php_path,
                    });
                    found_versions.push(version_part.to_string());
                }
            }
        }
    }

    // Add known versions that are not installed
    for version in &known_versions {
        if !found_versions.contains(&version.to_string()) {
            result.push(PhpVersion {
                version: version.to_string(),
                full_version: String::new(),
                installed: false,
                active: false,
                path: String::new(),
            });
        }
    }

    // Sort by version (descending)
    result.sort_by(|a, b| {
        let a_parts: Vec<u32> = a.version.split('.').filter_map(|s| s.parse().ok()).collect();
        let b_parts: Vec<u32> = b.version.split('.').filter_map(|s| s.parse().ok()).collect();
        b_parts.cmp(&a_parts)
    });

    Ok(result)
}

/// Get the active PHP version from the default 'php' command
fn get_active_php_version() -> (Option<String>, Option<String>) {
    Command::new("php")
        .arg("-v")
        .output()
        .ok()
        .and_then(|o| {
            if !o.status.success() {
                return None;
            }
            let output = String::from_utf8_lossy(&o.stdout);
            output.lines().next().and_then(|line| {
                // Parse "PHP 8.3.0 (cli) ..." to get version
                line.split_whitespace().nth(1).map(|full_v| {
                    let parts: Vec<&str> = full_v.split('.').collect();
                    let short = if parts.len() >= 2 {
                        format!("{}.{}", parts[0], parts[1])
                    } else {
                        full_v.to_string()
                    };
                    (Some(short), Some(full_v.to_string()))
                })
            })
        })
        .unwrap_or((None, None))
}

/// Get the full version string from a specific PHP binary
fn get_php_full_version(php_path: &str) -> Option<String> {
    Command::new(php_path)
        .arg("-v")
        .output()
        .ok()
        .and_then(|o| {
            if !o.status.success() {
                return None;
            }
            let output = String::from_utf8_lossy(&o.stdout);
            output.lines().next().and_then(|line| {
                line.split_whitespace().nth(1).map(|v| v.to_string())
            })
        })
}

/// Switch the active PHP version using update-alternatives (Debian/Ubuntu)
#[tauri::command]
pub fn switch_php_version(version: String) -> Result<(), String> {
    let php_path = format!("/usr/bin/php{}", version);

    // Verify the version exists
    if !Path::new(&php_path).exists() {
        return Err(format!("PHP {} is not installed", version));
    }

    // Use pkexec for privilege escalation
    let output = Command::new("pkexec")
        .args(["update-alternatives", "--set", "php", &php_path])
        .output()
        .map_err(|e| format!("Failed to switch PHP version: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        if stderr.contains("no alternatives") {
            // Try manual symlink approach
            let output2 = Command::new("pkexec")
                .args(["ln", "-sf", &php_path, "/usr/bin/php"])
                .output()
                .map_err(|e| format!("Failed to create symlink: {}", e))?;

            if output2.status.success() {
                Ok(())
            } else {
                Err(String::from_utf8_lossy(&output2.stderr).to_string())
            }
        } else {
            Err(stderr.to_string())
        }
    }
}

/// Install a PHP version (requires appropriate package manager)
#[tauri::command]
pub fn install_php_version(version: String, package_manager: String) -> Result<String, String> {
    let (cmd, args) = match package_manager.as_str() {
        "apt" => {
            // For Ubuntu/Debian, we need the ondrej/php PPA
            ("pkexec", vec![
                "apt-get".to_string(),
                "install".to_string(),
                "-y".to_string(),
                format!("php{}", version),
                format!("php{}-cli", version),
                format!("php{}-fpm", version),
                format!("php{}-common", version),
                format!("php{}-mysql", version),
                format!("php{}-xml", version),
                format!("php{}-curl", version),
                format!("php{}-mbstring", version),
            ])
        }
        "dnf" => {
            ("pkexec", vec![
                "dnf".to_string(),
                "install".to_string(),
                "-y".to_string(),
                format!("php{}", version.replace(".", "")),
            ])
        }
        "pacman" => {
            ("pkexec", vec![
                "pacman".to_string(),
                "-S".to_string(),
                "--noconfirm".to_string(),
                "php".to_string(),
            ])
        }
        _ => return Err(format!("Unsupported package manager: {}", package_manager)),
    };

    let output = Command::new(cmd)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to install PHP: {}", e))?;

    if output.status.success() {
        Ok(format!("PHP {} installed successfully", version))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
