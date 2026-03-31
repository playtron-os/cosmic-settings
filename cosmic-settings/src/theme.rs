// Copyright 2023 System76 <info@system76.com>
// SPDX-License-Identifier: GPL-3.0-only

use cosmic::{iced_core::Border, theme};

/// State default color
pub const STATE_DEFAULT: cosmic::iced::Color = cosmic::iced::Color {
    r: 0.0,
    g: 150.0 / 255.0,
    b: 136.0 / 255.0,
    a: 1.0,
};

#[must_use]
pub fn coming_soon_icon_container() -> cosmic::theme::Container<'static> {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        cosmic::widget::container::Style {
            icon_color: Some(STATE_DEFAULT),
            text_color: None,
            background: Some(cosmic::iced::Background::Color(cosmic::iced::Color {
                r: STATE_DEFAULT.r,
                g: STATE_DEFAULT.g,
                b: STATE_DEFAULT.b,
                a: 0.2,
            })),
            border: Border {
                color: cosmic::iced::Color::TRANSPARENT,
                radius: cosmic.corner_radii.radius_m.into(),
                width: 0.0,
            },
            shadow: Default::default(),
        }
    })
}

#[must_use]
pub fn connected_chip() -> cosmic::theme::Container<'static> {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        cosmic::widget::container::Style {
            icon_color: None,
            text_color: None,
            background: Some(cosmic::iced::Background::Color(cosmic::iced::Color {
                r: STATE_DEFAULT.r,
                g: STATE_DEFAULT.g,
                b: STATE_DEFAULT.b,
                a: 0.1,
            })),
            border: Border {
                color: cosmic::iced::Color::TRANSPARENT,
                radius: cosmic.corner_radii.radius_s.into(),
                width: 0.0,
            },
            shadow: Default::default(),
        }
    })
}

#[must_use]
pub fn display_container_frame() -> cosmic::theme::Container<'static> {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        cosmic::widget::container::Style {
            icon_color: None,
            text_color: None,
            background: Some(cosmic::iced::Background::Color(cosmic::iced::Color::WHITE)),
            border: Border {
                color: cosmic::iced::Color::WHITE,
                radius: cosmic.corner_radii.radius_xs.into(),
                width: 3.0,
            },
            shadow: Default::default(),
        }
    })
}

#[must_use]
pub fn display_container_screen() -> cosmic::theme::Container<'static> {
    theme::Container::custom(|theme| {
        let cosmic = theme.cosmic();
        cosmic::widget::container::Style {
            icon_color: None,
            text_color: None,
            background: Some(cosmic::iced::Background::Color(cosmic::iced::Color::BLACK)),
            border: Border {
                color: cosmic::iced::Color::BLACK,
                radius: cosmic.corner_radii.radius_0.into(),
                width: 0.0,
            },
            shadow: Default::default(),
        }
    })
}
