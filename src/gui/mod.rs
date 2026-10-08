pub mod preview;
pub mod sidebar;
pub mod top_bar;

use iced::widget::{column, container, row, text};
use iced::{Element, Length, Task, Theme};
use std::path::PathBuf;
use tts_epub::{parse_epub, Book};
use crate::theme;

#[derive(Default)]
pub struct App {
    book: Option<Book>,
    selected_chapter_idx: Option<usize>,
    error_message: Option<String>,
    status_message: Option<String>,
}

#[derive(Debug, Clone)]
pub enum Message {
    OpenEpubClicked,
    EpubSelected(Option<PathBuf>),
    ChapterSelected(usize),
    ChapterToggled(usize, bool),
}

impl App {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OpenEpubClicked => {
                Task::perform(
                    async {
                        rfd::AsyncFileDialog::new()
                            .add_filter("EPUB Books", &["epub"])
                            .pick_file()
                            .await
                            .map(|handle| handle.path().to_path_buf())
                    },
                    Message::EpubSelected,
                )
            }
            Message::EpubSelected(path) => {
                if let Some(path) = path {
                    match parse_epub(&path) {
                        Ok(book) => {
                            let total_words: usize = book.chapters.iter().map(|c| c.word_count).sum();
                            self.status_message = Some(format!(
                                "{} • {} chapters • {} words",
                                book.metadata.title,
                                book.chapters.len(),
                                total_words
                            ));
                            self.selected_chapter_idx = if book.chapters.is_empty() { None } else { Some(0) };
                            self.book = Some(book);
                            self.error_message = None;
                        }
                        Err(err) => {
                            self.error_message = Some(format!("Failed to parse EPUB: {err}"));
                        }
                    }
                }
                Task::none()
            }
            Message::ChapterSelected(idx) => {
                self.selected_chapter_idx = Some(idx);
                Task::none()
            }
            Message::ChapterToggled(idx, selected) => {
                if let Some(book) = &mut self.book {
                    if let Some(chapter) = book.chapters.get_mut(idx) {
                        chapter.selected = selected;
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let top = top_bar::view(self.status_message.as_deref());

        let content: Element<Message> = if let Some(book) = &self.book {
            let left = sidebar::view(book, self.selected_chapter_idx);
            let right = preview::view(book, self.selected_chapter_idx);
            row![left, right].spacing(12).height(Length::Fill).into()
        } else {
            preview::empty_state()
        };

        let mut root = column![top].spacing(12).padding(12);

        if let Some(err) = &self.error_message {
            let error_card = container(text(err).size(12).color(theme::RED))
                .style(|_t| container::Style {
                    background: Some(iced::Background::Color(theme::BASE)),
                    border: iced::Border {
                        color: theme::RED,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    ..Default::default()
                })
                .padding([8, 12]);
            root = root.push(error_card);
        }

        root.push(content).into()
    }

    pub fn theme(&self) -> Theme {
        Theme::custom("hyprland-catppuccin".to_string(), iced::theme::Palette {
            background: theme::CRUST,
            text: theme::TEXT,
            primary: theme::ACCENT,
            success: theme::GREEN,
            danger: theme::RED,
        })
    }
}
