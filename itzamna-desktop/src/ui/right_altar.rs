//! The Right Altar (El Altar de Creación)
//! Staging workbench (Unstaged vs Staged offerings) and the Inscription Chamber (Commit).

use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;
use crate::ui::tooltip::{mayan_tooltip, Position};

#[derive(Debug, Clone)]
pub enum RightAltarMessage {
    ToggleCollapse,
    OpenFileDiff(String),
    StageFile(String),
    UnstageFile(String),
    StageAll,
    UnstageAll,
    DiscardAll,
    CommitSummaryChanged(String),
    CommitDescriptionChanged(String),
    InscribeCommit,
    ToggleAmend,
    PushAfterCommit,
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
    pub commit_description: String,
    pub is_amending: bool,
    pub last_committed_sha: Option<String>,
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
            unstaged_files: Vec::new(),
            staged_files: Vec::new(),
            commit_summary: "".to_string(),
            commit_description: "".to_string(),
            is_amending: false,
            last_committed_sha: None,
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

        let unstaged_section = self.render_unstaged_section();
        let staged_section = self.render_staged_section();
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
            mayan_tooltip(
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
                "Stage all modified and untracked files into the index (git add -A)",
                Position::Top,
            ),
        ]
        .align_y(Alignment::Center);

        let mut list = column![header].spacing(3);

        if self.unstaged_files.is_empty() {
            list = list.push(text("No unstaged modifications.").size(10).color(MayaColors::GLYPH_DIM));
        }

        for file in &self.unstaged_files {
            let status_color = match file.status {
                "M" => MayaColors::SOLAR_AMBER,
                "A" | "?" => MayaColors::JADE_GLOW,
                _ => MayaColors::CINNABAR_BRIGHT,
            };

            let path_for_diff = file.path.clone();
            let file_link = mayan_tooltip(
                button(
                    row![
                        text(file.status).size(10).color(status_color),
                        Space::new().width(6),
                        text(&file.path)
                            .size(11)
                            .color(MayaColors::STARLIGHT_WHITE),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([1, 4]))
                .style(|_theme, _status| button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(RightAltarMessage::OpenFileDiff(path_for_diff).into()),
                "Inspect file diff in Etz'nab' Stone Diff workbench (git diff)",
                iced::widget::tooltip::Position::Top,
            );

            let row_item = row![
                file_link,
                Space::new().width(Length::Fill),
                mayan_tooltip(
                    button(text("+").size(11).color(MayaColors::JADE_GLOW))
                        .padding(Padding::from([0, 5]))
                        .style(|_theme, _status| button::Style {
                            background: Some(Color::TRANSPARENT.into()),
                            ..Default::default()
                        })
                        .on_press(RightAltarMessage::StageFile(file.path.clone()).into()),
                    "Stage file into index (git add)",
                    iced::widget::tooltip::Position::Left,
                ),
            ]
            .padding(Padding::from([2, 4]))
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
            mayan_tooltip(
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
                "Unstage all staged files from index (git reset HEAD)",
                iced::widget::tooltip::Position::Top,
            ),
        ]
        .align_y(Alignment::Center);

        let mut list = column![header].spacing(3);

        if self.staged_files.is_empty() {
            list = list.push(text("No files staged for inscription.").size(10).color(MayaColors::GLYPH_DIM));
        }

        for file in &self.staged_files {
            let path_for_diff = file.path.clone();
            let file_link = mayan_tooltip(
                button(
                    row![
                        text(file.status).size(10).color(MayaColors::JADE_GLOW),
                        Space::new().width(6),
                        text(&file.path)
                            .size(11)
                            .color(MayaColors::STARLIGHT_WHITE),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([1, 4]))
                .style(|_theme, _status| button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(RightAltarMessage::OpenFileDiff(path_for_diff).into()),
                "Inspect file diff in Etz'nab' Stone Diff workbench (git diff)",
                iced::widget::tooltip::Position::Top,
            );

            let row_item = row![
                file_link,
                Space::new().width(Length::Fill),
                mayan_tooltip(
                    button(text("-").size(11).color(MayaColors::CINNABAR_BRIGHT))
                        .padding(Padding::from([0, 5]))
                        .style(|_theme, _status| button::Style {
                            background: Some(Color::TRANSPARENT.into()),
                            ..Default::default()
                        })
                        .on_press(RightAltarMessage::UnstageFile(file.path.clone()).into()),
                    "Unstage file from index (git restore --staged)",
                    iced::widget::tooltip::Position::Left,
                ),
            ]
            .padding(Padding::from([2, 4]))
            .align_y(Alignment::Center);

            list = list.push(row_item);
        }

        list.into()
    }

    fn render_commit_chamber<'a, Message: 'a + From<RightAltarMessage> + Clone>(
        &'a self,
    ) -> Element<'a, Message> {
        let chamber_title = row![
            text(if self.is_amending { "🔥 AMENDAR INSCRIPCIÓN" } else { "✒️ CÁMARA DE INSCRIPCIÓN" })
                .size(10)
                .color(if self.is_amending { MayaColors::SOLAR_AMBER } else { MayaColors::SOLAR_GOLD }),
            Space::new().width(Length::Fill),
            text(format!("K'IN {} / 72", self.commit_summary.len()))
                .size(9)
                .color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        let amend_toggle = row![
            mayan_tooltip(
                button(
                    row![
                        text(if self.is_amending { "☑" } else { "☐" }).size(11).color(if self.is_amending { MayaColors::SOLAR_AMBER } else { MayaColors::GLYPH_DIM }),
                        Space::new().width(4),
                        text("Amend Previous Commit").size(9).color(if self.is_amending { MayaColors::SOLAR_AMBER } else { MayaColors::PARCHMENT_MUTED }),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([1, 4]))
                .style(|_, _| button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(RightAltarMessage::ToggleAmend.into()),
                "Amend the previous commit message and staged tree changes (git commit --amend)",
                iced::widget::tooltip::Position::Top,
            )
        ]
        .align_y(Alignment::Center);

        let summary_input = text_input(
            if self.is_amending { "Amend commit title..." } else { "Commit title (e.g. feat: awaken new ritual)..." },
            &self.commit_summary
        )
        .size(11)
        .padding(Padding::from([7, 9]))
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
        .on_input(|val| RightAltarMessage::CommitSummaryChanged(val).into());

        let desc_input = text_input("Optional description / glyph notes...", &self.commit_description)
            .size(10)
            .padding(Padding::from([6, 9]))
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
            .on_input(|val| RightAltarMessage::CommitDescriptionChanged(val).into());

        let commit_btn_label = if self.is_amending {
            "🔥 AMEND SACRED SEAL (AMEND)"
        } else {
            "🔥 SEAL & INSCRIBE (COMMIT)"
        };

        let commit_button = mayan_tooltip(
            button(
                text(commit_btn_label).size(11).color(MayaColors::OBSIDIAN_VOID)
            )
            .width(Length::Fill)
            .padding(Padding::from([9, 14]))
            .style(move |_theme, status| {
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
            .on_press(RightAltarMessage::InscribeCommit.into()),
            if self.is_amending {
                "Amend the previous commit with current staged files and updated message (git commit --amend)"
            } else {
                "Inscribe staged files into a permanent Git commit (git commit -m)"
            },
            iced::widget::tooltip::Position::Top,
        );

        // Follow-up push banner shown after a commit is created
        let push_followup: Element<'a, Message> = if let Some(ref sha) = self.last_committed_sha {
            container(
                row![
                    text("✨ Inscribed:").size(9).color(MayaColors::JADE_GLOW),
                    Space::new().width(4),
                    text(sha).size(9).color(MayaColors::SOLAR_GOLD),
                    Space::new().width(Length::Fill),
                    mayan_tooltip(
                        button(
                            row![
                                text("🚀").size(10),
                                Space::new().width(4),
                                text("Push to Horizon").size(9).color(MayaColors::OBSIDIAN_VOID),
                            ]
                            .align_y(Alignment::Center)
                        )
                        .padding(Padding::from([3, 8]))
                        .style(|_, status| {
                            let bg = match status {
                                button::Status::Hovered => MayaColors::SOLAR_AMBER,
                                _ => MayaColors::SOLAR_GOLD,
                            };
                            button::Style {
                                background: Some(bg.into()),
                                border: Border { color: MayaColors::SOLAR_AMBER, width: 1.0, radius: 3.0.into() },
                                ..Default::default()
                            }
                        })
                        .on_press(RightAltarMessage::PushAfterCommit.into()),
                        "Push your freshly inscribed commit to remote tracking branch (git push origin)",
                        iced::widget::tooltip::Position::Top,
                    ),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([4, 6]))
            .style(|_| container::Style {
                background: Some(Color { a: 0.25, ..MayaColors::JADE_SHADOW }.into()),
                border: Border { color: MayaColors::JADE_GLOW, width: 0.8, radius: 3.0.into() },
                ..Default::default()
            })
            .into()
        } else {
            Space::new().height(0).into()
        };

        container(
            column![
                chamber_title,
                Space::new().height(3),
                amend_toggle,
                Space::new().height(4),
                summary_input,
                Space::new().height(4),
                desc_input,
                Space::new().height(6),
                commit_button,
                Space::new().height(4),
                push_followup,
            ]
            .spacing(2)
            .padding(Padding::from([8, 12]))
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
