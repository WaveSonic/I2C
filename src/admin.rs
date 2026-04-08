use std::process::Command;

pub fn is_running_as_admin() -> bool {
    let output = Command::new("powershell")
        .args([
            "-Command",
            "([Security.Principal.WindowsPrincipal] \
             [Security.Principal.WindowsIdentity]::GetCurrent())\
             .IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)"
        ])
        .output();

    match output {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout).trim().to_string();
            text.eq_ignore_ascii_case("true")
        }
        Err(_) => false,
    }
}