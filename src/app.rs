use iced::{Task, window};

use crate::models::{AppState, Message};
use crate::utils::ffmpeg::rip_audio;
use crate::utils::file_operations::get_files;

pub fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
        Message::CD(path_buf) => {
            state.current_dir = path_buf;
            state.current_files = get_files(&state.current_dir);
            Task::none()
        }
        Message::JRIP(path_buf) => {
            state.popup = rip_audio(&path_buf);
            Task::none()
        }
        Message::ClosePopup => {
            state.popup = None;
            Task::none()
        }
    }
}
