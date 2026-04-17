use crate::errors::CompilerError;
use crate::toolchain;
use std::process::Command;

pub(crate) fn link_executable(obj_file: &str, exe_file: &str) -> Result<(), CompilerError> {
    let zig_path = toolchain::ensure_zig()?;

    let output = Command::new(&zig_path)
        .args(["cc", "-o", exe_file, obj_file])
        .output()
        .map_err(|e| CompilerError::LinkError(format!("Failed to run zig cc: {e}")))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(CompilerError::LinkError(format!("zig cc failed: {stderr}")))
    }
}
