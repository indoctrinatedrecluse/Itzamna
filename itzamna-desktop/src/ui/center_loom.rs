//! The Center Commit Viewport (El Telar de Yaxche)
//! Integrates GPU DagCanvas, Stone Diff Workbench, Rebase Crucible, and Oracle Radar.

use iced::widget::{button, column, container, row, text, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::canvas::{DagCanvas, DagMessage};
use crate::git::{DetailedCommit, GitCommitNode};
use crate::theme::MayaColors;
use crate::ui::diff_workbench::{DiffMessage, DiffWorkbench};
use crate::ui::oracle_radar::{OracleMessage, OracleRadar};
use crate::ui::rebase_crucible::{RebaseCrucible, RebaseMessage};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterTab {
    YaxcheDag,
    StoneDiff,
    RebaseCrucible,
    OracleRadar,
}

#[derive(Debug, Clone)]
pub enum CenterLoomMessage {
    Dag(DagMessage),
    Diff(DiffMessage),
    Rebase(RebaseMessage),
    Oracle(OracleMessage),
    SelectTab(CenterTab),
    ZoomIn,
    ZoomOut,
    ResetZoom,
    OpenFileInDiff(String),
}

pub struct CenterLoom {
    pub active_tab: CenterTab,
    pub dag: DagCanvas,
    pub diff_workbench: DiffWorkbench,
    pub rebase_crucible: RebaseCrucible,
    pub oracle_radar: OracleRadar,
    pub selected_commit: Option<GitCommitNode>,
    pub detailed_commit: Option<DetailedCommit>,
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
            rebase_crucible: RebaseCrucible::new(),
            oracle_radar: OracleRadar::new(),
            selected_commit,
            detailed_commit: None,
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
            Space::new().width(4),
            self.tab_button("⚖️ REBASE CRUCIBLE", CenterTab::RebaseCrucible),
            Space::new().width(4),
            self.tab_button("🔮 IXCHEL RADAR", CenterTab::OracleRadar),
            Space::new().width(12),
            text(format!("{} Commits", self.dag.commits.len()))
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            if self.active_tab == CenterTab::YaxcheDag {
                row![
                    tooltip(
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
                        text("Zoom out commit DAG").size(11),
                        tooltip::Position::Bottom,
                    ),
                    container(text(format!("{:.0}%", self.dag.zoom * 100.0)).size(10).color(MayaColors::SOLAR_GOLD))
                        .padding(Padding::from([2, 6])),
                    tooltip(
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
                        text("Zoom in commit DAG").size(11),
                        tooltip::Position::Bottom,
                    ),
                    Space::new().width(6),
                    tooltip(
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
                        text("Reset zoom and focus on active HEAD commit").size(11),
                        tooltip::Position::Bottom,
                    ),
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

                let inspector = self.render_commit_details();

                column![dag_element, inspector].into()
            }
            CenterTab::StoneDiff => {
                self.diff_workbench
                    .view()
                    .map(|msg| CenterLoomMessage::Diff(msg).into())
            }
            CenterTab::RebaseCrucible => {
                self.rebase_crucible
                    .view()
                    .map(|msg| CenterLoomMessage::Rebase(msg).into())
            }
            CenterTab::OracleRadar => {
                self.oracle_radar
                    .view()
                    .map(|msg| CenterLoomMessage::Oracle(msg).into())
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

    fn render_commit_details<'a, Message: 'a + From<CenterLoomMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let Some(c) = &self.selected_commit else {
            return container(row![text("Select a commit from the Yaxche tree to inspect").size(10).color(MayaColors::GLYPH_DIM)])
                .width(Length::Fill)
                .padding(Padding::from([6, 12]))
                .into();
        };

        let detailed = self.detailed_commit.as_ref();

        let header_row = row![
            text("☀️ KIN:").size(10).color(MayaColors::GLYPH_DIM),
            text(&c.short_sha).size(11).color(MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            text(&c.summary).size(11).color(MayaColors::STARLIGHT_WHITE),
            Space::new().width(Length::Fill),
            text(&c.author_name).size(10).color(MayaColors::JADE_GLOW),
            Space::new().width(8),
            text(&c.relative_time).size(10).color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        // If detailed commit is loaded, show changed files list with clickable diff links
        let files_row = if let Some(det) = detailed {
            let mut file_buttons = row![
                text(format!("Changed Files ({}): ", det.changed_files.len())).size(10).color(MayaColors::GLYPH_DIM)
            ].align_y(Alignment::Center).spacing(4);

            for f in det.changed_files.iter().take(5) {
                let status_color = match f.status {
                    "A" => MayaColors::JADE_GLOW,
                    "D" => MayaColors::CINNABAR_BRIGHT,
                    _ => MayaColors::SOLAR_AMBER,
                };

                let path_clone = f.path.clone();
                let f_btn = tooltip(
                    button(
                        row![
                            text(f.status).size(9).color(status_color),
                            Space::new().width(3),
                            text(&f.path).size(9).color(MayaColors::STARLIGHT_WHITE),
                        ]
                        .align_y(Alignment::Center)
                    )
                    .padding(Padding::from([1, 5]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_BASE.into()),
                        border: Border {
                            color: MayaColors::OBSIDIAN_BORDER,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(CenterLoomMessage::OpenFileInDiff(path_clone).into()),
                    text("Inspect file diff in Etz'nab' Stone Diff (git diff)").size(11),
                    tooltip::Position::Top,
                );

                file_buttons = file_buttons.push(f_btn);
            }

            file_buttons
        } else {
            row![text("Inspecting ODB tree...").size(9).color(MayaColors::GLYPH_DIM)].align_y(Alignment::Center)
        };

        let content = column![
            header_row,
            Space::new().height(4),
            files_row,
        ]
        .padding(Padding::from([6, 12]));

        container(content)
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.92, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
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

        let tooltip_desc = match tab {
            CenterTab::YaxcheDag => "Visual commit DAG graph with branch lanes & merge ribbons (git log --graph --all)",
            CenterTab::StoneDiff => "Interactive intraline diff viewer with hunk-level staging (git diff)",
            CenterTab::RebaseCrucible => "Interactive timeline rebase sequencer & 3-way conflict resolver (git rebase -i / git merge)",
            CenterTab::OracleRadar => "Repository health radar: DORA velocity metrics, lead time, and team analytics",
        };

        let btn = button(text(label).size(10).color(text_color))
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
            .on_press(CenterLoomMessage::SelectTab(tab).into());

        tooltip(btn, text(tooltip_desc).size(11), tooltip::Position::Bottom).into()
    }
}
