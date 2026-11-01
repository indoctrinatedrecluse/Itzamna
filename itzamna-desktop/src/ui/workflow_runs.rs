//! Celestial Workflows Altar (GitHub Actions & CI/CD Telemetry)
//! Powered by GitHub CLI (`gh`) to monitor live workflow runs, previous runs, and triggers.

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};
use serde::{Deserialize, Serialize};

use crate::theme::MayaColors;
use crate::ui::tooltip::mayan_tooltip;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowRunItem {
    #[serde(rename = "databaseId", default)]
    pub id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub status: String, // in_progress, completed, queued
    #[serde(default)]
    pub conclusion: Option<String>, // success, failure, cancelled
    #[serde(default)]
    pub event: String, // push, pull_request, workflow_dispatch
    #[serde(rename = "headBranch", default)]
    pub head_branch: String,
    #[serde(rename = "headSha", default)]
    pub head_sha: String,
    #[serde(rename = "createdAt", default)]
    pub created_at: String,
    #[serde(default)]
    pub url: String,
}

#[derive(Debug, Clone)]
pub enum WorkflowMessage {
    RefreshRuns,
    RunsLoaded(Result<Vec<WorkflowRunItem>, String>),
    RunsReceived(Result<Vec<WorkflowRunItem>, String>),
    OpenRunUrl(String),
    InstallGhCli,
    GhInstallStatus(String),
}

pub struct CelestialWorkflows {
    pub is_gh_installed: bool,
    pub runs: Vec<WorkflowRunItem>,
    pub is_loading: bool,
    pub status_message: Option<String>,
}

impl Default for CelestialWorkflows {
    fn default() -> Self {
        Self::new()
    }
}

impl CelestialWorkflows {
    pub fn new() -> Self {
        let is_gh_installed = Self::check_gh_installed();
        Self {
            is_gh_installed,
            runs: Vec::new(),
            is_loading: false,
            status_message: None,
        }
    }

    pub fn check_gh_installed() -> bool {
        std::process::Command::new("where.exe")
            .arg("gh")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false)
    }

    pub fn apply_runs(&mut self, res: Result<Vec<WorkflowRunItem>, String>) {
        self.is_loading = false;
        match res {
            Ok(runs) => {
                self.runs = runs;
                self.status_message = None;
            }
            Err(e) => {
                self.status_message = Some(e);
            }
        }
    }

    pub fn fetch_runs() -> Result<Vec<WorkflowRunItem>, String> {
        let output = std::process::Command::new("gh")
            .args([
                "run",
                "list",
                "--limit",
                "20",
                "--json",
                "databaseId,name,status,conclusion,event,headBranch,headSha,createdAt,url",
            ])
            .output()
            .map_err(|e| format!("Failed executing gh: {}", e))?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).to_string());
        }

        let runs: Vec<WorkflowRunItem> = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("JSON parse error: {}", e))?;

        Ok(runs)
    }

    pub fn view<'a, Message: 'a + From<WorkflowMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let header = row![
            text("⚡ CELESTIAL WORKFLOWS & GITHUB CI/CD").size(13).color(MayaColors::SOLAR_GOLD),
            Space::new().width(12),
            if self.is_gh_installed {
                container(text("GH CLI CONNECTED").size(8).color(MayaColors::JADE_GLOW))
                    .padding(Padding::from([2, 6]))
                    .style(|_| container::Style {
                        background: Some(Color { a: 0.25, ..MayaColors::JADE_GLOW }.into()),
                        border: Border { color: MayaColors::JADE_GLOW, width: 0.5, radius: 2.0.into() },
                        ..Default::default()
                    })
            } else {
                container(text("GH CLI MISSING").size(8).color(MayaColors::CINNABAR_BRIGHT))
                    .padding(Padding::from([2, 6]))
                    .style(|_| container::Style {
                        background: Some(Color { a: 0.25, ..MayaColors::CINNABAR_BRIGHT }.into()),
                        border: Border { color: MayaColors::CINNABAR_BRIGHT, width: 0.5, radius: 2.0.into() },
                        ..Default::default()
                    })
            },
            Space::new().width(Length::Fill),
            mayan_tooltip(
                button(
                    row![
                        text("🔄").size(11),
                        Space::new().width(4),
                        text("Refresh Runs").size(10).color(MayaColors::STARLIGHT_WHITE),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([4, 10]))
                .style(|_, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::SOLAR_GOLD,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 3.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(WorkflowMessage::RefreshRuns.into()),
                "Query latest GitHub Actions workflow status via GitHub CLI (gh run list)",
                iced::widget::tooltip::Position::Bottom,
            ),
        ]
        .padding(Padding::from([8, 14]))
        .align_y(Alignment::Center);

        if !self.is_gh_installed {
            let prompt_card = container(
                column![
                    text("🌑 GITHUB CLI NOT DETECTED").size(14).color(MayaColors::SOLAR_AMBER),
                    Space::new().height(8),
                    text("The Celestial Workflows view communicates directly with GitHub Actions using the official 'gh' CLI tool.")
                        .size(11)
                        .color(MayaColors::STARLIGHT_WHITE),
                    Space::new().height(4),
                    text("You can install it automatically using Windows Package Manager (winget) or download it directly.")
                        .size(10)
                        .color(MayaColors::PARCHMENT_MUTED),
                    Space::new().height(16),
                    button(
                        row![
                            text("🚀").size(12),
                            Space::new().width(6),
                            text("Install GitHub CLI via Winget").size(11).color(MayaColors::OBSIDIAN_VOID),
                        ]
                        .align_y(Alignment::Center)
                    )
                    .padding(Padding::from([8, 16]))
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
                    .on_press(WorkflowMessage::InstallGhCli.into()),
                ]
                .padding(Padding::from(24))
                .align_x(Alignment::Center)
            )
            .width(Length::Fixed(560.0))
            .style(|_| container::Style {
                background: Some(MayaColors::OBSIDIAN_BASE.into()),
                border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.5, radius: 8.0.into() },
                ..Default::default()
            });

            return container(column![header, container(prompt_card).center_x(Length::Fill).center_y(Length::Fill).height(Length::Fill)])
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        let mut list_col = column![].spacing(6).padding(Padding::from([6, 14]));

        if self.runs.is_empty() {
            list_col = list_col.push(
                container(
                    text("No workflow runs retrieved yet. Click 'Refresh Runs' to awaken telemetry.")
                        .size(11)
                        .color(MayaColors::PARCHMENT_MUTED)
                )
                .padding(Padding::from(20))
            );
        } else {
            for r in &self.runs {
                let (badge_text, badge_color) = match (r.status.as_str(), r.conclusion.as_deref()) {
                    ("completed", Some("success")) => ("SUCCESS", MayaColors::JADE_GLOW),
                    ("completed", Some("failure")) => ("FAILED", MayaColors::CINNABAR_BRIGHT),
                    ("completed", Some("cancelled")) => ("CANCELLED", MayaColors::GLYPH_DIM),
                    ("in_progress", _) => ("RUNNING", MayaColors::SOLAR_AMBER),
                    ("queued", _) => ("QUEUED", MayaColors::MAYA_BLUE),
                    _ => ("UNKNOWN", MayaColors::PARCHMENT_MUTED),
                };

                let short_sha = if r.head_sha.len() >= 7 { &r.head_sha[..7] } else { &r.head_sha };
                let url_clone = r.url.clone();

                let run_card = container(
                    row![
                        container(text(badge_text).size(8).color(badge_color))
                            .padding(Padding::from([2, 6]))
                            .style(move |_| container::Style {
                                background: Some(Color { a: 0.25, ..badge_color }.into()),
                                border: Border { color: badge_color, width: 0.8, radius: 3.0.into() },
                                ..Default::default()
                            }),
                        Space::new().width(10),
                        column![
                            row![
                                text(&r.name).size(12).color(MayaColors::STARLIGHT_WHITE),
                                Space::new().width(8),
                                text(format!("#{}", r.id)).size(9).color(MayaColors::GLYPH_DIM),
                            ],
                            Space::new().height(2),
                            row![
                                text(format!("Event: {}", r.event)).size(9).color(MayaColors::SOLAR_GOLD),
                                text(" · ").size(9).color(MayaColors::GLYPH_DIM),
                                text(format!("Branch: {}", r.head_branch)).size(9).color(MayaColors::JADE_GLOW),
                                text(" · ").size(9).color(MayaColors::GLYPH_DIM),
                                text(format!("SHA: {}", short_sha)).size(9).color(MayaColors::PARCHMENT_MUTED),
                                text(" · ").size(9).color(MayaColors::GLYPH_DIM),
                                text(&r.created_at).size(8).color(MayaColors::GLYPH_DIM),
                            ],
                        ]
                        .width(Length::Fill),
                        mayan_tooltip(
                            button(
                                row![
                                    text("🌐").size(10),
                                    Space::new().width(4),
                                    text("View Run").size(9).color(MayaColors::SOLAR_GOLD),
                                ]
                                .align_y(Alignment::Center)
                            )
                            .padding(Padding::from([3, 8]))
                            .style(|_, status| {
                                let border_color = match status {
                                    button::Status::Hovered => MayaColors::SOLAR_GOLD,
                                    _ => MayaColors::OBSIDIAN_BORDER,
                                };
                                button::Style {
                                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                                    border: Border { color: border_color, width: 0.8, radius: 3.0.into() },
                                    ..Default::default()
                                }
                            })
                            .on_press(WorkflowMessage::OpenRunUrl(url_clone).into()),
                            "Open this workflow run in your default web browser",
                            iced::widget::tooltip::Position::Top,
                        ),
                    ]
                    .padding(Padding::from([8, 12]))
                    .align_y(Alignment::Center)
                )
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Color { a: 0.88, ..MayaColors::OBSIDIAN_BASE }.into()),
                    border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 4.0.into() },
                    ..Default::default()
                });

                list_col = list_col.push(run_card);
            }
        }

        container(column![header, scrollable(list_col).height(Length::Fill)])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Color { a: 0.92, ..MayaColors::OBSIDIAN_BASE }.into()),
                ..Default::default()
            })
            .into()
    }
}
