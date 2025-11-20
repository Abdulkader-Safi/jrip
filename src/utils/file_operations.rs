use std::{fs, path::PathBuf};

/// Returns a list of files and directories in the given path.
/// Each tuple contains (name, is_directory).
/// Directories are listed first, followed by video files.
pub fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();
    let mut files = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for read in read_dir {
            if let Ok(dir_entry) = read {
                if let Some(name) = dir_entry.file_name().to_str() {
                    if dir_entry.path().is_dir() {
                        dirs.push((name.to_string(), true));
                    } else if is_video_file(name) {
                        dirs.push((name.to_string(), false));
                    }
                }
            }
        }
    }

    dirs.append(&mut files);
    dirs
}

/// Checks if a file is a video file based on its extension.
fn is_video_file(filename: &str) -> bool {
    let video_extensions = [
        "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm", "m4v",
        "mpg", "mpeg", "3gp", "ogv", "ts", "mts", "m2ts", "vob",
    ];

    if let Some(extension) = filename.rsplit('.').next() {
        video_extensions.contains(&extension.to_lowercase().as_str())
    } else {
        false
    }
}
