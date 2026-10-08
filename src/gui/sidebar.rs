use iced::widget::{button, checkbox, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};
use tts_epub::Book;
use crate::gui::Message;
use crate::theme;

pub fn view<'a>(
    book: &'a Book,
    selected_idx: Option<usize>,
) -> Element<'a, Message> {
    let mut chapter_list = column![].spacing(6);

    for (idx, chapter) in book.chapters.iter().enumerate() {
        let is_selected = selected_idx == Some(idx);
        let is_active = chapter.selected;

        let chapter_btn = button(
            row![
                text(format!("{:02}. {}", idx + 1, chapter.title))
                    .size(13)
                    .width(Length::Fill),
                text(format!("{} w", chapter.word_count))
                    .size(11)
                    .color(if is_selected { theme::ACCENT } else { theme::OVERLAY }),
            ]
            .align_y(Alignment::Center),
        )
        .style(if is_selected {
            theme::active_chapter_btn
        } else {
            theme::secondary_btn
        })
        .padding([7, 10])
        .on_press(Message::ChapterSelected(idx))
        .width(Length::Fill);

        let row_item = row![
            checkbox("", is_active).on_toggle(move |val| Message::ChapterToggled(idx, val)),
            chapter_btn,
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        chapter_list = chapter_list.push(row_item);
    }

    container(
        column![
            row![
                text("TABLE OF CONTENTS").size(11).color(theme::OVERLAY),
                text(format!("({} chapters)", book.chapters.len()))
                    .size(11)
                    .color(theme::OVERLAY),
            ]
            .spacing(6),
            scrollable(chapter_list).height(Length::Fill),
        ]
        .spacing(10),
    )
    .style(theme::tiled_card)
    .padding(14)
    .width(Length::FillPortion(1))
    .height(Length::Fill)
    .into()
}
