use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Site {
    pub id: String,
    pub name: String,
    pub path: String,
    pub domain: String,
    pub php_version: String,
    pub secured: bool,
    pub site_type: SiteType,
    pub proxy_target: Option<String>,
    pub laravel: Option<LaravelInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SiteType {
    Laravel,
    Symfony,
    WordPress,
    Static,
    Proxy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaravelInfo {
    pub detected: bool,
    pub version: Option<String>,
    pub constraint: Option<String>,
    pub php_version: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SitesConfig {
    pub sites: Vec<Site>,
    pub tld: String,
    pub sites_path: String,
}

impl Default for SitesConfig {
    fn default() -> Self {
        Self {
            sites: Vec::new(),
            tld: "test".to_string(),
            sites_path: dirs::home_dir()
                .map(|h| h.join("Code").to_string_lossy().to_string())
                .unwrap_or_else(|| "/home".to_string()),
        }
    }
}

fn get_config_path() -> std::path::PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("/tmp"))
        .join("servermark")
        .join("sites.json")
}

pub fn load_sites_config() -> SitesConfig {
    let path = get_config_path();
    if path.exists() {
        fs::read_to_string(&path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default()
    } else {
        SitesConfig::default()
    }
}

fn save_config(config: &SitesConfig) -> Result<(), String> {
    let path = get_config_path();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create config dir: {}", e))?;
    }
    let content = serde_json::to_string_pretty(config)
        .map_err(|e| format!("Failed to serialize config: {}", e))?;
    fs::write(&path, content).map_err(|e| format!("Failed to write config: {}", e))?;
    Ok(())
}

/// Detect the type of site based on directory contents
#[tauri::command]
pub fn detect_site_type(path: String) -> Result<SiteType, String> {
    let path = Path::new(&path);

    if !path.exists() {
        return Err("Path does not exist".to_string());
    }

    // Check for Laravel (artisan file)
    if path.join("artisan").exists() && path.join("composer.json").exists() {
        return Ok(SiteType::Laravel);
    }

    // Check for Symfony (bin/console)
    if path.join("bin/console").exists() && path.join("symfony.lock").exists() {
        return Ok(SiteType::Symfony);
    }

    // Check for WordPress (wp-config.php or wp-content)
    if path.join("wp-config.php").exists() || path.join("wp-content").exists() {
        return Ok(SiteType::WordPress);
    }

    // Default to static
    Ok(SiteType::Static)
}

/// List all configured sites
#[tauri::command]
pub fn list_sites() -> Result<Vec<Site>, String> {
    let config = load_sites_config();
    Ok(config.sites)
}

/// Add a new site
#[tauri::command]
pub fn add_site(path: String, name: Option<String>, php_version: Option<String>) -> Result<Site, String> {
    let site_path = Path::new(&path);

    if !site_path.exists() {
        return Err("Path does not exist".to_string());
    }

    let mut config = load_sites_config();

    // Generate name from directory if not provided
    let site_name = name.unwrap_or_else(|| {
        site_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "site".to_string())
    });

    // Check if site already exists
    if config.sites.iter().any(|s| s.path == path || s.name == site_name) {
        return Err("Site already exists".to_string());
    }

    // Detect site type
    let site_type = detect_site_type(path.clone()).unwrap_or(SiteType::Static);

    // Detect Laravel info if applicable
    let laravel = if matches!(site_type, SiteType::Laravel) {
        detect_laravel_info(&path)
    } else {
        None
    };

    // Get active PHP version if not specified
    let php = php_version.unwrap_or_else(|| get_active_php_version());

    let site = Site {
        id: format!("site-{}", chrono::Utc::now().timestamp_millis()),
        name: site_name.clone(),
        path: path.clone(),
        domain: format!("{}.{}", site_name.to_lowercase().replace(' ', "-"), config.tld),
        php_version: php,
        secured: false,
        site_type,
        proxy_target: None,
        laravel,
    };

    // Update .env file and fix permissions for Laravel projects
    if matches!(site.site_type, SiteType::Laravel) {
        update_laravel_env(&site)?;
        // Fix Docker hostnames (mysql -> 127.0.0.1, redis -> 127.0.0.1, etc.)
        let _ = super::webserver::fix_docker_hostnames_in_env(&site.path);
        // Fix storage/cache permissions
        let _ = fix_laravel_permissions_internal(&site.path);
    }

    config.sites.push(site.clone());
    save_config(&config)?;

    // Configure web server (Caddy or Nginx) with single pkexec call
    // This also adds Docker hostnames to /etc/hosts
    super::webserver::webserver_add_site(&site.name)?;

    Ok(site)
}

fn update_laravel_env(site: &Site) -> Result<(), String> {
    let env_path = Path::new(&site.path).join(".env");

    if !env_path.exists() {
        return Ok(()); // No .env file
    }

    let content = fs::read_to_string(&env_path)
        .map_err(|e| format!("Failed to read .env: {}", e))?;

    let protocol = if site.secured { "https" } else { "http" };
    let new_app_url = format!("APP_URL={}://{}", protocol, site.domain);

    // Replace or add APP_URL
    let new_content = if content.contains("APP_URL=") {
        // Replace existing APP_URL
        let lines: Vec<&str> = content.lines().collect();
        let updated_lines: Vec<String> = lines.iter()
            .map(|line| {
                if line.starts_with("APP_URL=") {
                    new_app_url.clone()
                } else {
                    line.to_string()
                }
            })
            .collect();
        updated_lines.join("\n")
    } else {
        // Add APP_URL after APP_NAME if exists, or at the beginning
        if content.contains("APP_NAME=") {
            content.replace("APP_NAME=", &format!("{}\nAPP_NAME=", new_app_url))
        } else {
            format!("{}\n{}", new_app_url, content)
        }
    };

    fs::write(&env_path, new_content)
        .map_err(|e| format!("Failed to write .env: {}", e))?;

    Ok(())
}

/// Remove a site
#[tauri::command]
pub fn remove_site(id: String) -> Result<(), String> {
    let mut config = load_sites_config();

    let site = config.sites.iter().find(|s| s.id == id).cloned();

    if let Some(site) = site {
        // Remove from config first
        config.sites.retain(|s| s.id != id);
        save_config(&config)?;

        // Remove from web server config (handles hosts, caddy/nginx config, reload)
        super::webserver::webserver_remove_site(&site.name)?;

        Ok(())
    } else {
        Err("Site not found".to_string())
    }
}

/// Update site PHP version
#[tauri::command]
pub fn update_site_php(id: String, php_version: String) -> Result<Site, String> {
    let mut config = load_sites_config();

    if let Some(site) = config.sites.iter_mut().find(|s| s.id == id) {
        site.php_version = php_version;

        let updated = site.clone();
        save_config(&config)?;

        // Update web server config (single pkexec call)
        super::webserver::webserver_update_site(&updated.name)?;

        Ok(updated)
    } else {
        Err("Site not found".to_string())
    }
}

/// Fix Laravel permissions for storage and bootstrap/cache directories
#[tauri::command]
pub fn fix_laravel_permissions(path: String) -> Result<(), String> {
    let site_path = Path::new(&path);

    if !site_path.exists() {
        return Err("Path does not exist".to_string());
    }

    // Check if it's a Laravel project
    if !site_path.join("artisan").exists() {
        return Err("Not a Laravel project".to_string());
    }

    let storage_path = site_path.join("storage");
    let cache_path = site_path.join("bootstrap/cache");

    // Get current user
    let current_user = std::env::var("USER").unwrap_or_else(|_| "www-data".to_string());

    // Script to fix permissions - uses pkexec for single password prompt
    let script = format!(
        r#"
set -e

# Fix storage directory
if [ -d "{storage}" ]; then
    chown -R {user}:www-data "{storage}"
    chmod -R 775 "{storage}"
    find "{storage}" -type d -exec chmod 775 {{}} \;
    find "{storage}" -type f -exec chmod 664 {{}} \;
fi

# Fix bootstrap/cache directory
if [ -d "{cache}" ]; then
    chown -R {user}:www-data "{cache}"
    chmod -R 775 "{cache}"
fi

# Ensure directories exist
mkdir -p "{storage}/app/public"
mkdir -p "{storage}/framework/cache"
mkdir -p "{storage}/framework/sessions"
mkdir -p "{storage}/framework/views"
mkdir -p "{storage}/logs"
mkdir -p "{cache}"

# Set permissions on newly created dirs
chown -R {user}:www-data "{storage}"
chown -R {user}:www-data "{cache}"
chmod -R 775 "{storage}"
chmod -R 775 "{cache}"

echo "Permissions fixed successfully"
"#,
        storage = storage_path.display(),
        cache = cache_path.display(),
        user = current_user
    );

    let output = Command::new("pkexec")
        .args(["bash", "-c", &script])
        .output()
        .map_err(|e| format!("Failed to fix permissions: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to fix permissions: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(())
}

/// Internal function to fix permissions (called during site creation)
pub fn fix_laravel_permissions_internal(path: &str) -> Result<(), String> {
    fix_laravel_permissions(path.to_string())
}

/// Get sites configuration (tld, sites_path)
#[tauri::command]
pub fn get_sites_config() -> SitesConfig {
    load_sites_config()
}

/// Update sites configuration
#[tauri::command]
#[allow(non_snake_case)]
pub fn update_sites_config(tld: Option<String>, sitesPath: Option<String>) -> Result<SitesConfig, String> {
    let mut config = load_sites_config();

    if let Some(tld) = tld {
        config.tld = tld;
    }
    if let Some(path) = sitesPath {
        config.sites_path = path;
    }

    save_config(&config)?;
    Ok(config)
}

// Helper functions

fn detect_laravel_info(path: &str) -> Option<LaravelInfo> {
    let composer_path = Path::new(path).join("composer.json");
    if let Ok(content) = fs::read_to_string(&composer_path) {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
            let constraint = json
                .get("require")
                .and_then(|r| r.get("laravel/framework"))
                .and_then(|v| v.as_str())
                .map(|s| s.to_string());

            return Some(LaravelInfo {
                detected: true,
                version: None, // Would need to run artisan --version
                constraint,
                php_version: json
                    .get("require")
                    .and_then(|r| r.get("php"))
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string()),
            });
        }
    }
    None
}

fn get_active_php_version() -> String {
    Command::new("php")
        .args(["-r", "echo PHP_MAJOR_VERSION . '.' . PHP_MINOR_VERSION;"])
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_else(|_| "8.3".to_string())
}

// ============================================================================
// Project Creation & Cloning
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
    pub versions: Vec<String>,
    pub default_version: String,
    pub create_command: String,
}

/// Get available framework templates
#[tauri::command]
pub fn get_framework_templates() -> Vec<FrameworkTemplate> {
    vec![
        FrameworkTemplate {
            id: "laravel".to_string(),
            name: "Laravel".to_string(),
            description: "The PHP Framework for Web Artisans".to_string(),
            versions: vec!["12".to_string(), "11".to_string(), "10".to_string()],
            default_version: "12".to_string(),
            create_command: "composer create-project laravel/laravel".to_string(),
        },
        FrameworkTemplate {
            id: "symfony".to_string(),
            name: "Symfony".to_string(),
            description: "High Performance PHP Framework".to_string(),
            versions: vec!["7.0".to_string(), "6.4".to_string(), "5.4".to_string()],
            default_version: "7.0".to_string(),
            create_command: "composer create-project symfony/skeleton".to_string(),
        },
        FrameworkTemplate {
            id: "wordpress".to_string(),
            name: "WordPress".to_string(),
            description: "Open Source Blogging Platform".to_string(),
            versions: vec!["latest".to_string()],
            default_version: "latest".to_string(),
            create_command: "wp core download".to_string(),
        },
        FrameworkTemplate {
            id: "static".to_string(),
            name: "Static Site".to_string(),
            description: "Simple HTML/CSS/JS site".to_string(),
            versions: vec!["n/a".to_string()],
            default_version: "n/a".to_string(),
            create_command: "mkdir -p".to_string(),
        },
    ]
}

/// Create a new project with a framework
#[tauri::command]
pub fn create_project(
    name: String,
    framework: String,
    version: Option<String>,
    php_version: Option<String>,
    path: Option<String>,
) -> Result<Site, String> {
    let config = load_sites_config();

    // Use provided path or fall back to default sites_path
    let base_path = path.unwrap_or_else(|| config.sites_path.clone());
    let project_path = Path::new(&base_path).join(&name);

    if project_path.exists() {
        return Err(format!("Directory {} already exists", project_path.display()));
    }

    // Create parent directory if needed
    fs::create_dir_all(&base_path)
        .map_err(|e| format!("Failed to create sites directory: {}", e))?;

    let php = php_version.unwrap_or_else(get_active_php_version);

    match framework.as_str() {
        "laravel" => {
            let ver = version.unwrap_or_else(|| "11".to_string());
            let constraint = if ver == "12" {
                "laravel/laravel"
            } else {
                &format!("laravel/laravel:^{}", ver)
            };

            let output = Command::new("composer")
                .args(["create-project", constraint, &name])
                .current_dir(&base_path)
                .output()
                .map_err(|e| format!("Failed to run composer: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "Failed to create Laravel project: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        "symfony" => {
            let ver = version.unwrap_or_else(|| "7.0".to_string());
            let output = Command::new("composer")
                .args([
                    "create-project",
                    &format!("symfony/skeleton:^{}", ver),
                    &name,
                ])
                .current_dir(&base_path)
                .output()
                .map_err(|e| format!("Failed to run composer: {}", e))?;

            if !output.status.success() {
                return Err(format!(
                    "Failed to create Symfony project: {}",
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
        }
        "wordpress" => {
            // Create directory and download WordPress
            fs::create_dir_all(&project_path)
                .map_err(|e| format!("Failed to create directory: {}", e))?;

            let output = Command::new("wp")
                .args(["core", "download"])
                .current_dir(&project_path)
                .output()
                .map_err(|e| format!("Failed to run wp-cli: {}", e))?;

            if !output.status.success() {
                // Fallback: try curl
                let curl_output = Command::new("curl")
                    .args([
                        "-o",
                        "wordpress.tar.gz",
                        "https://wordpress.org/latest.tar.gz",
                    ])
                    .current_dir(&project_path)
                    .output();

                if curl_output.is_ok() {
                    let _ = Command::new("tar")
                        .args(["xzf", "wordpress.tar.gz", "--strip-components=1"])
                        .current_dir(&project_path)
                        .output();
                    let _ = fs::remove_file(project_path.join("wordpress.tar.gz"));
                } else {
                    return Err("Failed to download WordPress. Install wp-cli or check your connection.".to_string());
                }
            }
        }
        "static" => {
            // Create directory with basic index.html
            fs::create_dir_all(&project_path)
                .map_err(|e| format!("Failed to create directory: {}", e))?;

            let index_html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{}</title>
</head>
<body>
    <h1>Welcome to {}</h1>
    <p>Your static site is ready.</p>
</body>
</html>"#,
                name, name
            );

            fs::write(project_path.join("index.html"), index_html)
                .map_err(|e| format!("Failed to write index.html: {}", e))?;
        }
        _ => {
            return Err(format!("Unknown framework: {}", framework));
        }
    }

    // Now add the site
    add_site(
        project_path.to_string_lossy().to_string(),
        Some(name),
        Some(php),
    )
}

/// Clone a Git repository and set up as a site
#[tauri::command]
pub fn clone_repository(
    repo_url: String,
    name: Option<String>,
    php_version: Option<String>,
) -> Result<Site, String> {
    let config = load_sites_config();

    // Extract name from repo URL if not provided
    let project_name = name.unwrap_or_else(|| {
        repo_url
            .trim_end_matches('/')
            .split('/')
            .last()
            .unwrap_or("project")
            .trim_end_matches(".git")
            .to_string()
    });

    let project_path = Path::new(&config.sites_path).join(&project_name);

    if project_path.exists() {
        return Err(format!("Directory {} already exists", project_path.display()));
    }

    // Create parent directory if needed
    fs::create_dir_all(&config.sites_path)
        .map_err(|e| format!("Failed to create sites directory: {}", e))?;

    // Clone the repository
    let output = Command::new("git")
        .args(["clone", &repo_url, &project_name])
        .current_dir(&config.sites_path)
        .output()
        .map_err(|e| format!("Failed to run git: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "Failed to clone repository: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Run composer install if composer.json exists
    if project_path.join("composer.json").exists() {
        let _ = Command::new("composer")
            .args(["install"])
            .current_dir(&project_path)
            .output();
    }

    // Run npm install if package.json exists
    if project_path.join("package.json").exists() {
        let _ = Command::new("npm")
            .args(["install"])
            .current_dir(&project_path)
            .output();
    }

    // Copy .env.example to .env if exists (Laravel)
    let env_example = project_path.join(".env.example");
    let env_file = project_path.join(".env");
    if env_example.exists() && !env_file.exists() {
        let _ = fs::copy(&env_example, &env_file);
    }

    // Generate Laravel key if artisan exists
    if project_path.join("artisan").exists() {
        let _ = Command::new("php")
            .args(["artisan", "key:generate"])
            .current_dir(&project_path)
            .output();
    }

    // Add the site
    add_site(
        project_path.to_string_lossy().to_string(),
        Some(project_name),
        php_version,
    )
}

// ============================================================================
// Secure / Unsecure Sites
// ============================================================================

/// Secure a site with HTTPS
#[tauri::command]
pub fn secure_site(id: String) -> Result<Site, String> {
    let mut config = load_sites_config();

    if let Some(site) = config.sites.iter_mut().find(|s| s.id == id) {
        site.secured = true;

        // Update .env for Laravel
        if matches!(site.site_type, SiteType::Laravel) {
            let _ = update_laravel_env(site);
        }

        let updated = site.clone();
        save_config(&config)?;

        // Update web server config (single pkexec call)
        super::webserver::webserver_update_site(&updated.name)?;

        Ok(updated)
    } else {
        Err("Site not found".to_string())
    }
}

/// Unsecure a site (remove HTTPS)
#[tauri::command]
pub fn unsecure_site(id: String) -> Result<Site, String> {
    let mut config = load_sites_config();

    if let Some(site) = config.sites.iter_mut().find(|s| s.id == id) {
        site.secured = false;

        // Update .env for Laravel
        if matches!(site.site_type, SiteType::Laravel) {
            let _ = update_laravel_env(site);
        }

        let updated = site.clone();
        save_config(&config)?;

        // Update web server config (single pkexec call)
        super::webserver::webserver_update_site(&updated.name)?;

        Ok(updated)
    } else {
        Err("Site not found".to_string())
    }
}
