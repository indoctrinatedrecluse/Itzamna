//! The Popol Vuh Chronicle (La Crónica del Popol Vuh — Advanced Git Log & History Explorer)
//! Deep commit query filtering, author criteria, and commit inspector links.

use iced::widget::{button, column, container, row, scrollable, text, text_input, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::git::GitCommitNode;
use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum PopolLogMessage {
    QueryChanged(String),
    AuthorFilterChanged(String),
    ExecuteFilter,
    SelectCommitSha(String),
}

pub struct PopolVuhLogWorkbench {
    pub query_input: String,
    pub author_input: String,
    pub commits: Vec<GitCommitNode>,
    pub total_count: usize,
}

impl Default for PopolVuhLogWorkbench {
    fn default() -> Self {
        Self::new()
    }
}

impl PopolVuhLogWorkbench {
    pub fn new() -> Self {
        Self {
            query_input: "".to_string(),
            author_input: "".to_string(),
            commits: Vec::new(),
            total_count: 0,
        }
    }

    pub fn set_commits(&mut self, commits: Vec<GitCommitNode>) {
        self.total_count = commits.len();
        self.commits = commits;
    }

    pub fn view<'a, Message: 'a + From<PopolLogMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_bar = row![
            text("📜 LA CRÓNICA DEL POPOL VUH (GIT LOG CHRONICLE)")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(10),
            text(format!("Showing {} / {} Ancestral Seals", self.commits.len(), self.total_count))
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            text_input("Filter message or SHA (git log --grep)...", &self.query_input)
                .size(10)
                .padding(Padding::from([3, 7]))
                .width(200)
                .style(|_theme, _status| text_input::Style {
                    background: MayaColors::OBSIDIAN_SURFACE.into(),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    icon: MayaColors::PARCHMENT_MUTED,
                    placeholder: MayaColors::GLYPH_DIM,
                    value: MayaColors::STARLIGHT_WHITE,
                    selection: MayaColors::JADE_SHADOW,
                })
                .on_input(|val| PopolLogMessage::QueryChanged(val).into()),
            Space::new().width(6),
            text_input("Author filter...", &self.author_input)
                .size(10)
                .padding(Padding::from([3, 7]))
                .width(130)
                .style(|_theme, _status| text_input::Style {
                    background: MayaColors::OBSIDIAN_SURFACE.into(),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    icon: MayaColors::PARCHMENT_MUTED,
                    placeholder: MayaColors::GLYPH_DIM,
                    value: MayaColors::STARLIGHT_WHITE,
                    selection: MayaColors::JADE_SHADOW,
                })
                .on_input(|val| PopolLogMessage::AuthorFilterChanged(val).into()),
            Space::new().width(6),
            tooltip(
                button(text("🔍 Filter").size(10).color(MayaColors::OBSIDIAN_VOID))
                    .padding(Padding::from([3, 10]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::JADE_GLOW.into()),
                        border: Border {
                            color: MayaColors::JADE_SHADOW,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(PopolLogMessage::ExecuteFilter.into()),
                text("Filter commit chronicle by message and author keywords").size(11),
                tooltip::Position::Bottom,
            ),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        let mut commit_cards = column![].spacing(6);

        if self.commits.is_empty() {
            commit_cards = commit_cards.push(
                container(text("No ancestral seals match the glyph query.").size(11).color(MayaColors::GLYPH_DIM))
                    .padding(16)
            );
        }

        for commit in &self.commits {
            commit_cards = commit_cards.push(self.render_commit_card(commit));
        }

        let body = scrollable(commit_cards.padding(Padding::from([8, 12])))
            .height(Length::Fill);

        container(column![title_bar, body])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.60, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_commit_card<'a, Message: 'a + From<PopolLogMessage> + Clone>(
        &'a self,
        commit: &'a GitCommitNode,
    ) -> Element<'a, Message> {
        let tag_chips = if !commit.tags.is_empty() {
            let mut r = row![].spacing(3).align_y(Alignment::Center);
            for t in &commit.tags {
                r = r.push(
                    container(text(format!("🏷️ {}", t)).size(8).color(MayaColors::OBSIDIAN_VOID))
                        .padding(Padding::from([1, 4]))
                        .style(|_theme| container::Style {
                            background: Some(MayaColors::SOLAR_AMBER.into()),
                            border: Border {
                                radius: 2.0.into(),
                                ..Default::default()
                            },
                            ..Default::default()
                        })
                );
            }
            r
        } else {
            row![]
        };

        let card_header = row![
            text("☀️ ").size(10),
            text(&commit.short_sha).size(10).color(MayaColors::SOLAR_GOLD),
            Space::new().width(6),
            tag_chips,
            Space::new().width(8),
            text(&commit.summary)
                .size(11)
                .color(MayaColors::STARLIGHT_WHITE)
                .wrapping(text::Wrapping::Word)
                .width(Length::Fill),
            Space::new().width(8),
            text(&commit.author_name).size(10).color(MayaColors::JADE_GLOW),
            Space::new().width(6),
            text(&commit.relative_time).size(9).color(MayaColors::GLYPH_DIM),
            Space::new().width(8),
            tooltip(
                button(text("Inspect Diff →").size(9).color(MayaColors::SOLAR_GOLD))
                    .padding(Padding::from([1, 6]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border {
                            color: MayaColors::OBSIDIAN_BORDER,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(PopolLogMessage::SelectCommitSha(commit.sha.clone()).into()),
                text("Focus commit in the Yaxche DAG and open diffs").size(11),
                tooltip::Position::Left,
            ),
        ]
        .align_y(Alignment::Center);

        container(card_header)
            .width(Length::Fill)
            .padding(Padding::from([6, 10]))
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.30, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}
