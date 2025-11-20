use iced::{Border, Shadow, widget::button};

pub fn dir_button_style() -> impl Fn(&iced::Theme, button::Status) -> button::Style
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
