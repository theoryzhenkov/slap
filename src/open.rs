use std::env;
use std::path::PathBuf;
use std::process::Command;

/// Check if any paths are directories and print warnings
pub fn warn_about_directories(paths: &[PathBuf]) {
    for path in paths {
        if path.is_dir() {
            eprintln!(
                "warning: '{}' is a directory; opening with editor may not work as expected",
                path.to_string_lossy()
            );
        }
    }
}

/// Open paths with the specified app or default editor
pub fn open_paths(paths: &[PathBuf], app: Option<String>) -> std::io::Result<()> {
    if paths.is_empty() {
        return Ok(());
    }

    // Convert paths to strings, using lossy conversion for non-UTF8 paths
    let path_strs: Vec<String> = paths.iter().map(|p| p.to_string_lossy().into_owned()).collect();

    if let Some(app) = app {
        // Use the `open` crate for cross-platform "open with app" support
        for path in &path_strs {
            open::with(path, &app)?;
        }
    } else {
        // Open with $EDITOR (falls back to vi)
        let editor = env::var("EDITOR").unwrap_or_else(|_| "vi".to_string());
        Command::new(&editor)
            .args(&path_strs)
            .status()?;
    }

    Ok(())
}

