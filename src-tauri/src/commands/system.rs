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

// ============================================================================
// DNS (dnsmasq) Detection and Installation
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsStatus {
    pub dnsmasq_installed: bool,
    pub dnsmasq_running: bool,
    pub dnsmasq_configured: bool, // Has .test domain config
    pub resolver_configured: bool, // systemd-resolved configured
}

#[tauri::command]
pub fn detect_dns() -> DnsStatus {
    // Check if dnsmasq is installed
    let dnsmasq_installed = Command::new("which")
        .arg("dnsmasq")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    // Check if dnsmasq is running (via systemctl or by checking if port 5353 is in use)
    let dnsmasq_running = Command::new("systemctl")
        .args(["is-active", "dnsmasq"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
        || Command::new("ss")
            .args(["-tlnp"])
            .output()
            .map(|o| String::from_utf8_lossy(&o.stdout).contains(":5353")
                  && String::from_utf8_lossy(&o.stdout).contains("dnsmasq"))
            .unwrap_or(false);

    // Check if .test domain is configured in dnsmasq (ServerMark config or any other)
    let dnsmasq_configured = fs::read_to_string("/etc/dnsmasq.d/servermark.conf")
        .map(|content| content.contains("address=/.test/"))
        .unwrap_or(false)
        || fs::read_dir("/etc/dnsmasq.d")
            .map(|entries| {
                entries.filter_map(|e| e.ok())
                    .any(|entry| {
                        fs::read_to_string(entry.path())
                            .map(|content| content.contains("address=/.test/") || content.contains("address=/test/"))
                            .unwrap_or(false)
                    })
            })
            .unwrap_or(false);

    // Check if systemd-resolved is configured to forward .test queries
    let resolver_configured = fs::read_to_string("/etc/systemd/resolved.conf.d/servermark.conf")
        .map(|content| content.contains("Domains=~test") || content.contains("DNS=127.0.0.1"))
        .unwrap_or(false);

    DnsStatus {
        dnsmasq_installed,
        dnsmasq_running,
        dnsmasq_configured,
        resolver_configured,
    }
}

#[tauri::command]
pub fn install_dns(package_manager: String) -> Result<(), String> {
    let install_cmd = match package_manager.as_str() {
        "apt" => "apt install -y dnsmasq",
        "dnf" => "dnf install -y dnsmasq",
        "pacman" => "pacman -S --noconfirm dnsmasq",
        "zypper" => "zypper install -y dnsmasq",
        _ => return Err("Unsupported package manager".to_string()),
    };

    // Robust DNS setup script that works with systemd-resolved
    // Strategy: Use dnsmasq on port 5353, forward via systemd-resolved
    let setup_script = format!(r##"
set -e

echo "=== ServerMark DNS Setup ==="

# 1. Install dnsmasq if not present
if ! command -v dnsmasq &> /dev/null; then
    echo "Installing dnsmasq..."
    {install_cmd}
fi

# 2. Stop dnsmasq service to reconfigure
echo "Stopping dnsmasq service..."
systemctl stop dnsmasq 2>/dev/null || true
systemctl disable dnsmasq 2>/dev/null || true

# 3. Create ServerMark dnsmasq config (minimal to avoid conflicts with existing configs)
echo "Configuring dnsmasq..."
mkdir -p /etc/dnsmasq.d

cat > /etc/dnsmasq.d/servermark.conf << 'DNSCONF'
# ServerMark DNS configuration
# Resolve *.test domains to localhost
address=/.test/127.0.0.1

# Use port 5353 to coexist with systemd-resolved
port=5353
DNSCONF

# 3b. Create base config only if no other config exists
if [ ! -f /etc/dnsmasq.d/local-dev.conf ] && [ ! -f /etc/dnsmasq.d/base.conf ]; then
    cat > /etc/dnsmasq.d/servermark-base.conf << 'BASECONF'
# ServerMark base DNS configuration
listen-address=127.0.0.1
bind-interfaces
no-resolv
server=8.8.8.8
server=1.1.1.1
cache-size=1000
BASECONF
fi

# 4. Ensure main dnsmasq.conf doesn't conflict
if [ -f /etc/dnsmasq.conf ]; then
    # Comment out any port= line in main config
    sed -i 's/^port=/#port=/' /etc/dnsmasq.conf 2>/dev/null || true
fi

# 5. Configure systemd-resolved to forward .test to dnsmasq
echo "Configuring systemd-resolved integration..."
mkdir -p /etc/systemd/resolved.conf.d

cat > /etc/systemd/resolved.conf.d/servermark.conf << 'RESOLVEDCONF'
[Resolve]
# Forward .test domain queries to local dnsmasq
DNS=127.0.0.1#5353
Domains=~test
RESOLVEDCONF

# 6. Restart systemd-resolved to pick up new config
echo "Restarting systemd-resolved..."
systemctl restart systemd-resolved

# 7. Enable and start dnsmasq
echo "Starting dnsmasq..."
systemctl enable dnsmasq
systemctl start dnsmasq

# 8. Verify setup
sleep 1
echo ""
echo "=== Verification ==="

if systemctl is-active --quiet dnsmasq; then
    echo "✓ dnsmasq is running on port 5353"
else
    echo "✗ dnsmasq failed to start"
    journalctl -u dnsmasq -n 5 --no-pager
    exit 1
fi

# Test DNS resolution
echo "Testing DNS resolution..."
if command -v resolvectl &> /dev/null; then
    resolvectl query test.test 2>/dev/null && echo "✓ DNS resolution working" || echo "Note: DNS test query (this is normal if no .test site exists yet)"
fi

echo ""
echo "=== Setup Complete ==="
echo "All *.test domains will now resolve to 127.0.0.1"
"##, install_cmd = install_cmd);

    let output = Command::new("pkexec")
        .args(["bash", "-c", &setup_script])
        .output()
        .map_err(|e| format!("Failed to install DNS: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Err(format!(
            "Failed to install DNS:\n{}\n{}",
            stdout, stderr
        ));
    }

    Ok(())
}

// ============================================================================
// Native Services Detection
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeService {
    pub name: String,
    pub display_name: String,
    pub installed: bool,
    pub running: bool,
    pub version: Option<String>,
    pub port: Option<u16>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeServicesStatus {
    pub services: Vec<NativeService>,
}

#[tauri::command]
pub fn detect_native_services() -> NativeServicesStatus {
    let mut services = Vec::new();

    // MySQL/MariaDB
    let mysql_version = Command::new("mysql")
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .split_whitespace()
                .nth(2)
                .unwrap_or("")
                .trim_end_matches(',')
                .to_string()
        });

    let mysql_running = Command::new("systemctl")
        .args(["is-active", "mysql"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
        || Command::new("systemctl")
            .args(["is-active", "mariadb"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

    services.push(NativeService {
        name: "mysql".to_string(),
        display_name: "MySQL/MariaDB".to_string(),
        installed: mysql_version.is_some(),
        running: mysql_running,
        version: mysql_version,
        port: Some(3306),
    });

    // PostgreSQL
    let pg_version = Command::new("psql")
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .split_whitespace()
                .last()
                .unwrap_or("")
                .to_string()
        });

    let pg_running = Command::new("systemctl")
        .args(["is-active", "postgresql"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    services.push(NativeService {
        name: "postgresql".to_string(),
        display_name: "PostgreSQL".to_string(),
        installed: pg_version.is_some(),
        running: pg_running,
        version: pg_version,
        port: Some(5432),
    });

    // Redis
    let redis_version = Command::new("redis-server")
        .arg("--version")
        .output()
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .split_whitespace()
                .find(|s| s.starts_with("v="))
                .map(|s| s.trim_start_matches("v=").to_string())
                .unwrap_or_default()
        });

    let redis_running = Command::new("systemctl")
        .args(["is-active", "redis"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
        || Command::new("systemctl")
            .args(["is-active", "redis-server"])
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);

    services.push(NativeService {
        name: "redis".to_string(),
        display_name: "Redis".to_string(),
        installed: redis_version.is_some(),
        running: redis_running,
        version: redis_version,
        port: Some(6379),
    });

    // Memcached
    let memcached_version = Command::new("memcached")
        .arg("-h")
        .output()
        .ok()
        .filter(|o| !o.stdout.is_empty() || !o.stderr.is_empty())
        .map(|o| {
            let output = String::from_utf8_lossy(&o.stdout);
            output
                .lines()
                .next()
                .and_then(|l| l.split_whitespace().nth(1))
                .unwrap_or("")
                .to_string()
        });

    let memcached_running = Command::new("systemctl")
        .args(["is-active", "memcached"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false);

    services.push(NativeService {
        name: "memcached".to_string(),
        display_name: "Memcached".to_string(),
        installed: memcached_version.is_some(),
        running: memcached_running,
        version: memcached_version,
        port: Some(11211),
    });

    NativeServicesStatus { services }
}

#[tauri::command]
pub fn control_native_service(service: String, action: String) -> Result<(), String> {
    let service_name = match service.as_str() {
        "mysql" => {
            // Check if it's mysql or mariadb
            if Command::new("systemctl")
                .args(["status", "mariadb"])
                .output()
                .map(|o| o.status.success() || String::from_utf8_lossy(&o.stdout).contains("mariadb"))
                .unwrap_or(false)
            {
                "mariadb"
            } else {
                "mysql"
            }
        }
        "postgresql" => "postgresql",
        "redis" => {
            if Command::new("systemctl")
                .args(["status", "redis-server"])
                .output()
                .map(|o| o.status.success() || String::from_utf8_lossy(&o.stdout).contains("redis-server"))
                .unwrap_or(false)
            {
                "redis-server"
            } else {
                "redis"
            }
        }
        "memcached" => "memcached",
        "dnsmasq" => "dnsmasq",
        "caddy" => "caddy",
        "nginx" => "nginx",
        _ => return Err(format!("Unknown service: {}", service)),
    };

    let action_arg = match action.as_str() {
        "start" | "stop" | "restart" => action.as_str(),
        _ => return Err(format!("Unknown action: {}", action)),
    };

    let output = Command::new("pkexec")
        .args(["systemctl", action_arg, service_name])
        .output()
        .map_err(|e| format!("Failed to {} {}: {}", action, service, e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to {} {}: {}",
            action,
            service,
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

// ============================================================================
// Terminal & Logs
// ============================================================================

#[tauri::command]
pub fn open_terminal(path: String) -> Result<(), String> {
    // Try common terminal emulators in order of preference
    let terminals: &[(&str, &[&str])] = &[
        ("gnome-terminal", &["--working-directory"]),
        ("konsole", &["--workdir"]),
        ("xfce4-terminal", &["--working-directory"]),
        ("terminator", &["--working-directory"]),
        ("tilix", &["--working-directory"]),
        ("alacritty", &["--working-directory"]),
        ("kitty", &["--directory"]),
    ];

    for (terminal, args) in terminals {
        if Command::new("which")
            .arg(terminal)
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
        {
            let mut cmd = Command::new(terminal);
            for arg in *args {
                cmd.arg(arg);
            }
            cmd.arg(&path);
            cmd.spawn()
                .map_err(|e| format!("Failed to open terminal: {}", e))?;
            return Ok(());
        }
    }

    // Fallback to xterm
    if Command::new("which")
        .arg("xterm")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        Command::new("xterm")
            .args(["-e", &format!("cd '{}' && $SHELL", path)])
            .spawn()
            .map_err(|e| format!("Failed to open terminal: {}", e))?;
        return Ok(());
    }

    Err("No supported terminal emulator found".to_string())
}

#[tauri::command]
pub fn get_service_logs(service: String, lines: u32) -> Result<String, String> {
    let service_name = match service.as_str() {
        "caddy" => "caddy",
        "nginx" => "nginx",
        "php-fpm" => "php*-fpm", // Wildcard for any PHP version
        "mysql" => "mysql",
        "mariadb" => "mariadb",
        "postgresql" => "postgresql",
        "redis" => "redis",
        "dnsmasq" => "dnsmasq",
        _ => return Err(format!("Unknown service: {}", service)),
    };

    // Use journalctl to get logs
    let output = Command::new("journalctl")
        .args(["-u", service_name, "-n", &lines.to_string(), "--no-pager"])
        .output()
        .map_err(|e| format!("Failed to get logs: {}", e))?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        // Try reading log files directly for some services
        let log_paths = match service.as_str() {
            "caddy" => vec!["/var/log/caddy/access.log", "/var/log/caddy/error.log"],
            "nginx" => vec!["/var/log/nginx/access.log", "/var/log/nginx/error.log"],
            _ => vec![],
        };

        for log_path in log_paths {
            if let Ok(content) = fs::read_to_string(log_path) {
                let log_lines: Vec<&str> = content.lines().rev().take(lines as usize).collect();
                return Ok(log_lines.into_iter().rev().collect::<Vec<_>>().join("\n"));
            }
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

// ============================================================================
// Database Management
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    pub name: String,
    pub size: Option<String>,
}

#[tauri::command]
pub fn list_databases(db_type: String, container_id: Option<String>) -> Result<Vec<Database>, String> {
    match db_type.as_str() {
        "mysql" => {
            let cmd = if let Some(id) = container_id {
                format!("docker exec {} mysql -uroot -psecret -e 'SHOW DATABASES;' 2>/dev/null", id)
            } else {
                "mysql -uroot -e 'SHOW DATABASES;' 2>/dev/null".to_string()
            };

            let output = Command::new("sh")
                .args(["-c", &cmd])
                .output()
                .map_err(|e| format!("Failed to list databases: {}", e))?;

            let databases: Vec<Database> = String::from_utf8_lossy(&output.stdout)
                .lines()
                .skip(1) // Skip header
                .filter(|name| !["information_schema", "mysql", "performance_schema", "sys"].contains(name))
                .map(|name| Database {
                    name: name.to_string(),
                    size: None,
                })
                .collect();

            Ok(databases)
        }
        "postgresql" => {
            let cmd = if let Some(id) = container_id {
                format!("docker exec {} psql -U postgres -c '\\l' -t 2>/dev/null", id)
            } else {
                "psql -U postgres -c '\\l' -t 2>/dev/null".to_string()
            };

            let output = Command::new("sh")
                .args(["-c", &cmd])
                .output()
                .map_err(|e| format!("Failed to list databases: {}", e))?;

            let databases: Vec<Database> = String::from_utf8_lossy(&output.stdout)
                .lines()
                .filter_map(|line| {
                    let name = line.split('|').next()?.trim();
                    if !name.is_empty() && !["postgres", "template0", "template1"].contains(&name) {
                        Some(Database {
                            name: name.to_string(),
                            size: None,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            Ok(databases)
        }
        _ => Err(format!("Unsupported database type: {}", db_type)),
    }
}

#[tauri::command]
pub fn create_database(db_type: String, name: String, container_id: Option<String>) -> Result<(), String> {
    // Validate database name (alphanumeric and underscore only)
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Database name can only contain letters, numbers, and underscores".to_string());
    }

    match db_type.as_str() {
        "mysql" => {
            let cmd = if let Some(id) = container_id {
                format!("docker exec {} mysql -uroot -psecret -e 'CREATE DATABASE IF NOT EXISTS `{}`;'", id, name)
            } else {
                format!("mysql -uroot -e 'CREATE DATABASE IF NOT EXISTS `{}`;'", name)
            };

            let output = Command::new("sh")
                .args(["-c", &cmd])
                .output()
                .map_err(|e| format!("Failed to create database: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "Failed to create database: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        "postgresql" => {
            let cmd = if let Some(id) = container_id {
                format!("docker exec {} psql -U postgres -c 'CREATE DATABASE \"{}\";'", id, name)
            } else {
                format!("psql -U postgres -c 'CREATE DATABASE \"{}\";'", name)
            };

            let output = Command::new("sh")
                .args(["-c", &cmd])
                .output()
                .map_err(|e| format!("Failed to create database: {}", e))?;

            if !output.status.success() && !String::from_utf8_lossy(&output.stderr).contains("already exists") {
                return Err(format!(
                    "Failed to create database: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        _ => return Err(format!("Unsupported database type: {}", db_type)),
    }

    Ok(())
}

#[tauri::command]
pub fn drop_database(db_type: String, name: String, container_id: Option<String>) -> Result<(), String> {
    // Prevent dropping system databases
    let protected = ["information_schema", "mysql", "performance_schema", "sys", "postgres", "template0", "template1"];
    if protected.contains(&name.as_str()) {
        return Err("Cannot drop system database".to_string());
    }

    match db_type.as_str() {
        "mysql" => {
            let cmd = if let Some(id) = container_id {
                format!("docker exec {} mysql -uroot -psecret -e 'DROP DATABASE IF EXISTS `{}`;'", id, name)
            } else {
                format!("mysql -uroot -e 'DROP DATABASE IF EXISTS `{}`;'", name)
            };

            let output = Command::new("sh")
                .args(["-c", &cmd])
                .output()
                .map_err(|e| format!("Failed to drop database: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "Failed to drop database: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        "postgresql" => {
            let cmd = if let Some(id) = container_id {
                format!("docker exec {} psql -U postgres -c 'DROP DATABASE IF EXISTS \"{}\";'", id, name)
            } else {
                format!("psql -U postgres -c 'DROP DATABASE IF EXISTS \"{}\";'", name)
            };

            let output = Command::new("sh")
                .args(["-c", &cmd])
                .output()
                .map_err(|e| format!("Failed to drop database: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "Failed to drop database: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        _ => return Err(format!("Unsupported database type: {}", db_type)),
    }

    Ok(())
}
