use std::{path::PathBuf, process::Command};

/// Extracts audio from a video file using ffmpeg.
/// Returns a message indicating success or failure.
pub fn rip_audio(path_buf: &PathBuf) -> Option<String> {
    if let Some(parent) = path_buf.parent() {
        let mut new_file = parent.to_path_buf();
        new_file.push("output.mp3");

        if let Ok(output) = Command::new("ffmpeg")
            .args([
                "-i",
                path_buf.to_str().unwrap_or("/home"),
                "-y",
                new_file.to_str().unwrap_or("/home"),
            ])
            .status()
        {
            if output.success() {
                Some(String::from("Audio Has Been Ripped"))
            } else {
                Some(String::from("Error Ripping"))
            }
        } else {
            Some(String::from("Failed to execute ffmpeg"))
        }
    } else {
        Some(String::from("Invalid file path"))
    }
}
