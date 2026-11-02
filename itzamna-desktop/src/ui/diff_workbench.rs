//! The Carved Stone Diff Workbench (El Banco de Trabajo de Diferencias)
//! Intra-line syntax-highlighted diffs with surgical hunk-level staging and multi-file stelae.

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::git::{DiffHunk, DiffLine, DiffLineTag, FileDiff};
use crate::theme::MayaColors;
use crate::ui::tooltip::{mayan_tooltip, Position};

#[derive(Debug, Clone)]
pub struct DiffFileHeader {
    pub path: String,
    pub status: String,
    pub additions: usize,
    pub deletions: usize,
}

#[derive(Debug, Clone)]
pub enum DiffMessage {
    SelectFile(String),
    OpenInEditor(String),
    StageHunk(usize),
    UnstageHunk(usize),
    DiscardHunk(usize),
    StageEntireFile,
    CloseDiff,
}

pub struct DiffWorkbench {
    pub context_title: String,
    pub commit_sha: Option<String>,
    pub is_staged_context: bool,
    pub files: Vec<DiffFileHeader>,
    pub file_path: String,
    pub file_diff: Option<FileDiff>,
}

impl Default for DiffWorkbench {
    fn default() -> Self {
        Self::new("src/app.rs".to_string())
    }
}

impl DiffWorkbench {
    pub fn new(file_path: String) -> Self {
        let sample_old = r#"fn main() {
    println!("Hello, World!");
    let x = 10;
    println!("Value: {}", x);
}"#;
        let sample_new = r#"fn main() {
    println!("🔱 Awakening Itzamna Studio...");
    let x = 42; // Sacred Kin Number
    let y = 108; // Tzolk'in Harmony
    println!("Solar Coordinates: {}, {}", x, y);
}"#;

        let diff = crate::git::DiffEngine::compute_diff(sample_old, sample_new, &file_path);

        Self {
            context_title: "Working Tree Alterations".to_string(),
            commit_sha: None,
            is_staged_context: false,
            files: vec![DiffFileHeader {
                path: file_path.clone(),
                status: "M".to_string(),
                additions: 3,
                deletions: 1,
            }],
            file_path,
            file_diff: Some(diff),
        }
    }

    pub fn set_commit_context(
        &mut self,
        sha: String,
        context_title: String,
        files: Vec<DiffFileHeader>,
        active_file: String,
        old_text: &str,
        new_text: &str,
    ) {
        self.commit_sha = Some(sha);
        self.is_staged_context = false;
        self.context_title = context_title;
        self.files = files;
        self.file_path = active_file.clone();
        self.file_diff = Some(crate::git::DiffEngine::compute_diff(old_text, new_text, &active_file));
    }

    pub fn set_working_context(
        &mut self,
        context_title: String,
        is_staged: bool,
        files: Vec<DiffFileHeader>,
        active_file: String,
        old_text: &str,
        new_text: &str,
    ) {
        self.commit_sha = None;
        self.is_staged_context = is_staged;
        self.context_title = context_title;
        self.files = files;
        self.file_path = active_file.clone();
        self.file_diff = Some(crate::git::DiffEngine::compute_diff(old_text, new_text, &active_file));
    }

    pub fn set_active_file(&mut self, active_file: String, old_text: &str, new_text: &str) {
        self.file_path = active_file.clone();
        self.file_diff = Some(crate::git::DiffEngine::compute_diff(old_text, new_text, &active_file));
    }

    pub fn set_file(&mut self, path: String, old_text: &str, new_text: &str) {
        self.file_path = path.clone();
        self.file_diff = Some(crate::git::DiffEngine::compute_diff(old_text, new_text, &path));
        if !self.files.iter().any(|f| f.path == path) {
            self.files.push(DiffFileHeader {
                path,
                status: "M".to_string(),
                additions: 0,
                deletions: 0,
            });
        }
    }

    pub fn view<'a, Message: 'a + From<DiffMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let is_commit_view = self.commit_sha.is_some();

        // Top Context Banner
        let header = row![
            text("🔪 ETZ'NAB' STONE DIFF").size(11).color(MayaColors::SOLAR_GOLD),
            text(" • ").size(11).color(MayaColors::GLYPH_DIM),
            text(&self.context_title).size(10).color(MayaColors::STARLIGHT_WHITE),
            text(format!(" ({} altered codices)", self.files.len())).size(9).color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            mayan_tooltip(
                button(
                    row![
                        text("✕ Return to DAG").size(10).color(MayaColors::PARCHMENT_MUTED),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([3, 10]))
                .style(|_, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::SOLAR_GOLD,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border {
                            color: border_color,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    }
                })
                .on_press(DiffMessage::CloseDiff.into()),
                "Close diff view and return to the sacred commit DAG",
                Position::Bottom,
            ),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        let mut stelae_col = column![].spacing(8).padding(Padding::from([8, 12]));

        if self.files.is_empty() {
            let single = DiffFileHeader {
                path: self.file_path.clone(),
                status: "M".to_string(),
                additions: 0,
                deletions: 0,
            };
            stelae_col = stelae_col.push(self.render_expanded_stela(&single, is_commit_view));
        } else {
            for f in &self.files {
                let is_active = f.path == self.file_path;
                if is_active {
                    stelae_col = stelae_col.push(self.render_expanded_stela(f, is_commit_view));
                } else {
                    stelae_col = stelae_col.push(self.render_minimized_stela(f));
                }
            }
        }

        let body = scrollable(stelae_col)
            .height(Length::Fill)
            .width(Length::Fill);

        container(column![header, body])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.88, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_minimized_stela<'a, Message: 'a + From<DiffMessage> + Clone>(
        &'a self,
        file: &DiffFileHeader,
    ) -> Element<'a, Message> {
        let (status_color, status_bg) = match file.status.as_str() {
            "A" => (MayaColors::JADE_GLOW, Color { a: 0.2, ..MayaColors::JADE_GLOW }),
            "D" => (MayaColors::CINNABAR_BRIGHT, Color { a: 0.2, ..MayaColors::CINNABAR_BRIGHT }),
            "R" => (MayaColors::MAYA_BLUE, Color { a: 0.2, ..MayaColors::MAYA_BLUE }),
            _ => (MayaColors::SOLAR_AMBER, Color { a: 0.2, ..MayaColors::SOLAR_AMBER }),
        };

        let card_content = row![
            text("▶").size(9).color(MayaColors::GLYPH_DIM),
            Space::new().width(6),
            container(text(file.status.clone()).size(8).color(status_color))
                .padding(Padding::from([1, 4]))
                .style(move |_| container::Style {
                    background: Some(status_bg.into()),
                    border: Border { color: status_color, width: 0.5, radius: 2.0.into() },
                    ..Default::default()
                }),
            Space::new().width(8),
            text(file.path.clone()).size(10).color(MayaColors::STARLIGHT_WHITE).width(Length::Fill),
            Space::new().width(8),
            text(format!("+{}", file.additions)).size(9).color(MayaColors::JADE_GLOW),
            Space::new().width(4),
            text(format!("-{}", file.deletions)).size(9).color(MayaColors::CINNABAR_BRIGHT),
            Space::new().width(10),
            text("Click to expand diff ➔").size(9).color(MayaColors::SOLAR_GOLD),
        ]
        .padding(Padding::from([5, 10]))
        .align_y(Alignment::Center);

        let btn = button(card_content)
            .width(Length::Fill)
            .padding(0)
            .style(|_, status| {
                let (bg, border_color) = match status {
                    button::Status::Hovered => (
                        Color { a: 0.40, ..MayaColors::OBSIDIAN_SURFACE },
                        MayaColors::SOLAR_GOLD,
                    ),
                    _ => (
                        Color { a: 0.20, ..MayaColors::OBSIDIAN_SURFACE },
                        MayaColors::OBSIDIAN_BORDER,
                    ),
                };
                button::Style {
                    background: Some(bg.into()),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                }
            })
            .on_press(DiffMessage::SelectFile(file.path.clone()).into());

        mayan_tooltip(
            btn,
            "Expand this relic's diff and collapse the current active view",
            Position::Top,
        )
    }

    fn render_expanded_stela<'a, Message: 'a + From<DiffMessage> + Clone>(
        &'a self,
        file: &DiffFileHeader,
        is_commit_view: bool,
    ) -> Element<'a, Message> {
        let (status_color, status_bg) = match file.status.as_str() {
            "A" => (MayaColors::JADE_GLOW, Color { a: 0.2, ..MayaColors::JADE_GLOW }),
            "D" => (MayaColors::CINNABAR_BRIGHT, Color { a: 0.2, ..MayaColors::CINNABAR_BRIGHT }),
            "R" => (MayaColors::MAYA_BLUE, Color { a: 0.2, ..MayaColors::MAYA_BLUE }),
            _ => (MayaColors::SOLAR_AMBER, Color { a: 0.2, ..MayaColors::SOLAR_AMBER }),
        };

        let card_header = row![
            text("▼").size(9).color(MayaColors::SOLAR_GOLD),
            Space::new().width(6),
            container(text(file.status.clone()).size(8).color(status_color))
                .padding(Padding::from([1, 4]))
                .style(move |_| container::Style {
                    background: Some(status_bg.into()),
                    border: Border { color: status_color, width: 0.5, radius: 2.0.into() },
                    ..Default::default()
                }),
            Space::new().width(8),
            text(file.path.clone()).size(11).color(MayaColors::SOLAR_GOLD),
            Space::new().width(10),
            text(format!("+{}", file.additions)).size(9).color(MayaColors::JADE_GLOW),
            Space::new().width(4),
            text(format!("-{}", file.deletions)).size(9).color(MayaColors::CINNABAR_BRIGHT),
            Space::new().width(Length::Fill),
            mayan_tooltip(
                button(
                    row![
                        text("✎ Edit in K'atun").size(10).color(MayaColors::STARLIGHT_WHITE),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([2, 8]))
                .style(|_, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::JADE_GLOW,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 3.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(DiffMessage::OpenInEditor(file.path.clone()).into()),
                "Open this file in the K'atun Code Editor to make adjustments",
                Position::Top,
            ),
            if !is_commit_view {
                row![
                    Space::new().width(6),
                    mayan_tooltip(
                        button(text("Stage File +").size(10).color(MayaColors::JADE_GLOW))
                            .padding(Padding::from([2, 8]))
                            .style(|_, status| {
                                let border_color = match status {
                                    button::Status::Hovered => MayaColors::JADE_GLOW,
                                    _ => MayaColors::JADE_SHADOW,
                                };
                                button::Style {
                                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                                    border: Border { color: border_color, width: 1.0, radius: 3.0.into() },
                                    ..Default::default()
                                }
                            })
                            .on_press(DiffMessage::StageEntireFile.into()),
                        "Stage all modifications in this file into Git index (git add)",
                        Position::Top,
                    ),
                ]
            } else {
                row![]
            },
        ]
        .padding(Padding::from([6, 10]))
        .align_y(Alignment::Center);

        let mut hunk_elements = column![].spacing(8).padding(Padding::from([6, 8]));

        if let Some(ref diff) = self.file_diff {
            if diff.hunks.is_empty() {
                hunk_elements = hunk_elements.push(
                    container(text("No modifications recorded in this stela buffer.").size(10).color(MayaColors::GLYPH_DIM))
                        .padding(Padding::from(10))
                );
            } else {
                for (hunk_idx, hunk) in diff.hunks.iter().enumerate() {
                    let hunk_card = self.render_hunk(hunk_idx, hunk, is_commit_view);
                    hunk_elements = hunk_elements.push(hunk_card);
                }
            }
        } else {
            hunk_elements = hunk_elements.push(
                container(text("Carving stone diff from Git ODB...").size(10).color(MayaColors::GLYPH_DIM))
                    .padding(Padding::from(10))
            );
        }

        container(
            column![
                card_header,
                container(Space::new())
                    .height(1)
                    .width(Length::Fill)
                    .style(|_| container::Style {
                        background: Some(MayaColors::OBSIDIAN_BORDER.into()),
                        ..Default::default()
                    }),
                hunk_elements,
            ]
        )
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color { a: 0.65, ..MayaColors::OBSIDIAN_SURFACE }.into()),
            border: Border {
                color: MayaColors::SOLAR_GOLD,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn render_hunk<'a, Message: 'a + From<DiffMessage> + Clone>(
        &'a self,
        hunk_idx: usize,
        hunk: &'a DiffHunk,
        is_commit_view: bool,
    ) -> Element<'a, Message> {
        let hunk_header = row![
            text(&hunk.header).size(10).color(MayaColors::SOLAR_AMBER),
            Space::new().width(Length::Fill),
            if !is_commit_view {
                mayan_tooltip(
                    button(text("Stage Hunk +").size(10).color(MayaColors::JADE_GLOW))
                        .padding(Padding::from([1, 6]))
                        .style(|_, status| {
                            let border_color = match status {
                                button::Status::Hovered => MayaColors::JADE_GLOW,
                                _ => MayaColors::JADE_SHADOW,
                            };
                            button::Style {
                                background: Some(Color { a: 0.4, ..MayaColors::JADE_SHADOW }.into()),
                                border: Border {
                                    color: border_color,
                                    width: 1.0,
                                    radius: 3.0.into(),
                                },
                                ..Default::default()
                            }
                        })
                        .on_press(DiffMessage::StageHunk(hunk_idx).into()),
                    "Surgically stage this modification block into the index (git add -p)",
                    Position::Left,
                )
            } else {
                button(text("Signet Hunk").size(9).color(MayaColors::GLYPH_DIM))
                    .padding(Padding::from([1, 6]))
                    .style(|_, _| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        border: Border { width: 0.0, ..Default::default() },
                        ..Default::default()
                    })
                    .into()
            },
        ]
        .padding(Padding::from([4, 8]))
        .align_y(Alignment::Center);

        let mut lines_col = column![].spacing(1);

        for line in &hunk.lines {
            let line_row = self.render_line(line);
            lines_col = lines_col.push(line_row);
        }

        container(column![hunk_header, lines_col])
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.85, ..MayaColors::OBSIDIAN_VOID }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 3.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_line<'a, Message: 'a>(&'a self, line: &'a DiffLine) -> Element<'a, Message> {
        let (sign, text_color, bg_color) = match line.tag {
            DiffLineTag::Context => (" ", MayaColors::STARLIGHT_WHITE, Color::TRANSPARENT),
            DiffLineTag::Addition => ("+", MayaColors::JADE_GLOW, Color::from_rgba(0.0, 0.95, 0.60, 0.12)),
            DiffLineTag::Deletion => ("-", MayaColors::CINNABAR_BRIGHT, Color::from_rgba(1.0, 0.28, 0.35, 0.14)),
        };

        let old_num_str = line.old_lineno.map(|n| n.to_string()).unwrap_or_default();
        let new_num_str = line.new_lineno.map(|n| n.to_string()).unwrap_or_default();

        let gutter = row![
            text(format!("{:>3}", old_num_str)).size(9).color(MayaColors::GLYPH_DIM),
            text(format!("{:>3}", new_num_str)).size(9).color(MayaColors::GLYPH_DIM),
            text(sign).size(10).color(text_color),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let code_line = text(&line.content)
            .size(11)
            .color(text_color);

        let content = row![
            gutter,
            Space::new().width(8),
            code_line,
        ]
        .padding(Padding::from([1, 6]))
        .align_y(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(bg_color.into()),
                ..Default::default()
            })
            .into()
    }
}
