//! The Chac Purifier (El Purificador de Chac — Clean Untracked & Multi-Stash Vault)
//! Untracked file purger with dry-run inspection and multi-stash vault explorer.

use iced::widget::{button, column, container, row, scrollable, text, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::git::StashEntryDetail;
use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum PurifierMessage {
    PreviewClean,
    ExecuteClean,
    PopStash(usize),
    DropStash(usize),
    SproutBranchFromStash(usize),
}

pub struct ChacPurifierWorkbench {
    pub untracked_files: Vec<String>,
    pub stashes: Vec<StashEntryDetail>,
    pub clean_feedback: Option<String>,
}

impl Default for ChacPurifierWorkbench {
    fn default() -> Self {
        Self::new()
    }
}

impl ChacPurifierWorkbench {
    pub fn new() -> Self {
        Self {
            untracked_files: Vec::new(),
            stashes: Vec::new(),
            clean_feedback: None,
        }
    }

    pub fn set_untracked(&mut self, files: Vec<String>) {
        self.untracked_files = files;
    }

    pub fn set_stashes(&mut self, stashes: Vec<StashEntryDetail>) {
        self.stashes = stashes;
    }

    pub fn view<'a, Message: 'a + From<PurifierMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_bar = row![
            text("🌧️ EL PURIFICADOR DE CHAC (CLEAN & STASH VAULT)")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(12),
            text(format!("{} Untracked Files · {} Stash Cists", self.untracked_files.len(), self.stashes.len()))
                .size(10)
                .color(MayaColors::GLYPH_DIM),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        let clean_section = self.render_clean_section();
        let stash_section = self.render_stash_section();

        let body = column![
            clean_section,
            Space::new().height(14),
            stash_section,
        ]
        .padding(Padding::from([8, 14]))
        .spacing(6);

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

    fn render_clean_section<'a, Message: 'a + From<PurifierMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title = row![
            text("🧹 UNTRACKED CLEAN PURIFIER").size(11).color(MayaColors::SOLAR_AMBER),
            Space::new().width(8),
            text("Safely sweep away untracked files & build artifacts").size(10).color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            tooltip(
                button(text("Dry Run Preview").size(10).color(MayaColors::STARLIGHT_WHITE))
                    .padding(Padding::from([3, 8]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border {
                            color: MayaColors::OBSIDIAN_BORDER,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(PurifierMessage::PreviewClean.into()),
                text("Scan and preview what files would be deleted (git clean -ndf)").size(11),
                tooltip::Position::Bottom,
            ),
            Space::new().width(6),
            tooltip(
                button(text("🔥 Purge Untracked").size(10).color(MayaColors::OBSIDIAN_VOID))
                    .padding(Padding::from([3, 10]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::CINNABAR_BRIGHT.into()),
                        border: Border {
                            color: MayaColors::CINNABAR_BRIGHT,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(PurifierMessage::ExecuteClean.into()),
                text("Permanently delete untracked files from the workspace (git clean -df)").size(11),
                tooltip::Position::Bottom,
            ),
        ]
        .align_y(Alignment::Center);

        let mut file_list = column![].spacing(2);

        if self.untracked_files.is_empty() {
            file_list = file_list.push(
                text("Sanctuary workspace is pristine — no untracked files found.").size(10).color(MayaColors::GLYPH_DIM)
            );
        } else {
            for f in self.untracked_files.iter().take(8) {
                file_list = file_list.push(
                    row![
                        text("📄 ").size(9),
                        text(f).size(10).color(MayaColors::STARLIGHT_WHITE),
                    ]
                    .align_y(Alignment::Center)
                );
            }
            if self.untracked_files.len() > 8 {
                file_list = file_list.push(
                    text(format!("... and {} more untracked files", self.untracked_files.len() - 8))
                        .size(9)
                        .color(MayaColors::GLYPH_DIM)
                );
            }
        }

        container(
            column![
                title,
                Space::new().height(8),
                file_list,
            ]
            .padding(Padding::from([10, 14]))
        )
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Color { a: 0.35, ..MayaColors::OBSIDIAN_SURFACE }.into()),
            border: Border {
                color: MayaColors::OBSIDIAN_BORDER,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn render_stash_section<'a, Message: 'a + From<PurifierMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title = row![
            text("🏺 MULTI-STASH VAULT").size(11).color(MayaColors::MAYA_BLUE),
            Space::new().width(8),
            text("Inspect and manage stashed snapshots").size(10).color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        let mut stash_rows = column![].spacing(4);

        if self.stashes.is_empty() {
            stash_rows = stash_rows.push(
                text("No stashed snapshots in the vault (git stash list is empty).").size(10).color(MayaColors::GLYPH_DIM)
            );
        } else {
            for s in &self.stashes {
                let idx = s.index;
                let row_item = row![
                    text(format!("stash@{{{}}}", s.index)).size(10).color(MayaColors::MAYA_BLUE),
                    Space::new().width(6),
                    text(&s.commit_sha).size(10).color(MayaColors::SOLAR_GOLD),
                    Space::new().width(8),
                    text(&s.message).size(11).color(MayaColors::STARLIGHT_WHITE),
                    Space::new().width(Length::Fill),
                    tooltip(
                        button(text("Sprout Branch 🌿").size(9).color(MayaColors::JADE_GLOW))
                            .padding(Padding::from([2, 6]))
                            .style(|_theme, _status| button::Style {
                                background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                                border: Border {
                                    color: MayaColors::OBSIDIAN_BORDER,
                                    width: 1.0,
                                    radius: 2.0.into(),
                                },
                                ..Default::default()
                            })
                            .on_press(PurifierMessage::SproutBranchFromStash(idx).into()),
                        text("Create a new branch from this stash (git stash branch)").size(11),
                        tooltip::Position::Left,
                    ),
                    Space::new().width(4),
                    tooltip(
                        button(text("Pop ✓").size(9).color(MayaColors::SOLAR_AMBER))
                            .padding(Padding::from([2, 6]))
                            .style(|_theme, _status| button::Style {
                                background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                                border: Border {
                                    color: MayaColors::OBSIDIAN_BORDER,
                                    width: 1.0,
                                    radius: 2.0.into(),
                                },
                                ..Default::default()
                            })
                            .on_press(PurifierMessage::PopStash(idx).into()),
                        text("Apply and remove this stash (git stash pop)").size(11),
                        tooltip::Position::Left,
                    ),
                    Space::new().width(4),
                    tooltip(
                        button(text("Drop ✕").size(9).color(MayaColors::CINNABAR_BRIGHT))
                            .padding(Padding::from([2, 6]))
                            .style(|_theme, _status| button::Style {
                                background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                                border: Border {
                                    color: MayaColors::OBSIDIAN_BORDER,
                                    width: 1.0,
                                    radius: 2.0.into(),
                                },
                                ..Default::default()
                            })
                            .on_press(PurifierMessage::DropStash(idx).into()),
                        text("Delete this stash from the vault (git stash drop)").size(11),
                        tooltip::Position::Left,
                    ),
                ]
                .align_y(Alignment::Center)
                .padding(Padding::from([4, 8]));

                stash_rows = stash_rows.push(
                    container(row_item)
                        .width(Length::Fill)
                        .style(|_theme| container::Style {
                            background: Some(Color { a: 0.20, ..MayaColors::OBSIDIAN_BASE }.into()),
                            border: Border {
                                color: MayaColors::OBSIDIAN_BORDER,
                                width: 1.0,
                                radius: 3.0.into(),
                            },
                            ..Default::default()
                        })
                );
            }
        }

        container(
            column![
                title,
                Space::new().height(8),
                scrollable(stash_rows).height(Length::Shrink),
            ]
            .padding(Padding::from([10, 14]))
        )
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Color { a: 0.35, ..MayaColors::OBSIDIAN_SURFACE }.into()),
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
