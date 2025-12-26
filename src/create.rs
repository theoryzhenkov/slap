use std::fs;
use std::path::PathBuf;
use std::env;
use tempfile::TempDir;

use crate::config::Config;

pub fn create_temp_dir(config: &Config) -> std::io::Result<TempDir> {
    if let Some(ref tmpdir_name) = config.tmpdir {
        let base_tmpdir = env::temp_dir();
        let normalized_name = tmpdir_name.strip_prefix('/').unwrap_or(tmpdir_name);
        let custom_tmpdir = base_tmpdir.join(normalized_name);

        fs::create_dir_all(&custom_tmpdir)?;
        tempfile::Builder::new().tempdir_in(custom_tmpdir)
    } else {
        TempDir::new()
    }
}

pub fn create_temp_file(config: &Config) -> std::io::Result<tempfile::NamedTempFile> {
    if let Some(ref tmpdir_name) = config.tmpdir {
        let base_tmpdir = env::temp_dir();
        let normalized_name = tmpdir_name.strip_prefix('/').unwrap_or(tmpdir_name);
        let custom_tmpdir = base_tmpdir.join(normalized_name);

        fs::create_dir_all(&custom_tmpdir)?;
        tempfile::Builder::new().tempfile_in(custom_tmpdir)
    } else {
        tempfile::NamedTempFile::new()
    }
}

/// Check if a path represents a directory (either by -d flag or trailing slash)
pub fn is_dir_path(path: &str, dir_mode: bool) -> bool {
    dir_mode || path.ends_with('/')
}

/// Create parent directories if needed
fn ensure_parent_exists(path: &PathBuf) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)?;
        }
    }
    Ok(())
}

/// Create paths in temp mode
pub fn create_temp_paths(
    config: &Config,
    paths: &[String],
    dir_mode: bool,
) -> std::io::Result<Vec<PathBuf>> {
    let mut created = Vec::new();

    if paths.is_empty() {
        // No paths: just create a temp file or dir
        if dir_mode {
            let dir = create_temp_dir(config)?;
            created.push(dir.keep());
        } else {
            let file = create_temp_file(config)?;
            created.push(file.into_temp_path().keep()?);
        }
    } else {
        // Create temp base directory, then structure inside
        let base = create_temp_dir(config)?;
        let base_path = base.keep();

        for path in paths {
            let full = base_path.join(path);

            if is_dir_path(path, dir_mode) {
                fs::create_dir_all(&full)?;
            } else {
                ensure_parent_exists(&full)?;
                fs::File::create(&full)?;
            }
            created.push(full);
        }
    }

    Ok(created)
}

/// Create paths in normal (non-temp) mode
pub fn create_paths(paths: &[String], dir_mode: bool) -> std::io::Result<Vec<PathBuf>> {
    let mut created = Vec::new();

    for path in paths {
        let path_buf = PathBuf::from(path);

        if is_dir_path(path, dir_mode) {
            fs::create_dir_all(&path_buf)?;
        } else {
            ensure_parent_exists(&path_buf)?;
            fs::File::create(&path_buf)?;
        }
        created.push(path_buf);
    }

    Ok(created)
}

