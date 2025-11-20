use std::{
    fs::{self},
    path::PathBuf,
    process::Command,
};

use iced::{
    Border, Element,
    Length::Fill,
    Shadow, Task,
    widget::{button, column, horizontal_rule, row, text},
    window,
};

#[derive(Debug)]
struct AppState {
    current_dir: PathBuf,
    current_files: Vec<(String, bool)>,
    popup: Option<String>,
}

impl Default for AppState {
    fn default() -> Self {
        let current_dir = std::env::current_dir().unwrap();
        let current_files = get_files(&current_dir);

        AppState {
            current_dir,
            current_files,
            popup: None,
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Exit,
    CD(PathBuf),
    JRIP(PathBuf),
    ClosePopup,
}

fn update(state: &mut AppState, message: Message) -> Task<Message> {
    match message {
        Message::Exit => window::get_latest().and_then(window::close),
        Message::CD(path_buf) => {
            state.current_dir = path_buf;
            state.current_files = get_files(&state.current_dir);
            Task::none()
        }
        Message::JRIP(path_buf) => {
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
                        state.popup =
                            Some(String::from("Audio Has Been Ripped"))
                    } else {
                        state.popup = Some(String::from("Error Ripping"))
                    }
                }
            }
            Task::none()
        }
        Message::ClosePopup => {
            state.popup = None;

            Task::none()
        }
    }
}

fn view(state: &AppState) -> Element<'_, Message> {
    let mut content = column![
        row![
            text(state.current_dir.to_str().unwrap_or("unkown directory"))
                .size(32)
                .width(Fill),
            button(text("Up").size(24)).on_press(Message::CD(
                state
                    .current_dir
                    .parent()
                    .unwrap_or(&state.current_dir)
                    .to_path_buf()
            )),
            button(text("Exit").size(24)).on_press(Message::Exit)
        ]
        .spacing(8)
    ]
    .spacing(2)
    .padding(4);

    content = content.push(horizontal_rule(2));

    if let Some(pat) = &state.popup {
        let popup_content = row![
            text(pat).width(Fill),
            button("Close").on_press(Message::ClosePopup)
        ];
        content = content.push(popup_content);

        content = content.push(horizontal_rule(2));
    }

    for file in &state.current_files {
        let file_name = text(&file.0).size(18);
        let mut file_path = state.current_dir.clone();
        file_path.push(&file.0);

        if file.1 {
            content = content.push(
                button(file_name)
                    .style(dir_button_style())
                    .on_press(Message::CD(file_path)),
            );
        } else {
            let file = column![row![
                file_name.width(Fill),
                button(text("Jrip")).on_press(Message::JRIP(file_path)),
            ]];
            content = content.push(file);
        }
    }

    content.into()
}

fn dir_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style
{
    |_t, _e| button::Style {
        background: None,
        text_color: iced::Color::from_rgb(
            3.0 / 255.0,
            161.0 / 255.0,
            252.0 / 255.0,
        ),
        border: Border::default(),
        shadow: Shadow::default(),
    }
}

fn get_files(path: &PathBuf) -> Vec<(String, bool)> {
    let mut dirs = Vec::default();
    let mut files = Vec::default();

    if let Ok(read_dir) = fs::read_dir(path) {
        for read in read_dir {
            if let Ok(dir_entry) = read {
                if let Some(name) = dir_entry.file_name().to_str() {
                    if dir_entry.path().is_dir() {
                        dirs.push((name.to_string(), true));
                    } else {
                        if name.ends_with("mov") {
                            dirs.push((name.to_string(), false));
                        }
                    }
                }
            }
        }
    }

    dirs.append(&mut files);
    dirs
}

fn main() -> iced::Result {
    iced::application("Jrip", update, view)
        .theme(|_s| iced::Theme::KanagawaDragon)
        .run()
}
