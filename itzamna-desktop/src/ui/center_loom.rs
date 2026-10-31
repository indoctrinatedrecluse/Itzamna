//! The Center Commit Viewport (El Telar de Yaxche)
//! Integrates the interactive GPU DagCanvas with zoom controls and commit inspector.

use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::canvas::{DagCanvas, DagMessage};
use crate::git::GitCommitNode;
use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum CenterLoomMessage {
    Dag(DagMessage),
    ZoomIn,
    ZoomOut,
    ResetZoom,
}

pub struct CenterLoom {
    pub dag: DagCanvas,
    pub selected_commit: Option<GitCommitNode>,
}

impl Default for CenterLoom {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl CenterLoom {
    pub fn new(commits: Vec<GitCommitNode>) -> Self {
        let selected_commit = commits.first().cloned();
        Self {
            dag: DagCanvas::new(commits),
            selected_commit,
        }
    }

    pub fn set_commits(&mut self, commits: Vec<GitCommitNode>) {
        if self.selected_commit.is_none() {
            self.selected_commit = commits.first().cloned();
        }
        self.dag.set_commits(commits);
    }

    pub fn select_sha(&mut self, sha: &str) {
        self.dag.selected_sha = Some(sha.to_string());
        if let Some(c) = self.dag.commits.iter().find(|c| c.short_sha == sha || c.sha.starts_with(sha)) {
            self.selected_commit = Some(c.clone());
        }
    }

    pub fn view<'a, Message: 'a + From<CenterLoomMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let commit_count = self.dag.commits.len();

        let zoom_badge = format!("{:.0}%", self.dag.zoom * 100.0);

        let toolbar = row![
            text("🌳 YAXCHE TREE:").size(10).color(MayaColors::GLYPH_DIM),
            text(" main").size(11).color(MayaColors::JADE_GLOW),
            Space::new().width(12),
            text(format!("{} Commits Inscribed", commit_count))
                .size(10)
                .color(MayaColors::PARCHMENT_MUTED),
            Space::new().width(Length::Fill),
            // Zoom controls
            button(text("−").size(12).color(MayaColors::STARLIGHT_WHITE))
                .padding(Padding::from([2, 7]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(CenterLoomMessage::ZoomOut.into()),
            container(text(zoom_badge).size(10).color(MayaColors::SOLAR_GOLD))
                .padding(Padding::from([2, 6])),
            button(text("+").size(12).color(MayaColors::STARLIGHT_WHITE))
                .padding(Padding::from([2, 7]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(CenterLoomMessage::ZoomIn.into()),
            Space::new().width(6),
            button(text("☀️ HEAD").size(10).color(MayaColors::SOLAR_AMBER))
                .padding(Padding::from([3, 8]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::SOLAR_AMBER,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(CenterLoomMessage::ResetZoom.into()),
        ]
        .padding(Padding::from([5, 12]))
        .align_y(Alignment::Center);

        // DAG Canvas
        let dag_element: Element<'a, Message> = self.dag.view(1400.0, 750.0).map(|msg| CenterLoomMessage::Dag(msg).into());

        // Inspector footer for the selected commit
        let inspector = if let Some(c) = &self.selected_commit {
            row![
                text("☀️ KIN:").size(10).color(MayaColors::GLYPH_DIM),
                text(&c.short_sha).size(11).color(MayaColors::SOLAR_GOLD),
                Space::new().width(10),
                text(&c.summary).size(11).color(MayaColors::STARLIGHT_WHITE),
                Space::new().width(Length::Fill),
                text(&c.author_name).size(10).color(MayaColors::JADE_GLOW),
                Space::new().width(8),
                text(&c.relative_time).size(10).color(MayaColors::GLYPH_DIM),
            ]
            .padding(Padding::from([6, 12]))
            .align_y(Alignment::Center)
        } else {
            row![text("Select a commit from the Yaxche tree to inspect").size(10).color(MayaColors::GLYPH_DIM)]
                .padding(Padding::from([6, 12]))
                .align_y(Alignment::Center)
        };

        let inspector_container = container(inspector)
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.90, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            });

        container(column![toolbar, dag_element, inspector_container])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.50, ..MayaColors::OBSIDIAN_BASE }.into()),
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
