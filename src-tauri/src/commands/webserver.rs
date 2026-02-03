use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

use super::sites::{load_sites_config, Site, SiteType};

/// Common Docker hostnames that should resolve to localhost
#[allow(dead_code)]
const DOCKER_HOSTNAMES: &[&str] = &[
    "mysql",
    "mariadb",
    "postgres",
    "redis",
    "memcached",
    "mailhog",
    "mailpit",
    "meilisearch",
    "elasticsearch",
    "mongo",
    "mongodb",
    "rabbitmq",
    "minio",
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebServerConfig {
    pub active: String, // "caddy" or "nginx"
    pub caddy_installed: bool,
    pub nginx_installed: bool,
}

fn get_webserver_config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("servermark")
        .join("webserver.json")
}

fn load_webserver_config() -> WebServerConfig {
    let path = get_webserver_config_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_else(|| WebServerConfig {
                active: "caddy".to_string(),
                caddy_installed: false,
                nginx_installed: false,
            })
    } else {
        WebServerConfig {
            active: "caddy".to_string(),
            caddy_installed: false,
            nginx_installed: false,
        }
    }
}

fn save_webserver_config(config: &WebServerConfig) -> Result<(), String> {
    let path = get_webserver_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {}", e))?;
    }
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("Failed to write config: {}", e))?;
    Ok(())
}

/// Generate Caddy config for a site
fn generate_caddy_site_config(site: &Site) -> String {
    let public_path = if matches!(site.site_type, SiteType::Laravel | SiteType::Symfony) {
        format!("{}/public", site.path)
    } else {
        site.path.clone()
    };

    let php_socket = format!("/var/run/php/php{}-fpm.sock", site.php_version);

    let (domain, tls_directive) = if site.secured {
        (format!("https://{}", site.domain), "    tls internal\n")
    } else {
        (format!("http://{}", site.domain), "")
    };

    format!(
        r#"{domain} {{
{tls_directive}    root * {public_path}

    php_fastcgi unix/{php_socket}
    file_server

    encode gzip
}}
"#,
        domain = domain,
        tls_directive = tls_directive,
        public_path = public_path,
        php_socket = php_socket,
    )
}

/// Generate Nginx config for a site
fn generate_nginx_site_config(site: &Site) -> String {
    let public_path = if matches!(site.site_type, SiteType::Laravel | SiteType::Symfony) {
        format!("{}/public", site.path)
    } else {
        site.path.clone()
    };

    let php_socket = format!("/var/run/php/php{}-fpm.sock", site.php_version);

    let ssl_config = if site.secured {
        format!(
            r#"
    listen 443 ssl;
    ssl_certificate /etc/servermark/ssl/{domain}.crt;
    ssl_certificate_key /etc/servermark/ssl/{domain}.key;
"#,
            domain = site.domain
        )
    } else {
        "    listen 80;".to_string()
    };

    format!(
        r#"server {{
    server_name {domain};
{ssl_config}
    root {public_path};
    index index.php index.html;

    location / {{
        try_files $uri $uri/ /index.php?$query_string;
    }}

    location ~ \.php$ {{
        fastcgi_pass unix:{php_socket};
        fastcgi_param SCRIPT_FILENAME $realpath_root$fastcgi_script_name;
        include fastcgi_params;
    }}

    location ~ /\.(?!well-known).* {{
        deny all;
    }}
}}
"#,
        domain = site.domain,
        ssl_config = ssl_config,
        public_path = public_path,
        php_socket = php_socket,
    )
}

/// Fix Docker hostnames in a Laravel .env file
/// Replaces hostnames like 'mysql', 'redis' with '127.0.0.1'
pub fn fix_docker_hostnames_in_env(site_path: &str) -> Result<(), String> {
    let env_path = Path::new(site_path).join(".env");

    if !env_path.exists() {
        return Ok(()); // No .env file to fix
    }

    let content =
        fs::read_to_string(&env_path).map_err(|e| format!("Failed to read .env: {}", e))?;

    let mut modified = content.clone();
    let mut changes_made = false;

    // Common Docker hostname patterns to replace
    let patterns = [
        ("DB_HOST=mysql", "DB_HOST=127.0.0.1"),
        ("DB_HOST=mariadb", "DB_HOST=127.0.0.1"),
        ("DB_HOST=postgres", "DB_HOST=127.0.0.1"),
        ("REDIS_HOST=redis", "REDIS_HOST=127.0.0.1"),
        ("CACHE_HOST=redis", "CACHE_HOST=127.0.0.1"),
        ("SESSION_HOST=redis", "SESSION_HOST=127.0.0.1"),
        ("QUEUE_HOST=redis", "QUEUE_HOST=127.0.0.1"),
        ("MAIL_HOST=mailhog", "MAIL_HOST=127.0.0.1"),
        ("MAIL_HOST=mailpit", "MAIL_HOST=127.0.0.1"),
        ("MEILISEARCH_HOST=meilisearch", "MEILISEARCH_HOST=127.0.0.1"),
        (
            "ELASTICSEARCH_HOST=elasticsearch",
            "ELASTICSEARCH_HOST=127.0.0.1",
        ),
        ("MONGODB_HOST=mongo", "MONGODB_HOST=127.0.0.1"),
        ("MONGODB_HOST=mongodb", "MONGODB_HOST=127.0.0.1"),
    ];

    for (from, to) in patterns {
        if modified.contains(from) {
            modified = modified.replace(from, to);
            changes_made = true;
        }
    }

    if changes_made {
        fs::write(&env_path, &modified).map_err(|e| format!("Failed to write .env: {}", e))?;
    }

    Ok(())
}

/// Build a comprehensive shell script for all web server operations
/// This runs with a single pkexec call to minimize password prompts
pub fn build_webserver_script(
    operation: &str,
    sites: &[Site],
    active_server: &str,
    site_name: Option<&str>,
) -> String {
    let mut script = String::from("#!/bin/bash\nset -e\n\n");

    // Ensure directories exist and add Docker hostnames to /etc/hosts
    script.push_str(r#"
# Ensure directories exist
mkdir -p /etc/caddy/sites.d
mkdir -p /etc/nginx/sites-available
mkdir -p /etc/nginx/sites-enabled
mkdir -p /etc/servermark/ssl
chown -R root:root /etc/caddy/sites.d 2>/dev/null || true
chmod 755 /etc/caddy/sites.d

# Add common Docker hostnames to /etc/hosts (for Laravel projects migrated from Docker)
for hostname in mysql mariadb postgres redis memcached mailhog mailpit meilisearch elasticsearch mongo mongodb rabbitmq minio; do
    grep -q "127.0.0.1.*\b${hostname}\b" /etc/hosts 2>/dev/null || echo "127.0.0.1 ${hostname}" >> /etc/hosts
done

"#);

    match operation {
        "sync_all" => {
            // Generate all site configs for the active server
            if active_server == "caddy" {
                script.push_str("# Clear old Caddy configs\nrm -f /etc/caddy/sites.d/*.conf\n\n");
                for site in sites {
                    let config = generate_caddy_site_config(site);
                    script.push_str(&format!(
                        "# Site: {}\ncat > '/etc/caddy/sites.d/{}.conf' << 'SITEEOF'\n{}\nSITEEOF\n\n",
                        site.name, site.name, config
                    ));
                }
                script.push_str("systemctl reload caddy || systemctl restart caddy\n");
            } else {
                script.push_str("# Clear old Nginx configs\nrm -f /etc/nginx/sites-enabled/servermark-*\nrm -f /etc/nginx/sites-available/servermark-*\n\n");
                for site in sites {
                    let config = generate_nginx_site_config(site);
                    let filename = format!("servermark-{}", site.name);
                    script.push_str(&format!(
                        "# Site: {}\ncat > '/etc/nginx/sites-available/{}' << 'SITEEOF'\n{}\nSITEEOF\nln -sf '/etc/nginx/sites-available/{}' '/etc/nginx/sites-enabled/{}'\n\n",
                        site.name, filename, config, filename, filename
                    ));

                    // Generate self-signed SSL cert if needed
                    if site.secured {
                        script.push_str(&format!(
                            r#"if [ ! -f "/etc/servermark/ssl/{domain}.crt" ]; then
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout "/etc/servermark/ssl/{domain}.key" \
        -out "/etc/servermark/ssl/{domain}.crt" \
        -subj "/CN={domain}" 2>/dev/null
fi
"#,
                            domain = site.domain
                        ));
                    }
                }
                script.push_str("nginx -t && systemctl reload nginx || systemctl restart nginx\n");
            }
        }
        "add_site" | "update_site" => {
            if let Some(name) = site_name {
                if let Some(site) = sites.iter().find(|s| s.name == name) {
                    if active_server == "caddy" {
                        let config = generate_caddy_site_config(site);
                        script.push_str(&format!(
                            "cat > '/etc/caddy/sites.d/{}.conf' << 'SITEEOF'\n{}\nSITEEOF\n",
                            site.name, config
                        ));
                        script.push_str("systemctl reload caddy || systemctl restart caddy\n");
                    } else {
                        let config = generate_nginx_site_config(site);
                        let filename = format!("servermark-{}", site.name);
                        script.push_str(&format!(
                            "cat > '/etc/nginx/sites-available/{}' << 'SITEEOF'\n{}\nSITEEOF\nln -sf '/etc/nginx/sites-available/{}' '/etc/nginx/sites-enabled/{}'\n",
                            filename, config, filename, filename
                        ));

                        if site.secured {
                            script.push_str(&format!(
                                r#"if [ ! -f "/etc/servermark/ssl/{domain}.crt" ]; then
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout "/etc/servermark/ssl/{domain}.key" \
        -out "/etc/servermark/ssl/{domain}.crt" \
        -subj "/CN={domain}" 2>/dev/null
fi
"#,
                                domain = site.domain
                            ));
                        }
                        script.push_str(
                            "nginx -t && systemctl reload nginx || systemctl restart nginx\n",
                        );
                    }

                    // Add hosts entry
                    script.push_str(&format!(
                        "grep -q '{}' /etc/hosts || echo '127.0.0.1 {}' >> /etc/hosts\n",
                        site.domain, site.domain
                    ));
                }
            }
        }
        "remove_site" => {
            if let Some(name) = site_name {
                // Remove from both servers to be safe
                script.push_str(&format!("rm -f '/etc/caddy/sites.d/{}.conf'\n", name));
                script.push_str(&format!(
                    "rm -f '/etc/nginx/sites-available/servermark-{}'\n",
                    name
                ));
                script.push_str(&format!(
                    "rm -f '/etc/nginx/sites-enabled/servermark-{}'\n",
                    name
                ));

                // Reload active server
                if active_server == "caddy" {
                    script.push_str("systemctl reload caddy 2>/dev/null || true\n");
                } else {
                    script.push_str("nginx -t && systemctl reload nginx 2>/dev/null || true\n");
                }

                // Remove from hosts (find domain from sites list)
                if let Some(site) = sites.iter().find(|s| s.name == name) {
                    script.push_str(&format!("sed -i '/{}/d' /etc/hosts\n", site.domain));
                }
            }
        }
        "switch_server" => {
            if active_server == "caddy" {
                script.push_str(
                    r#"
# Switch to Caddy
systemctl stop nginx 2>/dev/null || true
systemctl disable nginx 2>/dev/null || true

# Regenerate all Caddy configs
rm -f /etc/caddy/sites.d/*.conf
"#,
                );
                for site in sites {
                    let config = generate_caddy_site_config(site);
                    script.push_str(&format!(
                        "cat > '/etc/caddy/sites.d/{}.conf' << 'SITEEOF'\n{}\nSITEEOF\n",
                        site.name, config
                    ));
                }
                script.push_str(
                    r#"
systemctl enable caddy
systemctl start caddy
"#,
                );
            } else {
                script.push_str(
                    r#"
# Switch to Nginx
systemctl stop caddy 2>/dev/null || true
systemctl disable caddy 2>/dev/null || true

# Regenerate all Nginx configs
rm -f /etc/nginx/sites-enabled/servermark-*
rm -f /etc/nginx/sites-available/servermark-*
"#,
                );
                for site in sites {
                    let config = generate_nginx_site_config(site);
                    let filename = format!("servermark-{}", site.name);
                    script.push_str(&format!(
                        "cat > '/etc/nginx/sites-available/{}' << 'SITEEOF'\n{}\nSITEEOF\nln -sf '/etc/nginx/sites-available/{}' '/etc/nginx/sites-enabled/{}'\n",
                        filename, config, filename, filename
                    ));

                    if site.secured {
                        script.push_str(&format!(
                            r#"if [ ! -f "/etc/servermark/ssl/{domain}.crt" ]; then
    openssl req -x509 -nodes -days 365 -newkey rsa:2048 \
        -keyout "/etc/servermark/ssl/{domain}.key" \
        -out "/etc/servermark/ssl/{domain}.crt" \
        -subj "/CN={domain}" 2>/dev/null
fi
"#,
                            domain = site.domain
                        ));
                    }
                }
                script.push_str(
                    r#"
systemctl enable nginx
nginx -t && systemctl start nginx
"#,
                );
            }
        }
        _ => {}
    }

    script
}

/// Execute a web server operation with a single pkexec call
pub fn execute_webserver_operation(operation: &str, site_name: Option<&str>) -> Result<(), String> {
    let sites_config = load_sites_config();
    let ws_config = load_webserver_config();

    let script =
        build_webserver_script(operation, &sites_config.sites, &ws_config.active, site_name);

    let output = Command::new("pkexec")
        .args(["bash", "-c", &script])
        .output()
        .map_err(|e| format!("Failed to execute: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Operation failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Sync all sites to the active web server
#[tauri::command]
pub fn sync_webserver_configs() -> Result<(), String> {
    execute_webserver_operation("sync_all", None)
}

/// Switch between Caddy and Nginx
#[tauri::command]
pub fn switch_active_webserver(server: String) -> Result<(), String> {
    let mut config = load_webserver_config();
    config.active = server.clone();
    save_webserver_config(&config)?;

    let sites_config = load_sites_config();
    let script = build_webserver_script("switch_server", &sites_config.sites, &server, None);

    let output = Command::new("pkexec")
        .args(["bash", "-c", &script])
        .output()
        .map_err(|e| format!("Failed to switch server: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to switch server: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Get current active web server
#[tauri::command]
pub fn get_active_webserver() -> String {
    load_webserver_config().active
}

/// Add/update a site in the web server config
pub fn webserver_add_site(site_name: &str) -> Result<(), String> {
    execute_webserver_operation("add_site", Some(site_name))
}

/// Remove a site from the web server config
pub fn webserver_remove_site(site_name: &str) -> Result<(), String> {
    execute_webserver_operation("remove_site", Some(site_name))
}

/// Update a site (e.g., after SSL change)
pub fn webserver_update_site(site_name: &str) -> Result<(), String> {
    execute_webserver_operation("update_site", Some(site_name))
}
