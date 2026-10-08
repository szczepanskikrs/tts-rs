mod gui;
mod theme;

use gui::App;

fn main() -> iced::Result {
    iced::application("TTS-RS", App::update, App::view)
        .theme(App::theme)
        .window(iced::window::Settings {
            size: iced::Size::new(1920.0, 1080.0),
            min_size: Some(iced::Size::new(900.0, 600.0)),
            position: iced::window::Position::Centered,
            ..Default::default()
        })
        .run()
}
