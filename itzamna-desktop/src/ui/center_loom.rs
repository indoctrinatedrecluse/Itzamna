//! The Center Commit Viewport (El Telar de Yaxche)
//! Integrates GPU DagCanvas, Stone Diff Workbench, Rebase Crucible, and Oracle Radar.

use iced::widget::{button, column, container, row, scrollable, text, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::canvas::{DagCanvas, DagMessage};
use crate::git::{DetailedCommit, GitCommitNode};
use crate::theme::MayaColors;
use crate::ui::ahau_blame::{AhauBlameWorkbench, BlameMessage};
use crate::ui::chac_purifier::{ChacPurifierWorkbench, PurifierMessage};
use crate::ui::diff_workbench::{DiffMessage, DiffWorkbench};
use crate::ui::kukulcan_merge::{KukulcanMergeWorkbench, MergeMessage};
use crate::ui::markdown::MayanMarkdown;
use crate::ui::oracle_radar::{OracleMessage, OracleRadar};
use crate::ui::overseer_portals::{OverseerPortalsWorkbench, PortalMessage};
use crate::ui::popol_vuh_log::{PopolLogMessage, PopolVuhLogWorkbench};
use crate::ui::rebase_crucible::{RebaseCrucible, RebaseMessage};
use crate::ui::xibalba_reflog::{ReflogMessage, XibalbaReflogWorkbench};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CenterTab {
    YaxcheDag,
    StoneDiff,
    AhauBlame,
    PopolVuhLog,
    RebaseCrucible,
    KukulcanMerge,
    XibalbaReflog,
    ChacPurifier,
    OverseerPortals,
    OracleRadar,
}

#[derive(Debug, Clone)]
pub enum CenterLoomMessage {
    Dag(DagMessage),
    Diff(DiffMessage),
    Blame(BlameMessage),
    PopolLog(PopolLogMessage),
    Rebase(RebaseMessage),
    Merge(MergeMessage),
    Reflog(ReflogMessage),
    Purifier(PurifierMessage),
    Portal(PortalMessage),
    Oracle(OracleMessage),
    SelectTab(CenterTab),
    ZoomIn,
    ZoomOut,
    ResetZoom,
    OpenFileInDiff(String),
    SelectParent(String),
    CloseCommitDetails,
}

pub struct CenterLoom {
    pub active_tab: CenterTab,
    pub dag: DagCanvas,
    pub diff_workbench: DiffWorkbench,
    pub ahau_blame: AhauBlameWorkbench,
    pub popol_log: PopolVuhLogWorkbench,
    pub rebase_crucible: RebaseCrucible,
    pub kukulcan_merge: KukulcanMergeWorkbench,
    pub xibalba_reflog: XibalbaReflogWorkbench,
    pub chac_purifier: ChacPurifierWorkbench,
    pub overseer_portals: OverseerPortalsWorkbench,
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
            ahau_blame: AhauBlameWorkbench::default(),
            popol_log: PopolVuhLogWorkbench::default(),
            rebase_crucible: RebaseCrucible::new(),
            kukulcan_merge: KukulcanMergeWorkbench::default(),
            xibalba_reflog: XibalbaReflogWorkbench::default(),
            chac_purifier: ChacPurifierWorkbench::default(),
            overseer_portals: OverseerPortalsWorkbench::default(),
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
            self.tab_button("🌳 DAG", CenterTab::YaxcheDag),
            Space::new().width(2),
            self.tab_button("🔪 DIFF", CenterTab::StoneDiff),
            Space::new().width(2),
            self.tab_button("👑 BLAME", CenterTab::AhauBlame),
            Space::new().width(2),
            self.tab_button("📜 LOG", CenterTab::PopolVuhLog),
            Space::new().width(2),
            self.tab_button("⚖️ REBASE", CenterTab::RebaseCrucible),
            Space::new().width(2),
            self.tab_button("🌿 MERGE", CenterTab::KukulcanMerge),
            Space::new().width(2),
            self.tab_button("🪞 REFLOG", CenterTab::XibalbaReflog),
            Space::new().width(2),
            self.tab_button("🌧️ CLEAN", CenterTab::ChacPurifier),
            Space::new().width(2),
            self.tab_button("🌐 REMOTES", CenterTab::OverseerPortals),
            Space::new().width(2),
            self.tab_button("🔮 RADAR", CenterTab::OracleRadar),
            Space::new().width(8),
            text(format!("{} Commits", self.dag.commits.len()))
                .size(9)
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
        .padding(Padding::from([4, 8]))
        .align_y(Alignment::Center);

        let main_content: Element<'a, Message> = match self.active_tab {
            CenterTab::YaxcheDag => {
                let dag_element: Element<'a, Message> = self
                    .dag
                    .view(Length::Fill, Length::Fill)
                    .map(|msg| CenterLoomMessage::Dag(msg).into());

                let inspector = self.render_commit_details();

                if self.selected_commit.is_some() {
                    column![
                        container(dag_element)
                            .width(Length::Fill)
                            .height(Length::FillPortion(3)),
                        inspector,
                    ]
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
                } else {
                    column![
                        container(dag_element)
                            .width(Length::Fill)
                            .height(Length::Fill),
                        inspector,
                    ]
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into()
                }
            }
            CenterTab::StoneDiff => {
                self.diff_workbench
                    .view()
                    .map(|msg| CenterLoomMessage::Diff(msg).into())
            }
            CenterTab::AhauBlame => {
                self.ahau_blame
                    .view()
                    .map(|msg| CenterLoomMessage::Blame(msg).into())
            }
            CenterTab::PopolVuhLog => {
                self.popol_log
                    .view()
                    .map(|msg| CenterLoomMessage::PopolLog(msg).into())
            }
            CenterTab::RebaseCrucible => {
                self.rebase_crucible
                    .view()
                    .map(|msg| CenterLoomMessage::Rebase(msg).into())
            }
            CenterTab::KukulcanMerge => {
                self.kukulcan_merge
                    .view()
                    .map(|msg| CenterLoomMessage::Merge(msg).into())
            }
            CenterTab::XibalbaReflog => {
                self.xibalba_reflog
                    .view()
                    .map(|msg| CenterLoomMessage::Reflog(msg).into())
            }
            CenterTab::ChacPurifier => {
                self.chac_purifier
                    .view()
                    .map(|msg| CenterLoomMessage::Purifier(msg).into())
            }
            CenterTab::OverseerPortals => {
                self.overseer_portals
                    .view()
                    .map(|msg| CenterLoomMessage::Portal(msg).into())
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
            return container(
                row![
                    text("🔍 Select any commit node in the Yaxche tree to inspect full ancestral metadata, markdown prophecy, and file diffs")
                        .size(10)
                        .color(MayaColors::GLYPH_DIM)
                ]
                .align_y(Alignment::Center)
                .padding(Padding::from([4, 12]))
            )
            .width(Length::Fill)
            .height(Length::Fixed(28.0))
            .style(|_| container::Style {
                background: Some(Color { a: 0.70, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into();
        };

        let detailed = self.detailed_commit.as_ref();

        // 1. Header Bar: Status, Summary, Author, Time, Churn, and Close
        let header_row = row![
            container(text(format!("☀️ KIN: {}", c.short_sha)).size(11).color(MayaColors::SOLAR_GOLD))
                .padding(Padding::from([2, 6]))
                .style(|_| container::Style {
                    background: Some(Color { a: 0.25, ..MayaColors::SOLAR_GOLD }.into()),
                    border: Border { color: MayaColors::SOLAR_GOLD, width: 0.8, radius: 3.0.into() },
                    ..Default::default()
                }),
            Space::new().width(8),
            text(&c.summary).size(12).color(MayaColors::STARLIGHT_WHITE),
            Space::new().width(Length::Fill),
            text(&c.author_name).size(10).color(MayaColors::JADE_GLOW),
            text(" · ").size(10).color(MayaColors::GLYPH_DIM),
            text(&c.relative_time).size(10).color(MayaColors::GLYPH_DIM),
            Space::new().width(12),
            if let Some(det) = detailed {
                row![
                    text(format!("+{}", det.total_additions)).size(10).color(MayaColors::JADE_GLOW),
                    Space::new().width(4),
                    text(format!("-{}", det.total_deletions)).size(10).color(MayaColors::CINNABAR_BRIGHT),
                ]
                .align_y(Alignment::Center)
            } else {
                row![]
            },
            Space::new().width(12),
            tooltip(
                button(text("✕").size(10).color(MayaColors::PARCHMENT_MUTED))
                    .padding(Padding::from([1, 6]))
                    .style(|_, status| {
                        let text_color = match status {
                            button::Status::Hovered => MayaColors::CINNABAR_BRIGHT,
                            _ => MayaColors::PARCHMENT_MUTED,
                        };
                        button::Style {
                            background: Some(MayaColors::OBSIDIAN_BASE.into()),
                            border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 0.8, radius: 2.0.into() },
                            text_color,
                            ..Default::default()
                        }
                    })
                    .on_press(CenterLoomMessage::CloseCommitDetails.into()),
                text("Collapse commit inspector").size(11),
                tooltip::Position::Top,
            ),
        ]
        .padding(Padding::from([4, 10]))
        .align_y(Alignment::Center);

        // 2. Column 1: Ancestral Lineage & ODB Records
        let mut lineage_col = column![
            row![
                text("🏛️ ANCESTRAL LINEAGE").size(9).color(MayaColors::SOLAR_GOLD),
                Space::new().width(Length::Fill),
                text("ODB").size(8).color(MayaColors::GLYPH_DIM),
            ],
            Space::new().height(2),
            text(format!("⏰ Exact: {}", detailed.map(|d| d.author_time.as_str()).unwrap_or(&c.relative_time)))
                .size(9)
                .color(MayaColors::STARLIGHT_WHITE),
            text(format!("👤 Author: {}", detailed.map(|d| d.author_name.as_str()).unwrap_or(&c.author_name)))
                .size(9)
                .color(MayaColors::JADE_GLOW),
        ]
        .spacing(3);

        if let Some(d) = detailed {
            lineage_col = lineage_col.push(
                text(format!("📧 {}", d.author_email))
                    .size(8)
                    .color(MayaColors::GLYPH_DIM)
            );
            if d.committer_name != d.author_name {
                lineage_col = lineage_col.push(
                    column![
                        Space::new().height(2),
                        text(format!("✍️ Committer: {}", d.committer_name)).size(9).color(MayaColors::MAYA_BLUE),
                        text(format!("📧 {}", d.committer_email)).size(8).color(MayaColors::GLYPH_DIM),
                    ]
                );
            }
            lineage_col = lineage_col.push(
                text(format!("🌲 Tree: {}", d.tree_sha))
                    .size(9)
                    .color(MayaColors::GLYPH_DIM)
            );
            lineage_col = lineage_col.push(Space::new().height(4));
            lineage_col = lineage_col.push(
                text("Parent Inscriptions:").size(9).color(MayaColors::SOLAR_AMBER)
            );

            if d.parent_shas.is_empty() {
                lineage_col = lineage_col.push(
                    text("Genesis Altar (Root Commit)").size(8).color(MayaColors::GLYPH_DIM)
                );
            } else {
                for p_sha in &d.parent_shas {
                    let sha_clone = p_sha.clone();
                    let p_btn = button(
                        row![
                            text("⬆️ ").size(9).color(MayaColors::SOLAR_AMBER),
                            text(p_sha).size(9).color(MayaColors::STARLIGHT_WHITE),
                        ]
                        .align_y(Alignment::Center)
                    )
                    .padding(Padding::from([2, 5]))
                    .style(|_, status| {
                        let border_color = match status {
                            button::Status::Hovered => MayaColors::SOLAR_GOLD,
                            _ => MayaColors::OBSIDIAN_BORDER,
                        };
                        button::Style {
                            background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                            border: Border { color: border_color, width: 0.8, radius: 2.0.into() },
                            ..Default::default()
                        }
                    })
                    .on_press(CenterLoomMessage::SelectParent(sha_clone).into());

                    lineage_col = lineage_col.push(p_btn);
                }
            }
        }

        let col1 = container(
            scrollable(lineage_col.padding(Padding::from([6, 8]))).height(Length::Fill)
        )
        .width(Length::Fixed(240.0))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color { a: 0.65, ..MayaColors::OBSIDIAN_BASE }.into()),
            border: Border {
                color: MayaColors::OBSIDIAN_BORDER,
                width: 1.0,
                radius: 3.0.into(),
            },
            ..Default::default()
        });

        // 3. Column 2: Auto-Parsed Mayan Markdown Inscription
        let full_msg = detailed.map(|d| d.full_message.as_str()).unwrap_or(&c.summary);
        let markdown_content = MayanMarkdown::render(full_msg);

        let col2 = container(
            column![
                row![
                    text("📜 SACRED INSCRIPTION & PROPHETIC RECORD").size(9).color(MayaColors::SOLAR_GOLD),
                    Space::new().width(Length::Fill),
                    text("Auto-Parsed Markdown").size(8).color(MayaColors::JADE_GLOW),
                ]
                .padding(Padding::from([4, 8])),
                scrollable(
                    container(markdown_content)
                        .padding(Padding::from([4, 10]))
                        .width(Length::Fill)
                )
                .height(Length::Fill),
            ]
        )
        .width(Length::FillPortion(4))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color { a: 0.75, ..MayaColors::OBSIDIAN_BASE }.into()),
            border: Border {
                color: MayaColors::OBSIDIAN_BORDER,
                width: 1.0,
                radius: 3.0.into(),
            },
            ..Default::default()
        });

        // 4. Column 3: Complete Changed Files Codex
        let mut files_column = column![].spacing(3).padding(Padding::from([4, 6]));

        if let Some(det) = detailed {
            if det.changed_files.is_empty() {
                files_column = files_column.push(
                    text("No file modifications recorded in this commit").size(9).color(MayaColors::GLYPH_DIM)
                );
            } else {
                for f in &det.changed_files {
                    let (status_color, status_bg) = match f.status {
                        "A" => (MayaColors::JADE_GLOW, Color { a: 0.2, ..MayaColors::JADE_GLOW }),
                        "D" => (MayaColors::CINNABAR_BRIGHT, Color { a: 0.2, ..MayaColors::CINNABAR_BRIGHT }),
                        "R" => (MayaColors::MAYA_BLUE, Color { a: 0.2, ..MayaColors::MAYA_BLUE }),
                        _ => (MayaColors::SOLAR_AMBER, Color { a: 0.2, ..MayaColors::SOLAR_AMBER }),
                    };

                    let path_clone = f.path.clone();
                    let f_row = row![
                        container(text(f.status).size(8).color(status_color))
                            .padding(Padding::from([1, 4]))
                            .style(move |_| container::Style {
                                background: Some(status_bg.into()),
                                border: Border { color: status_color, width: 0.5, radius: 2.0.into() },
                                ..Default::default()
                            }),
                        Space::new().width(6),
                        text(&f.path)
                            .size(9)
                            .color(MayaColors::STARLIGHT_WHITE)
                            .width(Length::Fill),
                        Space::new().width(4),
                        text(format!("+{}", f.additions)).size(8).color(MayaColors::JADE_GLOW),
                        Space::new().width(2),
                        text(format!("-{}", f.deletions)).size(8).color(MayaColors::CINNABAR_BRIGHT),
                        Space::new().width(6),
                        tooltip(
                            button(text("DIFF ➔").size(8).color(MayaColors::SOLAR_GOLD))
                                .padding(Padding::from([1, 5]))
                                .style(|_, status| {
                                    let border_color = match status {
                                        button::Status::Hovered => MayaColors::SOLAR_GOLD,
                                        _ => MayaColors::OBSIDIAN_BORDER,
                                    };
                                    button::Style {
                                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                                        border: Border { color: border_color, width: 0.8, radius: 2.0.into() },
                                        ..Default::default()
                                    }
                                })
                                .on_press(CenterLoomMessage::OpenFileInDiff(path_clone).into()),
                            text("Inspect file diff in Etz'nab' Stone Diff (git diff)").size(10),
                            tooltip::Position::Top,
                        ),
                    ]
                    .padding(Padding::from([2, 4]))
                    .align_y(Alignment::Center);

                    files_column = files_column.push(
                        container(f_row)
                            .width(Length::Fill)
                            .style(|_| container::Style {
                                background: Some(Color { a: 0.2, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                                border: Border { radius: 2.0.into(), ..Default::default() },
                                ..Default::default()
                            })
                    );
                }
            }
        } else {
            files_column = files_column.push(
                text("Retrieving ODB diff tree...").size(9).color(MayaColors::GLYPH_DIM)
            );
        }

        let col3 = container(
            column![
                row![
                    text("🏺 ALTERED RELICS & TREE DELTAS").size(9).color(MayaColors::SOLAR_GOLD),
                    Space::new().width(Length::Fill),
                    text(format!("{} files", detailed.map(|d| d.changed_files.len()).unwrap_or(0))).size(8).color(MayaColors::GLYPH_DIM),
                ]
                .padding(Padding::from([4, 8])),
                scrollable(files_column).height(Length::Fill),
            ]
        )
        .width(Length::FillPortion(3))
        .height(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color { a: 0.65, ..MayaColors::OBSIDIAN_BASE }.into()),
            border: Border {
                color: MayaColors::OBSIDIAN_BORDER,
                width: 1.0,
                radius: 3.0.into(),
            },
            ..Default::default()
        });

        let columns_row = row![
            col1,
            col2,
            col3,
        ]
        .spacing(6)
        .width(Length::Fill)
        .height(Length::Fill);

        let content = column![
            header_row,
            container(Space::new())
                .width(Length::Fill)
                .height(1)
                .style(|_| container::Style {
                    background: Some(MayaColors::OBSIDIAN_BORDER.into()),
                    ..Default::default()
                }),
            container(columns_row)
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(Padding::from([4, 8])),
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        container(content)
            .width(Length::Fill)
            .height(Length::FillPortion(2))
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.94, ..MayaColors::OBSIDIAN_SURFACE }.into()),
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
            CenterTab::AhauBlame => "The Ahau Lineage: Line-by-line file blame and age heatmap (git blame)",
            CenterTab::PopolVuhLog => "The Popol Vuh Chronicle: Advanced search & commit history (git log --grep)",
            CenterTab::RebaseCrucible => "Interactive timeline rebase sequencer & 3-way conflict resolver (git rebase -i / git merge)",
            CenterTab::KukulcanMerge => "The K'uk'ulkan Confluence: Branch merge & cherry-pick altar (git merge / cherry-pick)",
            CenterTab::XibalbaReflog => "The Xibalba Mirror: Visual reflog timeline & reset dials (git reflog / reset)",
            CenterTab::ChacPurifier => "The Chac Purifier: Untracked file cleaner & multi-stash vault (git clean / stash)",
            CenterTab::OverseerPortals => "The Overseer Portals: Remote sync, force-with-lease push & prune (git push / fetch)",
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
