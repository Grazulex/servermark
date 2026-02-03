use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhpVersion {
    pub version: String,
    pub full_version: String,
    pub installed: bool,
    pub active: bool,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhpExtension {
    pub name: String,
    pub display_name: String,
    pub description: String,
    pub category: String, // "required", "recommended", "optional"
    pub default_selected: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PpaStatus {
    pub installed: bool,
    pub name: String,
    pub add_command: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstallProgress {
    pub step: String,
    pub current_step: u8,
    pub total_steps: u8,
    pub status: String, // "running", "complete", "error"
}

/// Get all PHP versions installed on the system
#[tauri::command]
pub fn get_php_versions() -> Result<Vec<PhpVersion>, String> {
    let known_versions = vec!["8.4", "8.3", "8.2", "8.1", "8.0", "7.4", "7.3", "7.2"];
    let search_paths = vec!["/usr/bin", "/usr/local/bin", "/opt/php"];

    let mut result = Vec::new();
    let mut found_versions: Vec<String> = Vec::new();

    // Get current active PHP version
    let (active_version, _active_full) = get_active_php_version();

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
                if version_part
                    .chars()
                    .next()
                    .map(|c| c.is_ascii_digit())
                    .unwrap_or(false)
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
        let a_parts: Vec<u32> = a
            .version
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
        let b_parts: Vec<u32> = b
            .version
            .split('.')
            .filter_map(|s| s.parse().ok())
            .collect();
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
            output
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1).map(|v| v.to_string()))
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
            (
                "pkexec",
                vec![
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
                ],
            )
        }
        "dnf" => (
            "pkexec",
            vec![
                "dnf".to_string(),
                "install".to_string(),
                "-y".to_string(),
                format!("php{}", version.replace(".", "")),
            ],
        ),
        "pacman" => (
            "pkexec",
            vec![
                "pacman".to_string(),
                "-S".to_string(),
                "--noconfirm".to_string(),
                "php".to_string(),
            ],
        ),
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

/// Check if ondrej/php PPA is installed (Ubuntu/Debian)
#[tauri::command]
pub fn check_php_ppa() -> PpaStatus {
    // Check for ondrej/php PPA in apt sources
    let ppa_patterns = [
        "/etc/apt/sources.list.d/ondrej-ubuntu-php",
        "/etc/apt/sources.list.d/ondrej-php",
        "/etc/apt/sources.list.d/ppa_ondrej_php",
    ];

    // Also check by grepping sources
    let grep_check = Command::new("grep")
        .args(["-r", "ondrej/php", "/etc/apt/sources.list.d/"])
        .output()
        .ok()
        .map(|o| o.status.success())
        .unwrap_or(false);

    let file_check = ppa_patterns.iter().any(|_pattern| {
        fs::read_dir("/etc/apt/sources.list.d/")
            .ok()
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .any(|e| e.file_name().to_string_lossy().contains("ondrej"))
            })
            .unwrap_or(false)
    });

    let installed = grep_check || file_check;

    PpaStatus {
        installed,
        name: "ondrej/php".to_string(),
        add_command: "sudo add-apt-repository ppa:ondrej/php -y && sudo apt update".to_string(),
    }
}

/// Add ondrej/php PPA (Ubuntu/Debian)
#[tauri::command]
pub fn add_php_ppa(app: AppHandle) -> Result<String, String> {
    let emit_progress = |step: &str, current: u8, total: u8, status: &str| {
        let _ = app.emit(
            "ppa-progress",
            InstallProgress {
                step: step.to_string(),
                current_step: current,
                total_steps: total,
                status: status.to_string(),
            },
        );
    };

    // Step 1: Install prerequisites
    emit_progress("Installing software-properties-common...", 1, 3, "running");

    let prereq = Command::new("pkexec")
        .args(["apt-get", "install", "-y", "software-properties-common"])
        .output()
        .map_err(|e| {
            emit_progress("Failed to install prerequisites", 1, 3, "error");
            format!("Failed to install prerequisites: {}", e)
        })?;

    if !prereq.status.success() {
        emit_progress("Failed to install prerequisites", 1, 3, "error");
        return Err(format!(
            "Failed to install software-properties-common: {}",
            String::from_utf8_lossy(&prereq.stderr)
        ));
    }

    // Step 2: Add the PPA
    emit_progress("Adding PPA ondrej/php...", 2, 3, "running");

    let output = Command::new("pkexec")
        .args(["add-apt-repository", "ppa:ondrej/php", "-y"])
        .output()
        .map_err(|e| {
            emit_progress("Failed to add PPA", 2, 3, "error");
            format!("Failed to add PPA: {}", e)
        })?;

    if !output.status.success() {
        emit_progress("Failed to add PPA", 2, 3, "error");
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    // Step 3: Update apt cache
    emit_progress("Updating package lists...", 3, 3, "running");

    let update = Command::new("pkexec")
        .args(["apt-get", "update"])
        .output()
        .map_err(|e| {
            emit_progress("Failed to update apt", 3, 3, "error");
            format!("Failed to update apt: {}", e)
        })?;

    if update.status.success() {
        emit_progress("PPA added successfully", 3, 3, "complete");
        Ok("PPA ondrej/php added successfully".to_string())
    } else {
        emit_progress("Failed to update apt", 3, 3, "error");
        Err(String::from_utf8_lossy(&update.stderr).to_string())
    }
}

/// Get available PHP extensions for installation
#[tauri::command]
pub fn get_php_extensions() -> Vec<PhpExtension> {
    vec![
        // Required extensions
        PhpExtension {
            name: "cli".to_string(),
            display_name: "CLI".to_string(),
            description: "Command-line interface".to_string(),
            category: "required".to_string(),
            default_selected: true,
        },
        PhpExtension {
            name: "fpm".to_string(),
            display_name: "FPM".to_string(),
            description: "FastCGI Process Manager for web servers".to_string(),
            category: "required".to_string(),
            default_selected: true,
        },
        PhpExtension {
            name: "common".to_string(),
            display_name: "Common".to_string(),
            description: "Common files and documentation".to_string(),
            category: "required".to_string(),
            default_selected: true,
        },
        // Recommended extensions
        PhpExtension {
            name: "mysql".to_string(),
            display_name: "MySQL".to_string(),
            description: "MySQL/MariaDB database support".to_string(),
            category: "recommended".to_string(),
            default_selected: true,
        },
        PhpExtension {
            name: "curl".to_string(),
            display_name: "cURL".to_string(),
            description: "HTTP client library".to_string(),
            category: "recommended".to_string(),
            default_selected: true,
        },
        PhpExtension {
            name: "mbstring".to_string(),
            display_name: "Mbstring".to_string(),
            description: "Multibyte string handling".to_string(),
            category: "recommended".to_string(),
            default_selected: true,
        },
        PhpExtension {
            name: "xml".to_string(),
            display_name: "XML".to_string(),
            description: "XML parsing and manipulation".to_string(),
            category: "recommended".to_string(),
            default_selected: true,
        },
        PhpExtension {
            name: "zip".to_string(),
            display_name: "Zip".to_string(),
            description: "Zip archive support".to_string(),
            category: "recommended".to_string(),
            default_selected: true,
        },
        PhpExtension {
            name: "bcmath".to_string(),
            display_name: "BCMath".to_string(),
            description: "Arbitrary precision mathematics".to_string(),
            category: "recommended".to_string(),
            default_selected: true,
        },
        // Optional extensions
        PhpExtension {
            name: "gd".to_string(),
            display_name: "GD".to_string(),
            description: "Image processing library".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
        PhpExtension {
            name: "intl".to_string(),
            display_name: "Intl".to_string(),
            description: "Internationalization support".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
        PhpExtension {
            name: "redis".to_string(),
            display_name: "Redis".to_string(),
            description: "Redis cache/session driver".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
        PhpExtension {
            name: "pgsql".to_string(),
            display_name: "PostgreSQL".to_string(),
            description: "PostgreSQL database support".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
        PhpExtension {
            name: "sqlite3".to_string(),
            display_name: "SQLite3".to_string(),
            description: "SQLite database support".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
        PhpExtension {
            name: "imagick".to_string(),
            display_name: "Imagick".to_string(),
            description: "ImageMagick image processing".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
        PhpExtension {
            name: "soap".to_string(),
            display_name: "SOAP".to_string(),
            description: "SOAP web services support".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
        PhpExtension {
            name: "ldap".to_string(),
            display_name: "LDAP".to_string(),
            description: "LDAP directory support".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
        PhpExtension {
            name: "xdebug".to_string(),
            display_name: "Xdebug".to_string(),
            description: "Debugging and profiling".to_string(),
            category: "optional".to_string(),
            default_selected: false,
        },
    ]
}

/// Install a PHP version with custom extensions
#[tauri::command]
pub fn install_php_with_extensions(
    app: AppHandle,
    version: String,
    extensions: Vec<String>,
    package_manager: String,
) -> Result<String, String> {
    log::info!("Starting PHP {} installation with extensions", version);

    let emit_progress = |step: &str, current: u8, total: u8, status: &str| {
        log::info!("Emitting progress: {} ({}/{})", step, current, total);
        let result = app.emit(
            "php-install-progress",
            InstallProgress {
                step: step.to_string(),
                current_step: current,
                total_steps: total,
                status: status.to_string(),
            },
        );
        if let Err(e) = result {
            log::error!("Failed to emit progress event: {}", e);
        }
    };

    emit_progress(
        &format!("Preparing PHP {} installation...", version),
        1,
        3,
        "running",
    );

    let packages: Vec<String> = match package_manager.as_str() {
        "apt" => {
            let mut pkgs = vec![format!("php{}", version)];
            for ext in &extensions {
                pkgs.push(format!("php{}-{}", version, ext));
            }
            pkgs
        }
        "dnf" => {
            let ver = version.replace(".", "");
            let mut pkgs = vec![format!("php{}", ver)];
            for ext in &extensions {
                pkgs.push(format!("php{}-php-{}", ver, ext));
            }
            pkgs
        }
        "pacman" => {
            // Arch uses different naming
            let mut pkgs = vec!["php".to_string()];
            for ext in &extensions {
                pkgs.push(format!("php-{}", ext));
            }
            pkgs
        }
        _ => {
            emit_progress("Unsupported package manager", 1, 3, "error");
            return Err(format!("Unsupported package manager: {}", package_manager));
        }
    };

    let package_count = packages.len();
    emit_progress(
        &format!("Installing {} packages...", package_count),
        2,
        3,
        "running",
    );

    let (cmd, args) = match package_manager.as_str() {
        "apt" => {
            let mut a = vec![
                "apt-get".to_string(),
                "install".to_string(),
                "-y".to_string(),
            ];
            a.extend(packages);
            ("pkexec", a)
        }
        "dnf" => {
            let mut a = vec!["dnf".to_string(), "install".to_string(), "-y".to_string()];
            a.extend(packages);
            ("pkexec", a)
        }
        "pacman" => {
            let mut a = vec![
                "pacman".to_string(),
                "-S".to_string(),
                "--noconfirm".to_string(),
            ];
            a.extend(packages);
            ("pkexec", a)
        }
        _ => {
            emit_progress("Unsupported package manager", 2, 3, "error");
            return Err(format!("Unsupported package manager: {}", package_manager));
        }
    };

    let output = Command::new(cmd).args(&args).output().map_err(|e| {
        emit_progress("Installation failed", 2, 3, "error");
        format!("Failed to install PHP: {}", e)
    })?;

    if output.status.success() {
        emit_progress(
            &format!("PHP {} installed successfully!", version),
            3,
            3,
            "complete",
        );
        Ok(format!(
            "PHP {} installed successfully with extensions: {}",
            version,
            extensions.join(", ")
        ))
    } else {
        emit_progress("Installation failed", 3, 3, "error");
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Uninstall a PHP version
#[tauri::command]
pub fn uninstall_php_version(
    app: AppHandle,
    version: String,
    package_manager: String,
) -> Result<String, String> {
    let emit_progress = |step: &str, current: u8, total: u8, status: &str| {
        let _ = app.emit(
            "php-uninstall-progress",
            InstallProgress {
                step: step.to_string(),
                current_step: current,
                total_steps: total,
                status: status.to_string(),
            },
        );
    };

    emit_progress(&format!("Removing PHP {}...", version), 1, 2, "running");

    let (cmd, args) = match package_manager.as_str() {
        "apt" => (
            "pkexec",
            vec![
                "apt-get".to_string(),
                "remove".to_string(),
                "--purge".to_string(),
                "-y".to_string(),
                format!("php{}*", version),
            ],
        ),
        "dnf" => {
            let ver = version.replace(".", "");
            (
                "pkexec",
                vec![
                    "dnf".to_string(),
                    "remove".to_string(),
                    "-y".to_string(),
                    format!("php{}*", ver),
                ],
            )
        }
        "pacman" => (
            "pkexec",
            vec![
                "pacman".to_string(),
                "-Rns".to_string(),
                "--noconfirm".to_string(),
                "php".to_string(),
            ],
        ),
        _ => {
            emit_progress("Unsupported package manager", 1, 2, "error");
            return Err(format!("Unsupported package manager: {}", package_manager));
        }
    };

    let output = Command::new(cmd).args(&args).output().map_err(|e| {
        emit_progress("Uninstall failed", 1, 2, "error");
        format!("Failed to uninstall PHP: {}", e)
    })?;

    if output.status.success() {
        emit_progress(
            &format!("PHP {} removed successfully!", version),
            2,
            2,
            "complete",
        );
        Ok(format!("PHP {} uninstalled successfully", version))
    } else {
        emit_progress("Uninstall failed", 2, 2, "error");
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
