//! The Bottom Prophecy Tray (El Feed de Profecías)
//! Real-time console log of repository events, git executions, and Ixchel Oracle telemetry.

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum ProphecyMessage {
    ToggleDrawer,
    ClearFeed,
}

#[derive(Debug, Clone)]
pub struct ProphecyEntry {
    pub timestamp: &'static str,
    pub glyph: &'static str,
    pub message: String,
    pub level: ProphecyLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProphecyLevel {
    Solar,   // Success / Commit
    Jade,    // Branch / Creation
    Maya,    // Network / Stream
    Xibalba, // Warning / Error
}

pub struct BottomProphecy {
    pub is_expanded: bool,
    pub entries: Vec<ProphecyEntry>,
}

impl Default for BottomProphecy {
    fn default() -> Self {
        Self::new()
    }
}

impl BottomProphecy {
    pub fn new() -> Self {
        Self {
            is_expanded: true,
            entries: vec![
                ProphecyEntry {
                    timestamp: "22:44:08",
                    glyph: "☀️",
                    message: "Solar commit sealed: 3b7f7a1 — 'feat(desktop): complete Phase 1'".into(),
                    level: ProphecyLevel::Solar,
                },
                ProphecyEntry {
                    timestamp: "22:47:44",
                    glyph: "🏛️",
                    message: "Awakening Stelae Palace (Phase 2 multi-panel layout)".into(),
                    level: ProphecyLevel::Jade,
                },
                ProphecyEntry {
                    timestamp: "22:49:05",
                    glyph: "📁",
                    message: "Background asset verified: assets/background.png online".into(),
                    level: ProphecyLevel::Jade,
                },
                ProphecyEntry {
                    timestamp: "22:50:00",
                    glyph: "🌙",
                    message: "Ixchel Oracle bridge listening on 127.0.0.1:7878".into(),
                    level: ProphecyLevel::Maya,
                },
            ],
        }
    }

    pub fn push(&mut self, timestamp: &'static str, glyph: &'static str, msg: String, level: ProphecyLevel) {
        self.entries.push(ProphecyEntry {
            timestamp,
            glyph,
            message: msg,
            level,
        });
    }

    pub fn view<'a, Message: 'a + From<ProphecyMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let toggle_icon = if self.is_expanded { "▼" } else { "▲" };

        let header = row![
            text("🔮 FEED DE PROFECÍAS")
                .size(10)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            text(format!("({} logs)", self.entries.len()))
                .size(9)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            button(text(toggle_icon).size(10).color(MayaColors::GLYPH_DIM))
                .padding(Padding::from([1, 6]))
                .style(|_theme, _status| button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(ProphecyMessage::ToggleDrawer.into()),
        ]
        .padding(Padding::from([3, 10]))
        .align_y(Alignment::Center);

        if !self.is_expanded {
            return container(header)
                .width(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(Color { a: 0.95, ..MayaColors::OBSIDIAN_BASE }.into()),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                })
                .into();
        }

        let mut log_list = column![].spacing(2);

        for entry in self.entries.iter().rev().take(6) {
            let color = match entry.level {
                ProphecyLevel::Solar => MayaColors::SOLAR_GOLD,
                ProphecyLevel::Jade => MayaColors::JADE_GLOW,
                ProphecyLevel::Maya => MayaColors::MAYA_BLUE,
                ProphecyLevel::Xibalba => MayaColors::CINNABAR_BRIGHT,
            };

            let log_row = row![
                text(format!("[{}]", entry.timestamp))
                    .size(9)
                    .color(MayaColors::GLYPH_DIM),
                Space::new().width(4),
                text(entry.glyph).size(10),
                Space::new().width(4),
                text(&entry.message).size(10).color(color),
            ]
            .align_y(Alignment::Center);

            log_list = log_list.push(log_row);
        }

        let body = scrollable(log_list.padding(Padding::from([2, 10])));

        container(column![header, body])
            .width(Length::Fill)
            .height(72)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.95, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}
