use std::env;
use std::process::{Command, Stdio};
use std::time::Duration;

pub fn command(command: &str, timeout: u64) -> std::io::Result<String> {
    let timeout_duration = Duration::from_secs(timeout);
    let shell = env::var("SHELL").unwrap_or_else(|_| {
        if cfg!(target_os = "windows") {
            "cmd.exe".to_string()
        } else {
            "sh".to_string()
        }
    });

    let mut timeout_args = vec![];
    let timeout_duration_str = timeout_duration.as_secs().to_string();
    if cfg!(target_os = "windows") {
        timeout_args.extend_from_slice(&["/C", "timeout", "/T", &timeout_duration_str, "/nobreak"]);
    } else {
        timeout_args.extend_from_slice(&["timeout", &timeout_duration_str]);
    }

    let child = Command::new(&shell)
        .args(&["-c", command])
        .args(timeout_args.as_slice())
        .stdout(Stdio::piped())
        .spawn()?;
    let output = child.wait_with_output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::Other, String::from_utf8_lossy(&output.stderr).to_string()))
    }
}

