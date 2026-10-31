//! The Center Commit Viewport (El Telar de Yaxche)
//! Central timeline stage rendering topological commit lanes and hieroglyphic commit seals.

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum CenterLoomMessage {
    SelectCommit(String),
    FilterChanged(String),
}

#[derive(Debug, Clone)]
pub struct CommitNode {
    pub sha: String,
    pub short_sha: String,
    pub message: String,
    pub author: String,
    pub relative_time: String,
    pub branch_lane: usize,
    pub is_head: bool,
    pub tags: Vec<String>,
}

pub struct CenterLoom {
    pub commits: Vec<CommitNode>,
    pub selected_sha: Option<String>,
}

impl Default for CenterLoom {
    fn default() -> Self {
        Self::new()
    }
}

impl CenterLoom {
    pub fn new() -> Self {
        Self {
            commits: vec![
                CommitNode {
                    sha: "3b7f7a192847a9bc".into(),
                    short_sha: "3b7f7a1".into(),
                    message: "feat(desktop): complete Phase 1 — The Cosmic Shell with animated Tzolk'in Loom and Sacred Glyph Ribbon".into(),
                    author: "indoctrinatedrecluse".into(),
                    relative_time: "8 min ago".into(),
                    branch_lane: 0,
                    is_head: true,
                    tags: vec!["v0.1.0-genesis".into()],
                },
                CommitNode {
                    sha: "4076417aa9812bc4".into(),
                    short_sha: "4076417".into(),
                    message: "feat: initial commit — Itzamna + Ixchel architecture specification and roadmap".into(),
                    author: "indoctrinatedrecluse".into(),
                    relative_time: "22 min ago".into(),
                    branch_lane: 0,
                    is_head: false,
                    tags: vec![],
                },
            ],
            selected_sha: Some("3b7f7a1".into()),
        }
    }

    pub fn view<'a, Message: 'a + From<CenterLoomMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let filter_bar = row![
            text("🌳 TIMELINE:").size(10).color(MayaColors::GLYPH_DIM),
            text(" main").size(11).color(MayaColors::JADE_GLOW),
            Space::new().width(14),
            text("🔍 Filter Inscriptions...").size(10).color(MayaColors::PARCHMENT_MUTED),
            Space::new().width(Length::Fill),
            text("Showing 2 of 2 Commits").size(10).color(MayaColors::GLYPH_DIM),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        let mut timeline_list = column![].spacing(6);

        for commit in &self.commits {
            let is_selected = self.selected_sha.as_deref() == Some(&commit.short_sha);

            let node_color = if commit.is_head {
                MayaColors::SOLAR_GOLD
            } else {
                MayaColors::JADE_GLOW
            };

            let node_symbol = if commit.is_head { "☀️" } else { "🟢" };

            let mut header_row = row![
                text(node_symbol).size(12),
                Space::new().width(6),
                text(&commit.short_sha)
                    .size(11)
                    .color(MayaColors::SOLAR_AMBER),
                Space::new().width(8),
                text(&commit.message)
                    .size(11)
                    .color(if is_selected { MayaColors::STARLIGHT_WHITE } else { MayaColors::PARCHMENT_MUTED }),
                Space::new().width(Length::Fill),
                text(&commit.author).size(10).color(MayaColors::GLYPH_DIM),
                Space::new().width(8),
                text(&commit.relative_time).size(10).color(MayaColors::GLYPH_DIM),
            ]
            .align_y(Alignment::Center);

            for tag in &commit.tags {
                header_row = header_row.push(Space::new().width(6)).push(
                    container(text(tag).size(9).color(MayaColors::OBSIDIAN_VOID))
                        .padding(Padding::from([1, 5]))
                        .style(|_theme| container::Style {
                            background: Some(MayaColors::SOLAR_GOLD.into()),
                            border: Border {
                                radius: 2.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                );
            }

            let commit_card = button(header_row)
                .width(Length::Fill)
                .padding(Padding::from([6, 10]))
                .style(move |_theme, status| {
                    let (bg, border) = if is_selected {
                        (
                            Color { a: 0.45, ..MayaColors::OBSIDIAN_SURFACE },
                            node_color,
                        )
                    } else {
                        match status {
                            button::Status::Hovered => (
                                Color { a: 0.25, ..MayaColors::OBSIDIAN_SURFACE },
                                MayaColors::OBSIDIAN_BORDER,
                            ),
                            _ => (
                                Color { a: 0.15, ..MayaColors::OBSIDIAN_BASE },
                                Color::TRANSPARENT,
                            ),
                        }
                    };

                    button::Style {
                        background: Some(bg.into()),
                        border: Border {
                            color: border,
                            width: 1.0,
                            radius: 4.0.into(),
                        },
                        ..Default::default()
                    }
                })
                .on_press(CenterLoomMessage::SelectCommit(commit.short_sha.clone()).into());

            timeline_list = timeline_list.push(commit_card);
        }

        let scroll_body = scrollable(timeline_list.padding(Padding::from([8, 12])))
            .height(Length::Fill);

        container(column![filter_bar, scroll_body])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.55, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}
