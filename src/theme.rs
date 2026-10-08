#![allow(dead_code)]

use iced::widget::{button, container};
use iced::{Background, Border, Color, Shadow};

pub const CRUST: Color = Color::from_rgb(0.067, 0.067, 0.106);
pub const MANTLE: Color = Color::from_rgb(0.094, 0.094, 0.145);
pub const BASE: Color = Color::from_rgb(0.118, 0.118, 0.180);
pub const SURFACE0: Color = Color::from_rgb(0.192, 0.196, 0.267);
pub const SURFACE1: Color = Color::from_rgb(0.271, 0.278, 0.353);
pub const TEXT: Color = Color::from_rgb(0.804, 0.839, 0.957);
pub const SUBTEXT: Color = Color::from_rgb(0.651, 0.678, 0.784);
pub const OVERLAY: Color = Color::from_rgb(0.424, 0.439, 0.525);
pub const ACCENT: Color = Color::from_rgb(0.796, 0.651, 0.969);
pub const ACCENT_HOVER: Color = Color::from_rgb(0.85, 0.73, 0.98);
pub const ACCENT_PRESSED: Color = Color::from_rgb(0.72, 0.58, 0.90);
pub const GREEN: Color = Color::from_rgb(0.651, 0.890, 0.631);
pub const RED: Color = Color::from_rgb(0.953, 0.545, 0.659);

pub const BLUE: Color = Color::from_rgb(0.537, 0.706, 0.980);

fn border(color: Color, width: f32, radius: f32) -> Border {
    Border {
        color,
        width,
        radius: radius.into(),
    }
}

pub fn container_card(bg: Color, border_color: Color, text_color: Color, radius: f32) -> container::Style {
    container::Style {
        background: Some(Background::Color(bg)),
        border: border(border_color, 1.0, radius),
        text_color: Some(text_color),
        shadow: Shadow::default(),
    }
}

pub fn tiled_card(_theme: &iced::Theme) -> container::Style {
    container_card(MANTLE, SURFACE0, TEXT, 8.0)
}

pub fn active_card(_theme: &iced::Theme) -> container::Style {
    container_card(BASE, ACCENT, TEXT, 8.0)
}

pub fn pill_badge(_theme: &iced::Theme) -> container::Style {
    container_card(SURFACE0, Color::TRANSPARENT, SUBTEXT, 12.0)
}

fn button_style(bg: Color, text: Color, border_color: Color, border_width: f32) -> button::Style {
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: text,
        border: border(border_color, border_width, 6.0),
        shadow: Shadow::default(),
    }
}

pub fn primary_btn(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button_style(ACCENT_HOVER, CRUST, Color::TRANSPARENT, 0.0),
        button::Status::Pressed => button_style(ACCENT_PRESSED, CRUST, Color::TRANSPARENT, 0.0),
        _ => button_style(ACCENT, CRUST, Color::TRANSPARENT, 0.0),
    }
}

pub fn secondary_btn(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button_style(SURFACE1, TEXT, ACCENT, 1.0),
        button::Status::Pressed => button_style(Color::from_rgb(0.23, 0.23, 0.33), TEXT, SURFACE1, 1.0),
        _ => button_style(SURFACE0, TEXT, SURFACE1, 1.0),
    }
}

pub fn active_chapter_btn(_theme: &iced::Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Hovered => button_style(SURFACE0, ACCENT, ACCENT, 1.0),
        _ => button_style(BASE, ACCENT, ACCENT, 1.0),
    }
}
