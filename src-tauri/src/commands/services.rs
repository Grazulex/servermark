use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceStatus {
    pub id: String,
    pub name: String,
    pub status: String,
    pub pid: Option<u32>,
}

#[tauri::command]
pub fn get_service_status(service_name: String) -> Result<ServiceStatus, String> {
    let output = Command::new("systemctl")
        .args(["is-active", &service_name])
        .output()
        .map_err(|e| format!("Failed to check service status: {}", e))?;

    let status = if output.status.success() {
        "running"
    } else {
        "stopped"
    }.to_string();

    // Try to get PID if running
    let pid = if status == "running" {
        Command::new("systemctl")
            .args(["show", &service_name, "--property=MainPID", "--value"])
            .output()
            .ok()
            .and_then(|o| {
                String::from_utf8_lossy(&o.stdout)
                    .trim()
                    .parse::<u32>()
                    .ok()
            })
            .filter(|&p| p > 0)
    } else {
        None
    };

    Ok(ServiceStatus {
        id: service_name.clone(),
        name: service_name,
        status,
        pid,
    })
}

#[tauri::command]
pub fn start_service(service_name: String) -> Result<(), String> {
    let output = Command::new("pkexec")
        .args(["systemctl", "start", &service_name])
        .output()
        .map_err(|e| format!("Failed to start service: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

#[tauri::command]
pub fn stop_service(service_name: String) -> Result<(), String> {
    let output = Command::new("pkexec")
        .args(["systemctl", "stop", &service_name])
        .output()
        .map_err(|e| format!("Failed to stop service: {}", e))?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
