use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeInfo {
    pub runtime: String,
    pub version: String,
    pub api_version: String,
    pub available: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Container {
    pub id: String,
    pub name: String,
    pub image: String,
    pub status: String,
    pub ports: Vec<PortMapping>,
    pub created: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub host: u16,
    pub container: u16,
    pub protocol: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContainerParams {
    pub image: String,
    pub name: String,
    pub ports: Vec<PortMapping>,
    pub environment: std::collections::HashMap<String, String>,
    pub volumes: Option<Vec<VolumeMapping>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMapping {
    pub name: String,
    pub container: String,
}

/// Detect available container runtime (Docker or Podman)
#[tauri::command]
pub fn detect_container_runtime() -> Result<RuntimeInfo, String> {
    // Try Docker first
    if let Ok(output) = Command::new("docker")
        .args(["version", "--format", "{{.Server.Version}}"])
        .output()
    {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();

            // Get API version
            let api_version = Command::new("docker")
                .args(["version", "--format", "{{.Server.APIVersion}}"])
                .output()
                .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
                .unwrap_or_default();

            return Ok(RuntimeInfo {
                runtime: "docker".to_string(),
                version,
                api_version,
                available: true,
            });
        }
    }

    // Try Podman
    if let Ok(output) = Command::new("podman")
        .args(["version", "--format", "{{.Version}}"])
        .output()
    {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout).trim().to_string();

            return Ok(RuntimeInfo {
                runtime: "podman".to_string(),
                version,
                api_version: String::new(),
                available: true,
            });
        }
    }

    Ok(RuntimeInfo {
        runtime: "none".to_string(),
        version: String::new(),
        api_version: String::new(),
        available: false,
    })
}

/// Get the container runtime command (docker or podman)
fn get_runtime_cmd() -> Result<String, String> {
    // Check Docker first
    if Command::new("docker")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return Ok("docker".to_string());
    }

    // Check Podman
    if Command::new("podman")
        .arg("--version")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
    {
        return Ok("podman".to_string());
    }

    Err("No container runtime found. Please install Docker or Podman.".to_string())
}

/// List all ServerMark containers
#[tauri::command]
pub fn list_containers() -> Result<Vec<Container>, String> {
    let runtime = get_runtime_cmd()?;

    let output = Command::new(&runtime)
        .args([
            "ps",
            "-a",
            "--filter",
            "name=servermark-",
            "--format",
            "{{.ID}}|{{.Names}}|{{.Image}}|{{.Status}}|{{.Ports}}|{{.CreatedAt}}",
        ])
        .output()
        .map_err(|e| format!("Failed to list containers: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut containers = Vec::new();

    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('|').collect();
        if parts.len() >= 5 {
            let status_str = parts[3].to_lowercase();
            let status = if status_str.contains("up") {
                "running"
            } else if status_str.contains("exited") {
                "stopped"
            } else if status_str.contains("paused") {
                "paused"
            } else {
                "stopped"
            };

            // Parse ports (simplified)
            let ports = parse_ports(parts[4]);

            containers.push(Container {
                id: parts[0].to_string(),
                name: parts[1].to_string(),
                image: parts[2].to_string(),
                status: status.to_string(),
                ports,
                created: parts.get(5).unwrap_or(&"").to_string(),
            });
        }
    }

    Ok(containers)
}

/// Parse Docker port string like "0.0.0.0:3306->3306/tcp"
fn parse_ports(port_str: &str) -> Vec<PortMapping> {
    let mut ports = Vec::new();

    for mapping in port_str.split(',') {
        let mapping = mapping.trim();
        if mapping.is_empty() {
            continue;
        }

        // Format: 0.0.0.0:3306->3306/tcp or :::3306->3306/tcp
        if let Some(arrow_pos) = mapping.find("->") {
            let host_part = &mapping[..arrow_pos];
            let container_part = &mapping[arrow_pos + 2..];

            // Extract host port
            let host_port = host_part
                .rsplit(':')
                .next()
                .and_then(|p| p.parse::<u16>().ok());

            // Extract container port and protocol
            let (container_port, protocol) = if let Some(slash_pos) = container_part.find('/') {
                (
                    container_part[..slash_pos].parse::<u16>().ok(),
                    &container_part[slash_pos + 1..],
                )
            } else {
                (container_part.parse::<u16>().ok(), "tcp")
            };

            if let (Some(hp), Some(cp)) = (host_port, container_port) {
                ports.push(PortMapping {
                    host: hp,
                    container: cp,
                    protocol: protocol.to_string(),
                });
            }
        }
    }

    ports
}

/// Create and start a new container
#[tauri::command]
pub fn create_container(params: CreateContainerParams) -> Result<String, String> {
    let runtime = get_runtime_cmd()?;

    let mut args = vec![
        "run".to_string(),
        "-d".to_string(),
        "--name".to_string(),
        params.name.clone(),
    ];

    // Add port mappings
    for port in &params.ports {
        args.push("-p".to_string());
        args.push(format!(
            "{}:{}/{}",
            port.host, port.container, port.protocol
        ));
    }

    // Add environment variables
    for (key, value) in &params.environment {
        args.push("-e".to_string());
        args.push(format!("{}={}", key, value));
    }

    // Add volumes
    if let Some(volumes) = &params.volumes {
        for vol in volumes {
            args.push("-v".to_string());
            args.push(format!("{}:{}", vol.name, vol.container));
        }
    }

    // Add restart policy
    args.push("--restart".to_string());
    args.push("unless-stopped".to_string());

    // Add image
    args.push(params.image);

    let output = Command::new(&runtime)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to create container: {}", e))?;

    if output.status.success() {
        let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(container_id)
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Start a container
#[tauri::command]
pub fn start_container(id: String) -> Result<(), String> {
    let runtime = get_runtime_cmd()?;

    let output = Command::new(&runtime)
        .args(["start", &id])
        .output()
        .map_err(|e| format!("Failed to start container: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Stop a container
#[tauri::command]
pub fn stop_container(id: String) -> Result<(), String> {
    let runtime = get_runtime_cmd()?;

    let output = Command::new(&runtime)
        .args(["stop", &id])
        .output()
        .map_err(|e| format!("Failed to stop container: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Remove a container
#[tauri::command]
pub fn remove_container(id: String, force: bool) -> Result<(), String> {
    let runtime = get_runtime_cmd()?;

    let mut args = vec!["rm"];
    if force {
        args.push("-f");
    }
    args.push(&id);

    let output = Command::new(&runtime)
        .args(&args)
        .output()
        .map_err(|e| format!("Failed to remove container: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// Get container logs
#[tauri::command]
pub fn get_container_logs(id: String, lines: Option<u32>) -> Result<String, String> {
    let runtime = get_runtime_cmd()?;

    let lines_str = lines.unwrap_or(100).to_string();
    let output = Command::new(&runtime)
        .args(["logs", "--tail", &lines_str, &id])
        .output()
        .map_err(|e| format!("Failed to get logs: {}", e))?;

    if output.status.success() {
        // Combine stdout and stderr (logs can be on either)
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        Ok(format!("{}{}", stdout, stderr))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
