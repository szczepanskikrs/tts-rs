use iced::widget::{button, container, row, text};
use iced::{Alignment, Element, Length};
use crate::gui::Message;
use crate::theme;

pub fn view(status_message: Option<&str>) -> Element<'static, Message> {
    let brand_badge = container(
        text("󰓃 TTS-RS")
            .size(13)
            .color(theme::CRUST),
    )
    .style(|_t| container::Style {
        background: Some(iced::Background::Color(theme::ACCENT)),
        border: iced::Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    })
    .padding([4, 10]);

    let open_btn = button(text("Open EPUB").size(13))
        .style(theme::primary_btn)
        .padding([6, 14])
        .on_press(Message::OpenEpubClicked);

    let status = status_message
        .unwrap_or("Ready • Select an EPUB file to inspect and convert")
        .to_string();

    let status_text = text(status)
        .size(13)
        .color(theme::SUBTEXT);

    container(
        row![brand_badge, open_btn, status_text]
            .spacing(12)
            .align_y(Alignment::Center),
    )
    .style(theme::tiled_card)
    .padding(10)
    .width(Length::Fill)
    .into()
}
