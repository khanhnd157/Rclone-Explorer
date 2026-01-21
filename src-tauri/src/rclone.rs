use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::Command;
use serde_json::Value;
use std::fs;
use std::io::Write;
use tauri::Emitter;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Remote {
    pub name: String,
    pub provider: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileItem {
    pub name: String,
    pub path: String,
    pub size: i64,
    pub modified: String,
    pub is_dir: bool,
    pub mime_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub operation: String,
    pub source: String,
    pub destination: String,
    pub status: String,
    pub progress: f32,
    pub speed: String,
    pub eta: Option<String>,
    pub started: String,
    pub finished: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CopyOptions {
    pub overwrite: bool,
    pub skip_existing: bool,
}

fn execute_rclone(args: &[&str]) -> Result<String, String> {
    let output = Command::new("rclone")
        .args(args)
        .output()
        .map_err(|e| format!("Failed to execute rclone: {}. Make sure rclone is installed.", e))?;

    if !output.status.success() {
        let error = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Rclone error: {}", error));
    }

    String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 output: {}", e))
}

#[tauri::command]
pub async fn list_remotes() -> Result<Vec<Remote>, String> {
    let output = execute_rclone(&["listremotes"])?;
    
    let mut remotes = Vec::new();
    for line in output.lines() {
        let name = line.trim().trim_end_matches(':');
        if !name.is_empty() {
            let config_output = execute_rclone(&["config", "dump"])
                .unwrap_or_default();
            
            let provider = if let Ok(config) = serde_json::from_str::<Value>(&config_output) {
                config.get(name)
                    .and_then(|r| r.get("type"))
                    .and_then(|t| t.as_str())
                    .map(|t| match t {
                        "drive" => "Google Drive",
                        "onedrive" => "OneDrive",
                        "dropbox" => "Dropbox",
                        "s3" => "Amazon S3",
                        _ => t,
                    })
                    .unwrap_or("Unknown")
                    .to_string()
            } else {
                "Unknown".to_string()
            };

            remotes.push(Remote {
                name: name.to_string(),
                provider,
                status: "Connected".to_string(),
            });
        }
    }
    
    Ok(remotes)
}

#[derive(Debug, Deserialize)]
struct RcloneFileItem {
    #[serde(rename = "Path")]
    path: String,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "MimeType")]
    mime_type: Option<String>,
    #[serde(rename = "ModTime")]
    mod_time: String,
    #[serde(rename = "IsDir")]
    is_dir: bool,
}

#[tauri::command]
pub async fn list_dir(remote: String, path: String) -> Result<Vec<FileItem>, String> {
    if remote == "This PC" && path == "/" {
        return list_drives().await;
    }

    if remote == "This PC" {
        return list_local_dir(path).await;
    }

    let remote_path = format!("{}:{}", remote, path);
    let output = execute_rclone(&["lsjson", &remote_path])?;
    
    let rclone_items: Vec<RcloneFileItem> = serde_json::from_str(&output)
        .map_err(|e| format!("Failed to parse rclone output: {}", e))?;
    
    let items = rclone_items.into_iter().map(|item| {
        let full_path = if path.ends_with('/') {
            format!("{}{}", path, item.name)
        } else {
            format!("{}/{}", path, item.name)
        };
        
        FileItem {
            name: item.name,
            path: full_path,
            size: item.size,
            modified: item.mod_time,
            is_dir: item.is_dir,
            mime_type: item.mime_type,
        }
    }).collect();
    
    Ok(items)
}

#[tauri::command]
pub async fn list_drives() -> Result<Vec<FileItem>, String> {
    let mut drives = Vec::new();

    #[cfg(target_os = "windows")]
    {
        use std::ffi::OsString;
        use std::os::windows::ffi::{OsStringExt, OsStrExt};
        
        unsafe {
            let buffer_size = winapi::um::fileapi::GetLogicalDriveStringsW(0, std::ptr::null_mut());
            if buffer_size == 0 {
                return Err("Failed to get drive list".to_string());
            }

            let mut buffer: Vec<u16> = vec![0; buffer_size as usize];
            let result = winapi::um::fileapi::GetLogicalDriveStringsW(buffer_size, buffer.as_mut_ptr());
            
            if result == 0 {
                return Err("Failed to get drive strings".to_string());
            }

            let mut start = 0;
            for i in 0..buffer.len() {
                if buffer[i] == 0 && i > start {
                    let drive_str = OsString::from_wide(&buffer[start..i])
                        .to_string_lossy()
                        .to_string();
                    
                    if !drive_str.is_empty() {
                        let drive_letter = drive_str.chars().next().unwrap();
                        let drive_wide: Vec<u16> = OsString::from(&drive_str)
                            .encode_wide()
                            .chain(Some(0))
                            .collect();
                        
                        let drive_type = winapi::um::fileapi::GetDriveTypeW(drive_wide.as_ptr());

                        let mut volume_name_buffer: [u16; 261] = [0; 261];
                        let volume_result = winapi::um::fileapi::GetVolumeInformationW(
                            drive_wide.as_ptr(),
                            volume_name_buffer.as_mut_ptr(),
                            volume_name_buffer.len() as u32,
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            std::ptr::null_mut(),
                            0,
                        );

                        let volume_label = if volume_result != 0 {
                            let end = volume_name_buffer.iter().position(|&c| c == 0).unwrap_or(0);
                            if end > 0 {
                                OsString::from_wide(&volume_name_buffer[..end])
                                    .to_string_lossy()
                                    .to_string()
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };

                        let drive_name = if !volume_label.is_empty() {
                            format!("{} ({}:)", volume_label, drive_letter)
                        } else {
                            match drive_type {
                                winapi::um::winbase::DRIVE_FIXED => format!("Local Disk ({}:)", drive_letter),
                                winapi::um::winbase::DRIVE_REMOVABLE => format!("Removable Disk ({}:)", drive_letter),
                                winapi::um::winbase::DRIVE_REMOTE => format!("Network Drive ({}:)", drive_letter),
                                winapi::um::winbase::DRIVE_CDROM => format!("CD Drive ({}:)", drive_letter),
                                _ => format!("Drive ({}:)", drive_letter),
                            }
                        };

                        let mut total_bytes: winapi::um::winnt::ULARGE_INTEGER = std::mem::zeroed();
                        let space_result = winapi::um::fileapi::GetDiskFreeSpaceExW(
                            drive_wide.as_ptr(),
                            std::ptr::null_mut(),
                            &mut total_bytes,
                            std::ptr::null_mut(),
                        );

                        let drive_size = if space_result != 0 {
                            *total_bytes.QuadPart() as i64
                        } else {
                            0
                        };

                        drives.push(FileItem {
                            name: drive_name,
                            path: drive_str.clone(),
                            size: drive_size,
                            modified: "".to_string(),
                            is_dir: true,
                            mime_type: None,
                        });
                    }
                    start = i + 1;
                }
            }
        }
        
        drives.sort_by(|a, b| a.path.cmp(&b.path));
    }

    #[cfg(not(target_os = "windows"))]
    {
        drives.push(FileItem {
            name: "Root (/)".to_string(),
            path: "/".to_string(),
            size: 0,
            modified: "".to_string(),
            is_dir: true,
            mime_type: None,
        });
    }

    Ok(drives)
}

#[tauri::command]
pub async fn list_local_dir(path: String) -> Result<Vec<FileItem>, String> {
    use std::fs;

    let entries = fs::read_dir(&path)
        .map_err(|e| format!("Failed to read directory: {}", e))?;

    let mut items = Vec::new();

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read entry: {}", e))?;
        let metadata = entry.metadata()
            .map_err(|e| format!("Failed to read metadata: {}", e))?;
        
        let name = entry.file_name().to_string_lossy().to_string();
        let path = entry.path().to_string_lossy().to_string();
        
        let modified = metadata.modified()
            .ok()
            .and_then(|time| time.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| {
                let secs = duration.as_secs();
                let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(secs as i64, 0);
                datetime.map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                    .unwrap_or_else(|| "".to_string())
            })
            .unwrap_or_else(|| "".to_string());

        items.push(FileItem {
            name,
            path,
            size: metadata.len() as i64,
            modified,
            is_dir: metadata.is_dir(),
            mime_type: None,
        });
    }

    items.sort_by(|a, b| {
        match (a.is_dir, b.is_dir) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
        }
    });

    Ok(items)
}

#[tauri::command]
pub async fn copy_items(
    from_remote: String,
    from_paths: Vec<String>,
    to_remote: String,
    to_path: String,
    options: CopyOptions,
) -> Result<String, String> {
    for from_path in from_paths {
        let source = format!("{}:{}", from_remote, from_path);
        let dest = format!("{}:{}", to_remote, to_path);
        
        let mut args = vec!["copy", &source, &dest];
        if options.skip_existing {
            args.push("--ignore-existing");
        }
        if !options.overwrite {
            args.push("--no-update-modtime");
        }
        
        execute_rclone(&args)?;
    }
    
    Ok(format!("job_{}", chrono::Utc::now().timestamp()))
}

#[tauri::command]
pub async fn move_items(
    from_remote: String,
    from_paths: Vec<String>,
    to_remote: String,
    to_path: String,
) -> Result<String, String> {
    for from_path in from_paths {
        let source = format!("{}:{}", from_remote, from_path);
        let dest = format!("{}:{}", to_remote, to_path);
        
        execute_rclone(&["move", &source, &dest])?;
    }
    
    Ok(format!("job_{}", chrono::Utc::now().timestamp()))
}

#[tauri::command]
pub async fn delete_items(remote: String, paths: Vec<String>) -> Result<(), String> {
    for path in paths {
        let remote_path = format!("{}:{}", remote, path);
        execute_rclone(&["deletefile", &remote_path])?;
    }
    
    Ok(())
}

#[tauri::command]
pub async fn get_jobs() -> Result<Vec<Job>, String> {
    Ok(vec![])
}

#[tauri::command]
pub async fn get_job(_job_id: String) -> Result<Option<Job>, String> {
    Ok(None)
}

#[tauri::command]
pub async fn config_list() -> Result<Vec<Remote>, String> {
    list_remotes().await
}

#[tauri::command]
pub async fn config_create(
    _name: String,
    _provider: String,
    _params: HashMap<String, String>,
) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn config_reconnect(_name: String) -> Result<(), String> {
    Ok(())
}

#[tauri::command]
pub async fn config_delete(_name: String) -> Result<(), String> {
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RcloneInfo {
    pub version: Option<String>,
    pub path: String,
    pub installed: bool,
}

#[tauri::command]
pub async fn check_rclone_version() -> Result<RcloneInfo, String> {
    let app_dir = dirs::data_local_dir()
        .ok_or_else(|| "Failed to get app data dir".to_string())?;
    
    let rclone_dir = app_dir.join("RcloneExplorer").join("rclone");
    let rclone_exe = if cfg!(windows) {
        rclone_dir.join("rclone.exe")
    } else {
        rclone_dir.join("rclone")
    };
    
    if rclone_exe.exists() {
        let output = Command::new(&rclone_exe)
            .arg("version")
            .output();
        
        match output {
            Ok(out) if out.status.success() => {
                let version_str = String::from_utf8_lossy(&out.stdout);
                let version = version_str.lines()
                    .next()
                    .and_then(|line| line.split_whitespace().nth(1))
                    .map(|v| v.trim_start_matches('v').to_string());
                
                Ok(RcloneInfo {
                    version,
                    path: rclone_exe.to_string_lossy().to_string(),
                    installed: true,
                })
            }
            _ => Ok(RcloneInfo {
                version: None,
                path: rclone_exe.to_string_lossy().to_string(),
                installed: false,
            })
        }
    } else {
        Ok(RcloneInfo {
            version: None,
            path: rclone_exe.to_string_lossy().to_string(),
            installed: false,
        })
    }
}

#[derive(Clone, Serialize)]
struct DownloadProgress {
    downloaded: u64,
    total: u64,
    percentage: f32,
}

#[tauri::command]
pub async fn install_rclone(app_handle: tauri::AppHandle) -> Result<String, String> {
    use tokio::io::AsyncWriteExt;
    use tokio::fs as async_fs;
    
    let app_dir = dirs::data_local_dir()
        .ok_or_else(|| "Failed to get app data dir".to_string())?;
    
    let rclone_dir = app_dir.join("RcloneExplorer").join("rclone");
    async_fs::create_dir_all(&rclone_dir).await
        .map_err(|e| format!("Failed to create rclone directory: {}", e))?;
    
    let (download_url, archive_name) = if cfg!(windows) {
        if cfg!(target_arch = "x86_64") {
            ("https://downloads.rclone.org/rclone-current-windows-amd64.zip", "rclone-current-windows-amd64.zip")
        } else {
            ("https://downloads.rclone.org/rclone-current-windows-386.zip", "rclone-current-windows-386.zip")
        }
    } else if cfg!(target_os = "macos") {
        ("https://downloads.rclone.org/rclone-current-osx-amd64.zip", "rclone-current-osx-amd64.zip")
    } else {
        ("https://downloads.rclone.org/rclone-current-linux-amd64.zip", "rclone-current-linux-amd64.zip")
    };
    
    let archive_path = rclone_dir.join(archive_name);
    
    let response = reqwest::get(download_url).await
        .map_err(|e| format!("Failed to download rclone: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("Failed to download rclone: HTTP {}", response.status()));
    }
    
    let total_size = response.content_length().unwrap_or(0);
    let mut downloaded: u64 = 0;
    let mut file = async_fs::File::create(&archive_path).await
        .map_err(|e| format!("Failed to create archive file: {}", e))?;
    
    let mut stream = response.bytes_stream();
    use futures_util::StreamExt;
    
    while let Some(chunk) = stream.next().await {
        let chunk = chunk.map_err(|e| format!("Failed to read download: {}", e))?;
        
        file.write_all(&chunk).await
            .map_err(|e| format!("Failed to write archive: {}", e))?;
        
        downloaded += chunk.len() as u64;
        
        let percentage = if total_size > 0 {
            (downloaded as f32 / total_size as f32) * 100.0
        } else {
            0.0
        };
        
        let _ = app_handle.emit("rclone-download-progress", DownloadProgress {
            downloaded,
            total: total_size,
            percentage,
        });
    }
    
    file.flush().await.map_err(|e| format!("Failed to flush file: {}", e))?;
    
    let file = fs::File::open(&archive_path)
        .map_err(|e| format!("Failed to open archive: {}", e))?;
    
    let mut archive = zip::ZipArchive::new(file)
        .map_err(|e| format!("Failed to read zip: {}", e))?;
    
    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .map_err(|e| format!("Failed to read zip entry: {}", e))?;
        
        let file_name = file.name();
        if file_name.ends_with("rclone.exe") || (file_name.ends_with("rclone") && !file_name.contains('.')) {
            let target_name = if cfg!(windows) { "rclone.exe" } else { "rclone" };
            let out_path = rclone_dir.join(target_name);
            
            let mut outfile = fs::File::create(&out_path)
                .map_err(|e| format!("Failed to create rclone executable: {}", e))?;
            std::io::copy(&mut file, &mut outfile)
                .map_err(|e| format!("Failed to extract rclone: {}", e))?;
            
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&out_path)
                    .map_err(|e| format!("Failed to get permissions: {}", e))?
                    .permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&out_path, perms)
                    .map_err(|e| format!("Failed to set permissions: {}", e))?;
            }
            
            break;
        }
    }
    
    fs::remove_file(&archive_path).ok();
    
    Ok("Rclone installed successfully".to_string())
}

#[tauri::command]
pub async fn update_rclone(app_handle: tauri::AppHandle) -> Result<String, String> {
    install_rclone(app_handle).await
}
