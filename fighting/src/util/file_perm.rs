//!
//! 文件操作

use std::{os::unix::fs::PermissionsExt, path::Path};

pub fn apply_permissions<P: AsRef<Path>>(
    path: P,
    permissions: std::fs::Permissions,
) -> std::io::Result<()> {
    let current = std::fs::metadata(&path)?.permissions();
    if current != permissions {
        std::fs::set_permissions(path, permissions)?;
    }
    Ok(())
}

pub fn secure_directory<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    apply_permissions(path.as_ref(), std::fs::Permissions::from_mode(0o700))
}

pub fn secure_file<P: AsRef<Path>>(path: P) -> std::io::Result<()> {
    apply_permissions(path, std::fs::Permissions::from_mode(0o600))
}
