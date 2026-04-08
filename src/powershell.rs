use crate::errors::AppError;
use std::process::Command;

pub fn run_powershell(script: &str) -> Result<String, AppError> {
    let output = Command::new("powershell")
        .args(["-Command", script])
        .output()
        .map_err(|e| AppError::CommandError(format!("Не вдалося запустити PowerShell: {}", e)))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        return Err(AppError::CommandError(stderr));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}