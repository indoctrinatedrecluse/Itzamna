//! The Center Commit Viewport (El Telar de Yaxche)
//! Integrates the interactive GPU DagCanvas, zoom controls, and the Etz'nab' Stone Diff Workbench.

use iced::widget::{button, column, container, row, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::canvas::{DagCanvas, DagMessage};
use crate::git::GitCommitNode;
use crate::theme::MayaColors;
use crate::ui::diff_workbench::{DiffMessage, DiffWorkbench};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterTab {
    YaxcheDag,
    StoneDiff,
}

#[derive(Debug, Clone)]
pub enum CenterLoomMessage {
    Dag(DagMessage),
    Diff(DiffMessage),
    SelectTab(CenterTab),
    ZoomIn,
    ZoomOut,
    ResetZoom,
}

pub struct CenterLoom {
    pub active_tab: CenterTab,
    pub dag: DagCanvas,
    pub diff_workbench: DiffWorkbench,
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
            active_tab: CenterTab::YaxcheDag,
            dag: DagCanvas::new(commits),
            diff_workbench: DiffWorkbench::default(),
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

    pub fn open_file_diff(&mut self, path: String) {
        self.diff_workbench = DiffWorkbench::new(path);
        self.active_tab = CenterTab::StoneDiff;
    }

    pub fn view<'a, Message: 'a + From<CenterLoomMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let tab_bar = row![
            self.tab_button("🌳 YAXCHE COMMIT DAG", CenterTab::YaxcheDag),
            Space::new().width(4),
            self.tab_button("🔪 ETZ'NAB' STONE DIFF", CenterTab::StoneDiff),
            Space::new().width(12),
            text(format!("{} Commits", self.dag.commits.len()))
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            // Zoom controls (only active in DAG mode)
            if self.active_tab == CenterTab::YaxcheDag {
                row![
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
                    container(text(format!("{:.0}%", self.dag.zoom * 100.0)).size(10).color(MayaColors::SOLAR_GOLD))
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
                .align_y(Alignment::Center)
            } else {
                row![]
            },
        ]
        .padding(Padding::from([5, 12]))
        .align_y(Alignment::Center);

        let main_content: Element<'a, Message> = match self.active_tab {
            CenterTab::YaxcheDag => {
                let dag_element: Element<'a, Message> = self
                    .dag
                    .view(1400.0, 750.0)
                    .map(|msg| CenterLoomMessage::Dag(msg).into());

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

                column![dag_element, inspector_container].into()
            }
            CenterTab::StoneDiff => {
                self.diff_workbench
                    .view()
                    .map(|msg| CenterLoomMessage::Diff(msg).into())
            }
        };

        container(column![tab_bar, main_content])
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

    fn tab_button<'a, Message: 'a + From<CenterLoomMessage> + Clone>(
        &'a self,
        label: &'static str,
        tab: CenterTab,
    ) -> Element<'a, Message> {
        let is_active = self.active_tab == tab;
        let text_color = if is_active {
            MayaColors::SOLAR_GOLD
        } else {
            MayaColors::PARCHMENT_MUTED
        };

        button(text(label).size(10).color(text_color))
            .padding(Padding::from([3, 8]))
            .style(move |_theme, status| {
                let border = if is_active {
                    Border {
                        color: MayaColors::SOLAR_GOLD,
                        width: 1.0,
                        radius: 3.0.into(),
                    }
                } else {
                    match status {
                        button::Status::Hovered => Border {
                            color: MayaColors::JADE_GLOW,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        _ => Border {
                            color: MayaColors::OBSIDIAN_BORDER,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                    }
                };

                button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border,
                    ..Default::default()
                }
            })
            .on_press(CenterLoomMessage::SelectTab(tab).into())
            .into()
    }
}
