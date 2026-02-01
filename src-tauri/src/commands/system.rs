use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemInfo {
    pub distro: String,
    pub distro_version: String,
    pub package_manager: String,
    pub kernel: String,
    pub hostname: String,
}

#[tauri::command]
pub fn detect_system() -> Result<SystemInfo, String> {
    // Read /etc/os-release to detect distribution
    let os_release = fs::read_to_string("/etc/os-release")
        .map_err(|e| format!("Failed to read /etc/os-release: {}", e))?;

    let mut distro = String::from("unknown");
    let mut distro_version = String::new();

    for line in os_release.lines() {
        if line.starts_with("ID=") {
            distro = line.trim_start_matches("ID=").trim_matches('"').to_string();
        }
        if line.starts_with("VERSION_ID=") {
            distro_version = line.trim_start_matches("VERSION_ID=").trim_matches('"').to_string();
        }
    }

    // Determine package manager based on distro
    let package_manager = match distro.as_str() {
        "ubuntu" | "debian" | "linuxmint" | "pop" | "elementary" | "zorin" => "apt",
        "fedora" | "rhel" | "centos" | "rocky" | "almalinux" => "dnf",
        "arch" | "manjaro" | "endeavouros" => "pacman",
        "opensuse" | "opensuse-leap" | "opensuse-tumbleweed" => "zypper",
        _ => "unknown",
    }.to_string();

    // Get kernel version
    let kernel = Command::new("uname")
        .arg("-r")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    // Get hostname
    let hostname = Command::new("hostname")
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default();

    Ok(SystemInfo {
        distro,
        distro_version,
        package_manager,
        kernel,
        hostname,
    })
}
