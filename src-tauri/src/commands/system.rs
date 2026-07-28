use serde::Serialize;
use tauri::command;
use std::process::Command;

#[derive(Debug, Serialize)]
pub struct SystemInfo {
    pub platform: String,
    pub arch: String,
    pub os_version: String,
    pub cpu_cores: u32,
    pub memory_gb: u64,
    pub app_version: String,
}

fn get_memory_gb() -> u64 {
    #[cfg(target_os = "linux")]
    {
        if let Ok(output) = Command::new("sh").args(["-c", "grep MemTotal /proc/meminfo | awk '{print }'"]).output() {
            if let Ok(kb) = String::from_utf8(output.stdout).unwrap_or_default().trim().parse::<u64>() {
                return kb / (1024 * 1024);
            }
        }
    }
    #[cfg(target_os = "macos")]
    {
        if let Ok(output) = Command::new("sh").args(["-c", "sysctl -n hw.memsize"]).output() {
            if let Ok(bytes) = String::from_utf8(output.stdout).unwrap_or_default().trim().parse::<u64>() {
                return bytes / (1024 * 1024 * 1024);
            }
        }
    }
    #[cfg(target_os = "windows")]
    {
        if let Ok(output) = Command::new("wmic").args(["memorychip", "get", "Capacity"]).output() {
            let text = String::from_utf8_lossy(&output.stdout);
            let total: u64 = text.lines()
                .filter_map(|l| l.trim().parse::<u64>().ok())
                .sum();
            if total > 0 {
                return total / (1024 * 1024 * 1024);
            }
        }
    }
    8
}

#[command]
pub async fn get_system_info() -> Result<SystemInfo, String> {
    Ok(SystemInfo {
        platform: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        os_version: std::env::consts::FAMILY.to_string(),
        cpu_cores: num_cpus::get() as u32,
        memory_gb: get_memory_gb(),
        app_version: "1.0.0".into(),
    })
}

#[command]
pub async fn open_url(url: String) -> Result<(), String> {
    open::that(&url).map_err(|e| format!("Failed to open URL: {e}"))
}
