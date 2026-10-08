use iced::widget::{column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};
use tts_epub::Book;
use crate::gui::Message;
use crate::theme;

pub fn view<'a>(
    book: &'a Book,
    selected_idx: Option<usize>,
) -> Element<'a, Message> {
    let preview_title = if let Some(idx) = selected_idx {
        book.chapters.get(idx).map(|c| c.title.as_str()).unwrap_or("Preview")
    } else {
        "Preview"
    };

    let preview_content = if let Some(idx) = selected_idx {
        book.chapters
            .get(idx)
            .map(|c| c.content.as_str())
            .unwrap_or("No content")
    } else {
        "Select a chapter from the list to view its cleaned content."
    };

    container(
        column![
            row![
                text("PREVIEW:").size(11).color(theme::OVERLAY),
                text(preview_title).size(12).color(theme::ACCENT),
            ]
            .spacing(8),
            scrollable(
                text(preview_content)
                    .size(13)
                    .color(theme::TEXT)
                    .line_height(1.5)
            )
            .height(Length::Fill),
        ]
        .spacing(12),
    )
    .style(theme::tiled_card)
    .padding(16)
    .width(Length::FillPortion(2))
    .height(Length::Fill)
    .into()
}

pub fn empty_state<'a>() -> Element<'a, Message> {
    container(
        column![
            text("No Book Loaded").size(20).color(theme::TEXT),
            text("Open an EPUB file to inspect chapters, configure voice tone, and synthesize.")
                .size(13)
                .color(theme::SUBTEXT),
        ]
        .spacing(8)
        .align_x(Alignment::Center),
    )
    .style(theme::tiled_card)
    .center_x(Length::Fill)
    .center_y(Length::Fill)
    .into()
}
