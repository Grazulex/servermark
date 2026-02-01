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

// ============================================================================
// Web Server Detection
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebServerStatus {
    pub caddy_installed: bool,
    pub caddy_running: bool,
    pub caddy_version: Option<String>,
    pub nginx_installed: bool,
    pub nginx_running: bool,
    pub nginx_version: Option<String>,
    pub active: Option<String>, // "caddy" | "nginx" | null
}

#[tauri::command]
pub fn detect_web_server() -> WebServerStatus {
    // Check Caddy
    let caddy_version = Command::new("caddy")
        .arg("version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .next()
                .unwrap_or("")
                .trim()
                .to_string()
        });

    let caddy_installed = caddy_version.is_some();

    let caddy_running = Command::new("systemctl")
        .args(["is-active", "caddy"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    // Check Nginx
    let nginx_version = Command::new("nginx")
        .arg("-v")
        .output()
        .ok()
        .filter(|o| o.status.success() || !o.stderr.is_empty())
        .map(|o| {
            // nginx -v outputs to stderr
            String::from_utf8_lossy(&o.stderr)
                .trim()
                .replace("nginx version: ", "")
                .to_string()
        });

    let nginx_installed = nginx_version.is_some();

    let nginx_running = Command::new("systemctl")
        .args(["is-active", "nginx"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    // Determine active server
    let active = if caddy_running {
        Some("caddy".to_string())
    } else if nginx_running {
        Some("nginx".to_string())
    } else {
        None
    };

    WebServerStatus {
        caddy_installed,
        caddy_running,
        caddy_version,
        nginx_installed,
        nginx_running,
        nginx_version,
        active,
    }
}

#[tauri::command]
pub fn install_web_server(server: String, package_manager: String) -> Result<(), String> {
    match server.as_str() {
        "caddy" => install_caddy(&package_manager),
        "nginx" => install_nginx(&package_manager),
        _ => Err("Unknown web server".to_string()),
    }
}

fn install_caddy(package_manager: &str) -> Result<(), String> {
    // Build the install command based on package manager
    let install_cmd = match package_manager {
        "apt" => "apt install -y caddy",
        "dnf" => "dnf install -y caddy",
        "pacman" => "pacman -S --noconfirm caddy",
        "zypper" => "zypper install -y caddy",
        _ => return Err("Unsupported package manager".to_string()),
    };

    // Complete setup script - runs with single pkexec prompt
    // Uses /etc/caddy/sites.d/ which Caddy can access (unlike user's home)
    let setup_script = format!(r##"
set -e

# 1. Install Caddy
{install_cmd}

# 2. Add caddy user to www-data group (for PHP-FPM socket access)
usermod -aG www-data caddy || true

# 3. Create ServerMark sites directory
mkdir -p /etc/caddy/sites.d
chown caddy:caddy /etc/caddy/sites.d
chmod 755 /etc/caddy/sites.d

# 4. Create empty Caddyfile for ServerMark sites
echo "# ServerMark sites" > /etc/caddy/sites.d/servermark.conf
chown caddy:caddy /etc/caddy/sites.d/servermark.conf

# 5. Add import to system Caddyfile if not present
IMPORT_LINE="import /etc/caddy/sites.d/*.conf"
if ! grep -q "sites.d" /etc/caddy/Caddyfile 2>/dev/null; then
    # Prepend import line to existing Caddyfile
    if [ -f /etc/caddy/Caddyfile ]; then
        TEMP_FILE=$(mktemp)
        echo "$IMPORT_LINE" > "$TEMP_FILE"
        echo "" >> "$TEMP_FILE"
        cat /etc/caddy/Caddyfile >> "$TEMP_FILE"
        mv "$TEMP_FILE" /etc/caddy/Caddyfile
    else
        echo "$IMPORT_LINE" > /etc/caddy/Caddyfile
    fi
fi

# 6. Enable and start Caddy
systemctl enable caddy
systemctl restart caddy
"##, install_cmd = install_cmd);

    let output = Command::new("pkexec")
        .args(["bash", "-c", &setup_script])
        .output()
        .map_err(|e| format!("Failed to install Caddy: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to install Caddy: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

fn install_nginx(package_manager: &str) -> Result<(), String> {
    let install_cmd = match package_manager {
        "apt" => "apt install -y nginx",
        "dnf" => "dnf install -y nginx",
        "pacman" => "pacman -S --noconfirm nginx",
        "zypper" => "zypper install -y nginx",
        _ => return Err("Unsupported package manager".to_string()),
    };

    let setup_script = format!(r#"
set -e
{install_cmd}
systemctl enable nginx
systemctl start nginx
"#, install_cmd = install_cmd);

    let output = Command::new("pkexec")
        .args(["bash", "-c", &setup_script])
        .output()
        .map_err(|e| format!("Failed to install Nginx: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to install Nginx: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

#[tauri::command]
pub fn switch_web_server(server: String) -> Result<(), String> {
    match server.as_str() {
        "caddy" => {
            // Stop nginx, start caddy
            let _ = Command::new("pkexec")
                .args(["systemctl", "stop", "nginx"])
                .output();
            Command::new("pkexec")
                .args(["systemctl", "start", "caddy"])
                .output()
                .map_err(|e| format!("Failed to start Caddy: {}", e))?;
        }
        "nginx" => {
            // Stop caddy, start nginx
            let _ = Command::new("pkexec")
                .args(["systemctl", "stop", "caddy"])
                .output();
            Command::new("pkexec")
                .args(["systemctl", "start", "nginx"])
                .output()
                .map_err(|e| format!("Failed to start Nginx: {}", e))?;
        }
        _ => return Err("Unknown web server".to_string()),
    }

    Ok(())
}
