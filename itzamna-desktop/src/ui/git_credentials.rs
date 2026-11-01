//! Git Credentials Altar (Local Repository Scope)
//! Allows viewing and mutating user.name, user.email, and remote origin URL directly.

use iced::widget::{button, column, container, row, text, text_input, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum CredentialsMessage {
    Open,
    Close,
    NameChanged(String),
    EmailChanged(String),
    RemoteUrlChanged(String),
    Save,
}

#[derive(Debug, Clone)]
pub struct CredentialsModal {
    pub is_open: bool,
    pub name: String,
    pub email: String,
    pub remote_url: String,
    pub feedback_message: Option<(String, bool)>, // (message, is_success)
}

impl Default for CredentialsModal {
    fn default() -> Self {
        Self::new()
    }
}

impl CredentialsModal {
    pub fn new() -> Self {
        Self {
            is_open: false,
            name: String::new(),
            email: String::new(),
            remote_url: String::new(),
            feedback_message: None,
        }
    }

    pub fn set_data(&mut self, name: String, email: String, remote_url: String) {
        self.name = name;
        self.email = email;
        self.remote_url = remote_url;
        self.feedback_message = None;
    }

    pub fn view<'a, Message: 'a + From<CredentialsMessage> + Clone>(&'a self) -> Option<Element<'a, Message>> {
        if !self.is_open {
            return None;
        }

        let title_row = row![
            text("⚙️").size(16),
            Space::new().width(8),
            text("SACRED GIT CREDENTIALS (LOCAL REPOSITORY)")
                .size(13)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(Length::Fill),
            button(text("✕").size(12).color(MayaColors::GLYPH_DIM))
                .padding(Padding::from([2, 6]))
                .style(|_, status| {
                    let text_color = match status {
                        button::Status::Hovered => MayaColors::CINNABAR_BRIGHT,
                        _ => MayaColors::GLYPH_DIM,
                    };
                    button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        text_color,
                        ..Default::default()
                    }
                })
                .on_press(CredentialsMessage::Close.into()),
        ]
        .align_y(Alignment::Center);

        let subtitle = text("Configure local committer identity and remote conduit for this sanctuary.")
            .size(10)
            .color(MayaColors::PARCHMENT_MUTED);

        let name_label = text("👤 Committer Name (user.name)").size(10).color(MayaColors::STARLIGHT_WHITE);
        let name_input = text_input("e.g. Pacal the Great", &self.name)
            .size(11)
            .padding(Padding::from([6, 10]))
            .on_input(|s| CredentialsMessage::NameChanged(s).into())
            .style(|_, _| text_input::Style {
                background: MayaColors::OBSIDIAN_VOID.into(),
                border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 4.0.into() },
                icon: MayaColors::GLYPH_DIM,
                placeholder: MayaColors::GLYPH_DIM,
                value: MayaColors::STARLIGHT_WHITE,
                selection: MayaColors::JADE_SHADOW,
            });

        let email_label = text("📧 Committer Email (user.email)").size(10).color(MayaColors::STARLIGHT_WHITE);
        let email_input = text_input("e.g. pacal@palenque.maya", &self.email)
            .size(11)
            .padding(Padding::from([6, 10]))
            .on_input(|s| CredentialsMessage::EmailChanged(s).into())
            .style(|_, _| text_input::Style {
                background: MayaColors::OBSIDIAN_VOID.into(),
                border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 4.0.into() },
                icon: MayaColors::GLYPH_DIM,
                placeholder: MayaColors::GLYPH_DIM,
                value: MayaColors::STARLIGHT_WHITE,
                selection: MayaColors::JADE_SHADOW,
            });

        let remote_label = text("🌐 Remote Origin URL (remote.origin.url)").size(10).color(MayaColors::STARLIGHT_WHITE);
        let remote_input = text_input("e.g. https://github.com/org/repo.git", &self.remote_url)
            .size(11)
            .padding(Padding::from([6, 10]))
            .on_input(|s| CredentialsMessage::RemoteUrlChanged(s).into())
            .style(|_, _| text_input::Style {
                background: MayaColors::OBSIDIAN_VOID.into(),
                border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 4.0.into() },
                icon: MayaColors::GLYPH_DIM,
                placeholder: MayaColors::GLYPH_DIM,
                value: MayaColors::STARLIGHT_WHITE,
                selection: MayaColors::JADE_SHADOW,
            });

        let feedback: Element<'a, Message> = if let Some((msg, ok)) = &self.feedback_message {
            let color = if *ok { MayaColors::JADE_GLOW } else { MayaColors::CINNABAR_BRIGHT };
            text(msg).size(10).color(color).into()
        } else {
            Space::new().height(0).into()
        };

        let actions_row = row![
            button(text("Cancel").size(11).color(MayaColors::PARCHMENT_MUTED))
                .padding(Padding::from([6, 14]))
                .style(|_, _| button::Style {
                    background: Some(MayaColors::OBSIDIAN_BASE.into()),
                    border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 4.0.into() },
                    ..Default::default()
                })
                .on_press(CredentialsMessage::Close.into()),
            Space::new().width(Length::Fill),
            button(
                row![
                    text("💾").size(11),
                    Space::new().width(4),
                    text("Inscribe Credentials").size(11).color(MayaColors::OBSIDIAN_VOID),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([6, 16]))
            .style(|_, status| {
                let bg = match status {
                    button::Status::Hovered => MayaColors::SOLAR_AMBER,
                    _ => MayaColors::SOLAR_GOLD,
                };
                button::Style {
                    background: Some(bg.into()),
                    border: Border { color: MayaColors::SOLAR_AMBER, width: 1.0, radius: 4.0.into() },
                    ..Default::default()
                }
            })
            .on_press(CredentialsMessage::Save.into()),
        ]
        .align_y(Alignment::Center);

        let card = container(
            column![
                title_row,
                Space::new().height(4),
                subtitle,
                Space::new().height(14),
                name_label,
                Space::new().height(2),
                name_input,
                Space::new().height(10),
                email_label,
                Space::new().height(2),
                email_input,
                Space::new().height(10),
                remote_label,
                Space::new().height(2),
                remote_input,
                Space::new().height(8),
                feedback,
                Space::new().height(14),
                actions_row,
            ]
            .padding(Padding::from([20, 24]))
            .width(Length::Fixed(480.0))
        )
        .style(|_| container::Style {
            background: Some(MayaColors::OBSIDIAN_BASE.into()), // 100% solid, fully opaque!
            border: Border {
                color: MayaColors::SOLAR_GOLD,
                width: 1.5,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        let backdrop = container(card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(|_| container::Style {
                background: Some(Color { a: 0.70, ..MayaColors::OBSIDIAN_VOID }.into()),
                ..Default::default()
            });

        Some(backdrop.into())
    }
}
