use iced::{
    Element,
    Length::Fill,
    widget::{button, column, horizontal_rule, row, text},
};

use crate::models::{AppState, Message};
use crate::ui::styles::dir_button_style;

pub fn view(state: &AppState) -> Element<'_, Message> {
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
