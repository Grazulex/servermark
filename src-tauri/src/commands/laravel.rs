use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaravelInfo {
    pub detected: bool,
    pub version: Option<String>,
    pub constraint: Option<String>,
    pub php_version: Option<String>,
    pub has_update: bool,
    pub latest_version: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ComposerJson {
    require: Option<std::collections::HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
struct ComposerLock {
    packages: Option<Vec<ComposerPackage>>,
}

#[derive(Debug, Deserialize)]
struct ComposerPackage {
    name: String,
    version: String,
}

#[derive(Debug, Deserialize)]
struct PackagistResponse {
    packages: Option<PackagistPackages>,
}

#[derive(Debug, Deserialize)]
struct PackagistPackages {
    #[serde(rename = "laravel/framework")]
    laravel_framework: Option<Vec<PackagistVersion>>,
}

#[derive(Debug, Deserialize)]
struct PackagistVersion {
    version: String,
}

/// Detect Laravel version in a project directory
#[tauri::command]
pub fn detect_laravel_version(project_path: String) -> Result<LaravelInfo, String> {
    let path = Path::new(&project_path);

    // Check if composer.json exists
    let composer_json_path = path.join("composer.json");
    if !composer_json_path.exists() {
        return Ok(LaravelInfo {
            detected: false,
            version: None,
            constraint: None,
            php_version: None,
            has_update: false,
            latest_version: None,
        });
    }

    // Read composer.json
    let composer_json_content = fs::read_to_string(&composer_json_path)
        .map_err(|e| format!("Failed to read composer.json: {}", e))?;

    let composer_json: ComposerJson = serde_json::from_str(&composer_json_content)
        .map_err(|e| format!("Failed to parse composer.json: {}", e))?;

    // Check if laravel/framework is in require
    let require = composer_json.require.unwrap_or_default();
    let laravel_constraint = require.get("laravel/framework").cloned();
    let php_constraint = require.get("php").cloned();

    if laravel_constraint.is_none() {
        return Ok(LaravelInfo {
            detected: false,
            version: None,
            constraint: None,
            php_version: php_constraint,
            has_update: false,
            latest_version: None,
        });
    }

    // Try to get actual installed version from composer.lock
    let composer_lock_path = path.join("composer.lock");
    let installed_version = if composer_lock_path.exists() {
        let lock_content = fs::read_to_string(&composer_lock_path).ok();
        lock_content.and_then(|content| {
            let lock: ComposerLock = serde_json::from_str(&content).ok()?;
            lock.packages?
                .iter()
                .find(|p| p.name == "laravel/framework")
                .map(|p| p.version.trim_start_matches('v').to_string())
        })
    } else {
        None
    };

    Ok(LaravelInfo {
        detected: true,
        version: installed_version,
        constraint: laravel_constraint,
        php_version: php_constraint,
        has_update: false, // Will be checked separately
        latest_version: None,
    })
}

/// Get the latest Laravel version from Packagist
#[tauri::command]
pub fn get_latest_laravel_version() -> Result<String, String> {
    // Use curl to fetch from Packagist API
    let output = Command::new("curl")
        .args([
            "-s",
            "https://repo.packagist.org/p2/laravel/framework.json",
        ])
        .output()
        .map_err(|e| format!("Failed to fetch from Packagist: {}", e))?;

    if !output.status.success() {
        return Err("Failed to fetch from Packagist".to_string());
    }

    let response: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Failed to parse Packagist response: {}", e))?;

    // Find the latest stable version
    if let Some(packages) = response.get("packages") {
        if let Some(laravel) = packages.get("laravel/framework") {
            if let Some(versions) = laravel.as_array() {
                for version_info in versions {
                    if let Some(version) = version_info.get("version").and_then(|v| v.as_str()) {
                        // Skip dev/alpha/beta versions
                        if !version.contains("dev")
                            && !version.contains("alpha")
                            && !version.contains("beta")
                            && !version.contains("RC")
                        {
                            return Ok(version.trim_start_matches('v').to_string());
                        }
                    }
                }
            }
        }
    }

    Err("Could not determine latest Laravel version".to_string())
}

/// Upgrade Laravel in a project
#[tauri::command]
pub fn upgrade_laravel(project_path: String, target_version: Option<String>) -> Result<String, String> {
    let path = Path::new(&project_path);

    // Build the composer command
    let version_constraint = target_version
        .map(|v| format!("laravel/framework:^{}", v))
        .unwrap_or_else(|| "laravel/framework".to_string());

    let output = Command::new("composer")
        .current_dir(path)
        .args(["require", &version_constraint, "--update-with-dependencies"])
        .output()
        .map_err(|e| format!("Failed to run composer: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Create a new Laravel project
#[tauri::command]
pub fn create_laravel_project(
    project_path: String,
    project_name: String,
    version: Option<String>,
) -> Result<String, String> {
    let path = Path::new(&project_path);

    let mut args = vec!["create-project", "laravel/laravel", &project_name];

    let version_arg;
    if let Some(v) = &version {
        version_arg = format!("^{}", v);
        args.push(&version_arg);
    }

    args.push("--prefer-dist");

    let output = Command::new("composer")
        .current_dir(path)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to create Laravel project: {}", e))?;

    if output.status.success() {
        Ok(format!("Laravel project '{}' created successfully", project_name))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
