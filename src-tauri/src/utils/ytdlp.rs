use crate::utils::shell::run_command;

pub fn get_ytdlp_version() -> Result<String, String> {
    let ytdlp = get_ytdlp().unwrap();
    run_command(ytdlp, vec!["--version"])
}

pub fn get_ytdlp() -> Result<String, String> {
    let yt_dlp_path = get_ytdlp_binary().unwrap();
    // println!("Using yt-dlp path: {}", yt_dlp_path);

    // Check if the binary exists
    let path = std::path::Path::new(&yt_dlp_path);
    if !path.exists() {
        return Err(format!("yt-dlp binary not found at: {}", yt_dlp_path));
    }

    // Check if it's executable (Unix systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata =
            std::fs::metadata(&path).map_err(|e| format!("Failed to get file metadata: {}", e))?;
        let permissions = metadata.permissions();
        if permissions.mode() & 0o111 == 0 {
            return Err(format!("yt-dlp binary is not executable: {}", yt_dlp_path));
        }
    }

    Ok(yt_dlp_path)
}

fn get_ytdlp_binary() -> Result<String, String> {
    #[cfg(target_os = "windows")]
    let binary_name = "yt-dlp.exe";
    #[cfg(not(target_os = "windows"))]
    let binary_name = "yt-dlp";

    let resource_dir = std::env::current_exe()
        .ok()
        .and_then(|path| path.parent().map(|p| p.to_path_buf()))
        .ok_or("Could not determine executable directory")?;

    let bundled_path = resource_dir.join(binary_name);

    if bundled_path.exists() {
        Ok(bundled_path.to_string_lossy().to_string())
    } else {
        // Fall back to system PATH
        Ok(binary_name.to_string())
    }
}
