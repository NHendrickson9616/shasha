//! Verify generated Rust with the compiler before returning it to callers.

use std::{
    io::Write,
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    time::{SystemTime, UNIX_EPOCH},
};

static NEXT_OUTPUT: AtomicU64 = AtomicU64::new(0);

/// Check syntax, name resolution, and types without building an executable.
/// This does not prove that the function implements the prompt.
pub fn validate_rust(source: &str) -> Result<(), String> {
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| format!("failed to get validation timestamp: {error}"))?
        .as_nanos();
    let output = std::env::temp_dir().join(format!(
        "shasha-check-{}-{stamp}-{}.rmeta",
        std::process::id(),
        NEXT_OUTPUT.fetch_add(1, Ordering::Relaxed)
    ));
    let mut child = Command::new("rustc")
        .args([
            "--edition=2024",
            "--crate-type=lib",
            "--emit=metadata",
            "--crate-name=generated",
            "-o",
        ])
        .arg(&output)
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("could not run rustc to validate generated code: {error}"))?;
    let write_result = child
        .stdin
        .take()
        .expect("piped stdin")
        .write_all(source.as_bytes());
    if let Err(error) = write_result {
        let _ = child.kill();
        let _ = child.wait();
        let _ = std::fs::remove_file(&output);
        return Err(format!("could not send generated code to rustc: {error}"));
    }
    let result = child
        .wait_with_output()
        .map_err(|error| format!("rustc validation failed: {error}"))?;
    let _ = std::fs::remove_file(&output);
    if !result.status.success() {
        return Err(format!(
            "generated Rust failed to compile:\n{}",
            String::from_utf8_lossy(&result.stderr)
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_valid_rust_and_rejects_invalid_rust() {
        validate_rust("fn add(left: i32, right: i32) -> i32 { left + right }").unwrap();
        let error = validate_rust("fn add(left: i32) -> i32 { missing }").unwrap_err();
        assert!(error.contains("failed to compile"));
    }
}
