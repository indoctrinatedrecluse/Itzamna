//! The Right Altar (El Altar de Creación)
//! Staging workbench (Unstaged vs Staged offerings) and the Inscription Chamber (Commit).

use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum RightAltarMessage {
    ToggleCollapse,
    StageFile(String),
    UnstageFile(String),
    StageAll,
    UnstageAll,
    DiscardAll,
    CommitSummaryChanged(String),
    InscribeCommit,
}

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub status: &'static str,
}

pub struct RightAltar {
    pub is_collapsed: bool,
    pub unstaged_files: Vec<FileEntry>,
    pub staged_files: Vec<FileEntry>,
    pub commit_summary: String,
}

impl Default for RightAltar {
    fn default() -> Self {
        Self::new()
    }
}

impl RightAltar {
    pub fn new() -> Self {
        Self {
            is_collapsed: false,
            unstaged_files: vec![
                FileEntry { path: "src/canvas/background_mode.rs".into(), status: "M" },
                FileEntry { path: "src/ui/left_codex.rs".into(), status: "A" },
                FileEntry { path: "src/ui/right_altar.rs".into(), status: "A" },
                FileEntry { path: "assets/background.png".into(), status: "?" },
            ],
            staged_files: vec![
                FileEntry { path: "Cargo.toml".into(), status: "M" },
                FileEntry { path: "README.md".into(), status: "M" },
            ],
            commit_summary: "feat(stelae): awaken four sacred quadrant panels".to_string(),
        }
    }

    pub fn view<'a, Message: 'a + From<RightAltarMessage> + Clone>(&'a self) -> Element<'a, Message> {
        if self.is_collapsed {
            return container(
                button(text("◀").size(14).color(MayaColors::SOLAR_GOLD))
                    .padding(Padding::from([12, 6]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_BASE.into()),
                        border: Border {
                            color: MayaColors::OBSIDIAN_BORDER,
                            width: 1.0,
                            radius: 0.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(RightAltarMessage::ToggleCollapse.into()),
            )
            .height(Length::Fill)
            .into();
        }

        // Stele Header
        let header = row![
            button(text("▶").size(11).color(MayaColors::GLYPH_DIM))
                .padding(Padding::from([2, 6]))
                .style(|_theme, status| {
                    let text_color = match status {
                        button::Status::Hovered => MayaColors::SOLAR_GOLD,
                        _ => MayaColors::GLYPH_DIM,
                    };
                    button::Style {
                        text_color,
                        background: Some(Color::TRANSPARENT.into()),
                        ..Default::default()
                    }
                })
                .on_press(RightAltarMessage::ToggleCollapse.into()),
            Space::new().width(4),
            text("🏺 EL ALTAR DE CREACIÓN")
                .size(12)
                .color(MayaColors::SOLAR_GOLD),
        ]
        .padding(Padding::from([8, 12]))
        .align_y(Alignment::Center);

        // 1. Unstaged Offerings Section
        let unstaged_section = self.render_unstaged_section();

        // 2. Staged Sacrifices Section
        let staged_section = self.render_staged_section();

        // 3. Inscription Chamber (Commit)
        let commit_chamber = self.render_commit_chamber();

        let files_scroll = scrollable(
            column![
                unstaged_section,
                Space::new().height(14),
                staged_section,
            ]
            .padding(Padding::from([4, 10]))
            .spacing(4)
        )
        .height(Length::Fill);

        container(column![header, files_scroll, commit_chamber])
            .width(320)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.90, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_unstaged_section<'a, Message: 'a + From<RightAltarMessage> + Clone>(
        &'a self,
    ) -> Element<'a, Message> {
        let count_badge = format!(" ({})", self.unstaged_files.len());
        let header = row![
            text("OFRENDAS (UNSTAGED)")
                .size(10)
                .color(MayaColors::SOLAR_AMBER),
            text(count_badge)
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            button(text("Stage All +").size(10).color(MayaColors::JADE_GLOW))
                .padding(Padding::from([1, 6]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::JADE_SHADOW,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(RightAltarMessage::StageAll.into()),
        ]
        .align_y(Alignment::Center);

        let mut list = column![header].spacing(3);

        for file in &self.unstaged_files {
            let status_color = match file.status {
                "M" => MayaColors::SOLAR_AMBER,
                "A" | "?" => MayaColors::JADE_GLOW,
                _ => MayaColors::CINNABAR_BRIGHT,
            };

            let row_item = row![
                text(file.status).size(10).color(status_color),
                Space::new().width(6),
                text(&file.path)
                    .size(11)
                    .color(MayaColors::STARLIGHT_WHITE),
                Space::new().width(Length::Fill),
                button(text("+").size(11).color(MayaColors::JADE_GLOW))
                    .padding(Padding::from([0, 5]))
                    .style(|_theme, _status| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        ..Default::default()
                    })
                    .on_press(RightAltarMessage::StageFile(file.path.clone()).into()),
            ]
            .padding(Padding::from([2, 6]))
            .align_y(Alignment::Center);

            list = list.push(row_item);
        }

        list.into()
    }

    fn render_staged_section<'a, Message: 'a + From<RightAltarMessage> + Clone>(
        &'a self,
    ) -> Element<'a, Message> {
        let count_badge = format!(" ({})", self.staged_files.len());
        let header = row![
            text("CONSAGRADOS (STAGED)")
                .size(10)
                .color(MayaColors::JADE_GLOW),
            text(count_badge)
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            button(text("Unstage All -").size(10).color(MayaColors::CINNABAR_BRIGHT))
                .padding(Padding::from([1, 6]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(RightAltarMessage::UnstageAll.into()),
        ]
        .align_y(Alignment::Center);

        let mut list = column![header].spacing(3);

        for file in &self.staged_files {
            let row_item = row![
                text(file.status).size(10).color(MayaColors::JADE_GLOW),
                Space::new().width(6),
                text(&file.path)
                    .size(11)
                    .color(MayaColors::STARLIGHT_WHITE),
                Space::new().width(Length::Fill),
                button(text("-").size(11).color(MayaColors::CINNABAR_BRIGHT))
                    .padding(Padding::from([0, 5]))
                    .style(|_theme, _status| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        ..Default::default()
                    })
                    .on_press(RightAltarMessage::UnstageFile(file.path.clone()).into()),
            ]
            .padding(Padding::from([2, 6]))
            .align_y(Alignment::Center);

            list = list.push(row_item);
        }

        list.into()
    }

    fn render_commit_chamber<'a, Message: 'a + From<RightAltarMessage> + Clone>(
        &'a self,
    ) -> Element<'a, Message> {
        let chamber_title = row![
            text("✒️ CÁMARA DE INSCRIPCIÓN")
                .size(10)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(Length::Fill),
            text(format!("K'IN {} / 72", self.commit_summary.len()))
                .size(9)
                .color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        let input_field = text_input("Describe the sacred deed in stone...", &self.commit_summary)
            .size(11)
            .padding(Padding::from([8, 10]))
            .style(|_theme, _status| text_input::Style {
                background: MayaColors::OBSIDIAN_SURFACE.into(),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                icon: MayaColors::PARCHMENT_MUTED,
                placeholder: MayaColors::GLYPH_DIM,
                value: MayaColors::STARLIGHT_WHITE,
                selection: MayaColors::JADE_SHADOW,
            })
            .on_input(|val| RightAltarMessage::CommitSummaryChanged(val).into());

        let commit_button = button(
            row![
                text("🔥 SEAL & INSCRIBE").size(11).color(MayaColors::OBSIDIAN_VOID),
                text(" (COMMIT)").size(9).color(Color { a: 0.8, ..MayaColors::OBSIDIAN_VOID }),
            ]
            .align_y(Alignment::Center)
        )
        .width(Length::Fill)
        .padding(Padding::from([9, 14]))
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered => MayaColors::SOLAR_GOLD,
                _ => MayaColors::SOLAR_AMBER,
            };
            button::Style {
                background: Some(bg.into()),
                border: Border {
                    color: MayaColors::SOLAR_GOLD,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .on_press(RightAltarMessage::InscribeCommit.into());

        container(
            column![
                chamber_title,
                Space::new().height(6),
                input_field,
                Space::new().height(8),
                commit_button,
            ]
            .spacing(2)
            .padding(Padding::from([10, 12]))
        )
        .style(|_theme| container::Style {
            background: Some(MayaColors::OBSIDIAN_BASE.into()),
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
