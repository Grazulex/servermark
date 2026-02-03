use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::{Command, Stdio};

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
#[allow(dead_code)]
struct PackagistResponse {
    packages: Option<PackagistPackages>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct PackagistPackages {
    #[serde(rename = "laravel/framework")]
    laravel_framework: Option<Vec<PackagistVersion>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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
        .args(["-s", "https://repo.packagist.org/p2/laravel/framework.json"])
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
pub fn upgrade_laravel(
    project_path: String,
    target_version: Option<String>,
) -> Result<String, String> {
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
        Ok(format!(
            "Laravel project '{}' created successfully",
            project_name
        ))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// ============================================
// Laravel Scheduler (Cron) Management
// ============================================

/// Generate the cron comment identifier for a site
fn get_cron_identifier(site_path: &str) -> String {
    format!("# ServerMark Scheduler: {}", site_path)
}

/// Generate the cron job line for Laravel scheduler
fn get_cron_job(site_path: &str, php_path: &str) -> String {
    format!(
        "* * * * * cd {} && {} artisan schedule:run >> {}/storage/logs/scheduler.log 2>&1",
        site_path, php_path, site_path
    )
}

/// Check if the Laravel scheduler is enabled for a site
#[tauri::command]
pub fn get_scheduler_status(site_path: String) -> Result<bool, String> {
    let identifier = get_cron_identifier(&site_path);

    // Get current crontab
    let output = Command::new("crontab").arg("-l").output();

    match output {
        Ok(out) => {
            if out.status.success() {
                let crontab = String::from_utf8_lossy(&out.stdout);
                Ok(crontab.contains(&identifier))
            } else {
                // No crontab for user - scheduler not enabled
                Ok(false)
            }
        }
        Err(_) => Ok(false),
    }
}

/// Enable the Laravel scheduler for a site
#[tauri::command]
pub fn enable_scheduler(site_path: String, php_version: String) -> Result<(), String> {
    let path = Path::new(&site_path);

    // Verify it's a Laravel project
    if !path.join("artisan").exists() {
        return Err("Not a Laravel project".to_string());
    }

    // Check if already enabled
    if get_scheduler_status(site_path.clone())? {
        return Ok(()); // Already enabled
    }

    // Determine PHP path
    let php_path = format!("/usr/bin/php{}", php_version);
    if !Path::new(&php_path).exists() {
        return Err(format!("PHP {} not found at {}", php_version, php_path));
    }

    let identifier = get_cron_identifier(&site_path);
    let cron_job = get_cron_job(&site_path, &php_path);

    // Get current crontab
    let current_crontab = Command::new("crontab")
        .arg("-l")
        .output()
        .map(|out| {
            if out.status.success() {
                String::from_utf8_lossy(&out.stdout).to_string()
            } else {
                String::new()
            }
        })
        .unwrap_or_default();

    // Add new cron job
    let new_crontab = if current_crontab.is_empty() {
        format!("{}\n{}\n", identifier, cron_job)
    } else {
        format!(
            "{}\n{}\n{}\n",
            current_crontab.trim_end(),
            identifier,
            cron_job
        )
    };

    // Write new crontab
    let mut child = Command::new("crontab")
        .arg("-")
        .stdin(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to spawn crontab: {}", e))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(new_crontab.as_bytes())
            .map_err(|e| format!("Failed to write crontab: {}", e))?;
    }

    let status = child
        .wait()
        .map_err(|e| format!("Failed to wait for crontab: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err("Failed to update crontab".to_string())
    }
}

/// Disable the Laravel scheduler for a site
#[tauri::command]
pub fn disable_scheduler(site_path: String) -> Result<(), String> {
    let _identifier = get_cron_identifier(&site_path);

    // Get current crontab
    let output = Command::new("crontab")
        .arg("-l")
        .output()
        .map_err(|e| format!("Failed to read crontab: {}", e))?;

    if !output.status.success() {
        return Ok(()); // No crontab, nothing to disable
    }

    let current_crontab = String::from_utf8_lossy(&output.stdout);

    // Filter out the scheduler lines for this site
    let new_lines: Vec<&str> = current_crontab
        .lines()
        .filter(|line| {
            // Remove the identifier line and the cron job line
            !line.contains(&format!("ServerMark Scheduler: {}", site_path))
                && !line.contains(&format!("cd {} &&", site_path))
        })
        .collect();

    let new_crontab = new_lines.join("\n");

    // Write new crontab (or remove if empty)
    if new_crontab.trim().is_empty() {
        // Remove crontab entirely
        Command::new("crontab")
            .arg("-r")
            .output()
            .map_err(|e| format!("Failed to remove crontab: {}", e))?;
    } else {
        let mut child = Command::new("crontab")
            .arg("-")
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn crontab: {}", e))?;

        if let Some(mut stdin) = child.stdin.take() {
            stdin
                .write_all(format!("{}\n", new_crontab).as_bytes())
                .map_err(|e| format!("Failed to write crontab: {}", e))?;
        }

        let status = child
            .wait()
            .map_err(|e| format!("Failed to wait for crontab: {}", e))?;

        if !status.success() {
            return Err("Failed to update crontab".to_string());
        }
    }

    Ok(())
}

// ============================================
// Laravel Queue Worker Management (systemd user services)
// ============================================

/// Generate a slug from site path for service naming
fn get_site_slug(site_path: &str) -> String {
    site_path.trim_start_matches('/').replace(['/', '.'], "-")
}

/// Get the systemd user service name for a site
fn get_queue_service_name(site_path: &str) -> String {
    format!("servermark-queue-{}.service", get_site_slug(site_path))
}

/// Get the systemd user service directory
fn get_systemd_user_dir() -> Result<std::path::PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME not set")?;
    let dir = std::path::PathBuf::from(home).join(".config/systemd/user");
    Ok(dir)
}

/// Generate the systemd service file content
fn generate_queue_service(site_path: &str, site_name: &str, php_path: &str) -> String {
    format!(
        r#"[Unit]
Description=Laravel Queue Worker - {site_name}
After=network.target

[Service]
ExecStart={php_path} {site_path}/artisan queue:work --sleep=3 --tries=3 --max-time=3600
WorkingDirectory={site_path}
Restart=always
RestartSec=5
StandardOutput=null
StandardError=journal

[Install]
WantedBy=default.target
"#,
        site_name = site_name,
        php_path = php_path,
        site_path = site_path
    )
}

/// Check if the Laravel queue worker is running for a site
#[tauri::command]
pub fn get_queue_status(site_path: String) -> Result<bool, String> {
    let service_name = get_queue_service_name(&site_path);

    let output = Command::new("systemctl")
        .args(["--user", "is-active", &service_name])
        .output();

    match output {
        Ok(out) => {
            let status = String::from_utf8_lossy(&out.stdout).trim().to_string();
            Ok(status == "active")
        }
        Err(_) => Ok(false),
    }
}

/// Start the Laravel queue worker for a site
#[tauri::command]
pub fn start_queue_worker(
    site_path: String,
    php_version: String,
    site_name: String,
) -> Result<(), String> {
    let path = Path::new(&site_path);

    // Verify it's a Laravel project
    if !path.join("artisan").exists() {
        return Err("Not a Laravel project".to_string());
    }

    // Check if already running
    if get_queue_status(site_path.clone())? {
        return Ok(()); // Already running
    }

    // Determine PHP path
    let php_path = format!("/usr/bin/php{}", php_version);
    if !Path::new(&php_path).exists() {
        return Err(format!("PHP {} not found at {}", php_version, php_path));
    }

    // Ensure systemd user directory exists
    let systemd_dir = get_systemd_user_dir()?;
    fs::create_dir_all(&systemd_dir)
        .map_err(|e| format!("Failed to create systemd user directory: {}", e))?;

    // Generate and write service file
    let service_name = get_queue_service_name(&site_path);
    let service_path = systemd_dir.join(&service_name);
    let service_content = generate_queue_service(&site_path, &site_name, &php_path);

    fs::write(&service_path, service_content)
        .map_err(|e| format!("Failed to write service file: {}", e))?;

    // Reload systemd user daemon
    Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .output()
        .map_err(|e| format!("Failed to reload systemd: {}", e))?;

    // Enable and start the service
    let output = Command::new("systemctl")
        .args(["--user", "enable", "--now", &service_name])
        .output()
        .map_err(|e| format!("Failed to start service: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(format!(
            "Failed to start queue worker: {}",
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

/// Stop the Laravel queue worker for a site
#[tauri::command]
pub fn stop_queue_worker(site_path: String) -> Result<(), String> {
    let service_name = get_queue_service_name(&site_path);

    // Stop and disable the service
    let _ = Command::new("systemctl")
        .args(["--user", "stop", &service_name])
        .output();

    let _ = Command::new("systemctl")
        .args(["--user", "disable", &service_name])
        .output();

    // Remove the service file
    let systemd_dir = get_systemd_user_dir()?;
    let service_path = systemd_dir.join(&service_name);

    if service_path.exists() {
        fs::remove_file(&service_path)
            .map_err(|e| format!("Failed to remove service file: {}", e))?;
    }

    // Reload systemd
    Command::new("systemctl")
        .args(["--user", "daemon-reload"])
        .output()
        .map_err(|e| format!("Failed to reload systemd: {}", e))?;

    Ok(())
}

// ============================================
// Laravel Logs
// ============================================

/// Get the scheduler logs for a site (from storage/logs/scheduler.log)
#[tauri::command]
pub fn get_scheduler_logs(site_path: String, lines: Option<u32>) -> Result<String, String> {
    let log_path = Path::new(&site_path).join("storage/logs/scheduler.log");

    if !log_path.exists() {
        return Ok(
            "No scheduler logs yet. The scheduler will create logs after its first run."
                .to_string(),
        );
    }

    let lines = lines.unwrap_or(100);

    // Use tail to get the last N lines
    let output = Command::new("tail")
        .args(["-n", &lines.to_string(), log_path.to_str().unwrap()])
        .output()
        .map_err(|e| format!("Failed to read scheduler logs: {}", e))?;

    if output.status.success() {
        let logs = String::from_utf8_lossy(&output.stdout).to_string();
        if logs.trim().is_empty() {
            Ok("Scheduler log file is empty.".to_string())
        } else {
            Ok(logs)
        }
    } else {
        Err("Failed to read scheduler logs".to_string())
    }
}

/// Get the queue worker logs for a site (from journalctl)
#[tauri::command]
pub fn get_queue_logs(site_path: String, lines: Option<u32>) -> Result<String, String> {
    let service_name = get_queue_service_name(&site_path);
    let lines = lines.unwrap_or(100);

    let output = Command::new("journalctl")
        .args([
            "--user",
            "-u",
            &service_name,
            "-n",
            &lines.to_string(),
            "--no-pager",
        ])
        .output()
        .map_err(|e| format!("Failed to read queue logs: {}", e))?;

    if output.status.success() {
        let logs = String::from_utf8_lossy(&output.stdout).to_string();
        if logs.trim().is_empty() || logs.contains("No entries") {
            Ok("No queue worker logs yet. Start the queue worker to see logs.".to_string())
        } else {
            Ok(logs)
        }
    } else {
        // journalctl might fail if service never existed
        Ok("No queue worker logs available.".to_string())
    }
}

/// Clear the scheduler logs for a site
#[tauri::command]
pub fn clear_scheduler_logs(site_path: String) -> Result<(), String> {
    let log_path = Path::new(&site_path).join("storage/logs/scheduler.log");

    if log_path.exists() {
        fs::write(&log_path, "").map_err(|e| format!("Failed to clear scheduler logs: {}", e))?;
    }

    Ok(())
}
