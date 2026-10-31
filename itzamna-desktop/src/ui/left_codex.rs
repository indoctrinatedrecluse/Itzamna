//! The Left Codex (El Códice)
//! Hierarchical sidebar managing sanctuaries (repos), branches (Yaxche), tags (Lamat), stashes (Ak'bal), and remotes.

use iced::widget::{button, column, container, row, scrollable, text, text_input, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum LeftCodexMessage {
    ToggleCollapse,
    SelectBranch(String),
    SelectTag(String),
    SelectStash(usize),
    ToggleBranchInput,
    NewBranchNameChanged(String),
    ConfirmSproutBranch,
    ToggleTagInput,
    NewTagNameChanged(String),
    ConfirmCreateTag,
    FetchRemote,
    CreateStash,

    // Multi-Repo and Genesis Rituals
    ToggleRepoDropdown,
    SelectRepo(String),
    OpenCloneModal,
    CloseCloneModal,
    CloneUrlChanged(String),
    CloneDirChanged(String),
    ConfirmClone,
    OpenInitModal,
    CloseInitModal,
    InitDirChanged(String),
    ConfirmInit,
}

pub struct LeftCodex {
    pub is_collapsed: bool,
    pub current_repo_name: String,
    pub current_repo_path: String,
    pub recent_repos: Vec<String>,
    pub is_repo_dropdown_open: bool,
    pub is_clone_modal_open: bool,
    pub clone_url_input: String,
    pub clone_dir_input: String,
    pub is_init_modal_open: bool,
    pub init_dir_input: String,

    pub active_branch: String,
    pub local_branches: Vec<String>,
    pub remote_branches: Vec<String>,
    pub tags: Vec<String>,
    pub stashes: Vec<String>,
    pub is_sprouting_branch: bool,
    pub new_branch_input: String,
    pub is_inscribing_tag: bool,
    pub new_tag_input: String,
}

impl Default for LeftCodex {
    fn default() -> Self {
        Self::new()
    }
}

impl LeftCodex {
    pub fn new() -> Self {
        Self {
            is_collapsed: false,
            current_repo_name: "Itzamna".to_string(),
            current_repo_path: ".".to_string(),
            recent_repos: vec![".".to_string()],
            is_repo_dropdown_open: false,
            is_clone_modal_open: false,
            clone_url_input: "".to_string(),
            clone_dir_input: "".to_string(),
            is_init_modal_open: false,
            init_dir_input: "".to_string(),

            active_branch: "main".to_string(),
            local_branches: vec!["main".to_string()],
            remote_branches: vec!["origin/main".to_string()],
            tags: Vec::new(),
            stashes: Vec::new(),
            is_sprouting_branch: false,
            new_branch_input: "".to_string(),
            is_inscribing_tag: false,
            new_tag_input: "".to_string(),
        }
    }

    pub fn view<'a, Message: 'a + From<LeftCodexMessage> + Clone>(&'a self) -> Element<'a, Message> {
        if self.is_collapsed {
            return container(
                button(text("▶").size(14).color(MayaColors::SOLAR_GOLD))
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
                    .on_press(LeftCodexMessage::ToggleCollapse.into()),
            )
            .height(Length::Fill)
            .into();
        }

        let header = row![
            text("📜 EL CÓDICE")
                .size(13)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(Length::Fill),
            button(text("◀").size(11).color(MayaColors::GLYPH_DIM))
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
                .on_press(LeftCodexMessage::ToggleCollapse.into()),
        ]
        .padding(Padding::from([8, 12]))
        .align_y(Alignment::Center);

        let repo_section = self.render_repo_section();
        let branches_section = self.render_branch_section();
        let remotes_section = self.render_remote_section();
        let tags_section = self.render_tag_section();
        let stashes_section = self.render_stash_section();

        let content_list = column![
            repo_section,
            Space::new().height(8),
            branches_section,
            Space::new().height(12),
            remotes_section,
            Space::new().height(12),
            tags_section,
            Space::new().height(12),
            stashes_section,
        ]
        .padding(Padding::from([4, 10]))
        .spacing(4);

        let body = scrollable(content_list).height(Length::Fill);

        container(column![header, body])
            .width(250)
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

    fn render_repo_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let repo_pill = button(
            row![
                text("🏛️").size(11),
                Space::new().width(6),
                text(&self.current_repo_name).size(11).color(MayaColors::JADE_GLOW),
                Space::new().width(Length::Fill),
                text(if self.is_repo_dropdown_open { "▲" } else { "▼" }).size(9).color(MayaColors::GLYPH_DIM),
            ]
            .align_y(Alignment::Center)
        )
        .padding(Padding::from([4, 8]))
        .width(Length::Fill)
        .style(|_theme, _status| button::Style {
            background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
            border: Border {
                color: MayaColors::OBSIDIAN_BORDER,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .on_press(LeftCodexMessage::ToggleRepoDropdown.into());

        let actions = row![
            tooltip(
                button(text("🌐 Clone").size(9).color(MayaColors::MAYA_BLUE))
                    .padding(Padding::from([2, 6]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_BASE.into()),
                        border: Border {
                            color: MayaColors::MAYA_BLUE,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(LeftCodexMessage::OpenCloneModal.into()),
                "Summon and clone an external repository via Git URL",
                tooltip::Position::Bottom,
            ),
            Space::new().width(6),
            tooltip(
                button(text("✨ Init").size(9).color(MayaColors::SOLAR_GOLD))
                    .padding(Padding::from([2, 6]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_BASE.into()),
                        border: Border {
                            color: MayaColors::SOLAR_GOLD,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(LeftCodexMessage::OpenInitModal.into()),
                "Sanctify a new local directory as a pristine Git sanctuary",
                tooltip::Position::Bottom,
            ),
        ]
        .align_y(Alignment::Center);

        let mut col = column![row![repo_pill].align_y(Alignment::Center), Space::new().height(4), actions];

        // Dropdown list
        if self.is_repo_dropdown_open {
            let mut list = column![].spacing(2);
            for path in &self.recent_repos {
                let name = std::path::Path::new(path)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| path.clone());

                let is_active = path == &self.current_repo_path;
                let color = if is_active { MayaColors::JADE_GLOW } else { MayaColors::STARLIGHT_WHITE };

                list = list.push(
                    button(
                        row![
                            text(if is_active { "●" } else { "○" }).size(8).color(color),
                            Space::new().width(6),
                            column![
                                text(name).size(10).color(color),
                                text(path).size(8).color(MayaColors::GLYPH_DIM),
                            ]
                        ]
                        .align_y(Alignment::Center)
                    )
                    .width(Length::Fill)
                    .padding(Padding::from([3, 6]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_BASE.into()),
                        border: Border {
                            color: MayaColors::OBSIDIAN_BORDER,
                            width: 1.0,
                            radius: 2.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(LeftCodexMessage::SelectRepo(path.clone()).into())
                );
            }

            col = col.push(
                container(list)
                    .padding(Padding::from([4, 6]))
                    .style(|_| container::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border {
                            color: MayaColors::JADE_GLOW,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
            );
        }

        // Clone Modal
        if self.is_clone_modal_open {
            let clone_box = column![
                text("🌐 THE CLONE CHAMBER").size(10).color(MayaColors::MAYA_BLUE),
                Space::new().height(4),
                text_input("Remote URL (https://...)", &self.clone_url_input)
                    .on_input(|s| LeftCodexMessage::CloneUrlChanged(s).into())
                    .size(10)
                    .padding(4),
                Space::new().height(4),
                text_input("Target directory path", &self.clone_dir_input)
                    .on_input(|s| LeftCodexMessage::CloneDirChanged(s).into())
                    .size(10)
                    .padding(4),
                Space::new().height(6),
                row![
                    button(text("Transmute & Clone").size(9).color(MayaColors::OBSIDIAN_VOID))
                        .padding(Padding::from([3, 8]))
                        .style(|_, _| button::Style {
                            background: Some(MayaColors::MAYA_BLUE.into()),
                            border: Border { radius: 3.0.into(), ..Default::default() },
                            ..Default::default()
                        })
                        .on_press(LeftCodexMessage::ConfirmClone.into()),
                    Space::new().width(6),
                    button(text("Cancel").size(9).color(MayaColors::GLYPH_DIM))
                        .padding(Padding::from([3, 6]))
                        .on_press(LeftCodexMessage::CloseCloneModal.into()),
                ]
            ]
            .padding(Padding::from([6, 8]));

            col = col.push(
                container(clone_box)
                    .style(|_| container::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: MayaColors::MAYA_BLUE, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    })
            );
        }

        // Init Modal
        if self.is_init_modal_open {
            let init_box = column![
                text("✨ THE GENESIS ALTAR").size(10).color(MayaColors::SOLAR_GOLD),
                Space::new().height(4),
                text_input("Local directory to initialize", &self.init_dir_input)
                    .on_input(|s| LeftCodexMessage::InitDirChanged(s).into())
                    .size(10)
                    .padding(4),
                Space::new().height(6),
                row![
                    button(text("Sanctify Genesis").size(9).color(MayaColors::OBSIDIAN_VOID))
                        .padding(Padding::from([3, 8]))
                        .style(|_, _| button::Style {
                            background: Some(MayaColors::SOLAR_GOLD.into()),
                            border: Border { radius: 3.0.into(), ..Default::default() },
                            ..Default::default()
                        })
                        .on_press(LeftCodexMessage::ConfirmInit.into()),
                    Space::new().width(6),
                    button(text("Cancel").size(9).color(MayaColors::GLYPH_DIM))
                        .padding(Padding::from([3, 6]))
                        .on_press(LeftCodexMessage::CloseInitModal.into()),
                ]
            ]
            .padding(Padding::from([6, 8]));

            col = col.push(
                container(init_box)
                    .style(|_| container::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: MayaColors::SOLAR_GOLD, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    })
            );
        }

        col.into()
    }

    fn render_branch_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_row = row![
            text("🌿 YAXCHÉ (BRANCHES)").size(11).color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            tooltip(
                button(text(if self.is_sprouting_branch { "✖" } else { "＋" }).size(11).color(MayaColors::JADE_GLOW))
                    .padding(Padding::from([0, 4]))
                    .style(|_theme, _status| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        ..Default::default()
                    })
                    .on_press(LeftCodexMessage::ToggleBranchInput.into()),
                "Sprout a new branch from current HEAD (git branch)",
                tooltip::Position::Top,
            ),
        ]
        .align_y(Alignment::Center);

        let mut branch_list = column![title_row].spacing(2);

        if self.is_sprouting_branch {
            let sprout_input = row![
                text_input("New branch name...", &self.new_branch_input)
                    .on_input(|s| LeftCodexMessage::NewBranchNameChanged(s).into())
                    .size(10)
                    .padding(Padding::from([2, 4])),
                button(text("Sprout").size(9).color(MayaColors::JADE_GLOW))
                    .padding(Padding::from([2, 6]))
                    .on_press(LeftCodexMessage::ConfirmSproutBranch.into()),
            ]
            .spacing(4)
            .align_y(Alignment::Center);

            branch_list = branch_list.push(sprout_input);
        }

        for branch in &self.local_branches {
            let is_active = branch == &self.active_branch;
            let dot_color = if is_active { MayaColors::JADE_GLOW } else { MayaColors::OBSIDIAN_BORDER };
            let text_color = if is_active { MayaColors::JADE_GLOW } else { MayaColors::STARLIGHT_WHITE };

            let branch_row = row![
                text("•").size(14).color(dot_color),
                Space::new().width(6),
                text(branch).size(11).color(text_color),
            ]
            .align_y(Alignment::Center);

            let btn = button(branch_row)
                .padding(Padding::from([2, 4]))
                .width(Length::Fill)
                .style(move |_theme, status| {
                    let bg = match status {
                        button::Status::Hovered => Some(Color { a: 0.15, ..MayaColors::JADE_GLOW }.into()),
                        _ => None,
                    };
                    button::Style {
                        background: bg,
                        border: Border { radius: 2.0.into(), ..Default::default() },
                        ..Default::default()
                    }
                })
                .on_press(LeftCodexMessage::SelectBranch(branch.clone()).into());

            branch_list = branch_list.push(tooltip(
                btn,
                text(format!("Switch active branch to '{}' (git checkout)", branch)).size(9),
                tooltip::Position::Right,
            ));
        }

        branch_list.into()
    }

    fn render_remote_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_row = row![
            text("🌐 REMOTES (PORTALS)").size(11).color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            tooltip(
                button(text("⟳").size(11).color(MayaColors::MAYA_BLUE))
                    .padding(Padding::from([0, 4]))
                    .style(|_theme, _status| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        ..Default::default()
                    })
                    .on_press(LeftCodexMessage::FetchRemote.into()),
                "Synchronize references from all configured remotes (git fetch)",
                tooltip::Position::Top,
            ),
        ]
        .align_y(Alignment::Center);

        let mut remote_list = column![title_row].spacing(2);

        for remote in &self.remote_branches {
            let remote_row = row![
                text("○").size(10).color(MayaColors::MAYA_BLUE),
                Space::new().width(6),
                text(remote).size(10).color(MayaColors::PARCHMENT_MUTED),
            ]
            .align_y(Alignment::Center);

            let btn = button(remote_row)
                .padding(Padding::from([2, 4]))
                .width(Length::Fill)
                .style(|_theme, status| {
                    let bg = match status {
                        button::Status::Hovered => Some(Color { a: 0.15, ..MayaColors::MAYA_BLUE }.into()),
                        _ => None,
                    };
                    button::Style {
                        background: bg,
                        border: Border { radius: 2.0.into(), ..Default::default() },
                        ..Default::default()
                    }
                });

            remote_list = remote_list.push(tooltip(
                btn,
                text(format!("Remote tracking branch: {}", remote)).size(9),
                tooltip::Position::Right,
            ));
        }

        remote_list.into()
    }

    fn render_tag_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_row = row![
            text("🏷️ LAMAT (TAGS)").size(11).color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            tooltip(
                button(text(if self.is_inscribing_tag { "✖" } else { "＋" }).size(11).color(MayaColors::SOLAR_GOLD))
                    .padding(Padding::from([0, 4]))
                    .style(|_theme, _status| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        ..Default::default()
                    })
                    .on_press(LeftCodexMessage::ToggleTagInput.into()),
                "Inscribe a new tag seal on current HEAD (git tag)",
                tooltip::Position::Top,
            ),
        ]
        .align_y(Alignment::Center);

        let mut tag_list = column![title_row].spacing(2);

        if self.is_inscribing_tag {
            let tag_input = row![
                text_input("New tag name (e.g. v1.0)...", &self.new_tag_input)
                    .on_input(|s| LeftCodexMessage::NewTagNameChanged(s).into())
                    .size(10)
                    .padding(Padding::from([2, 4])),
                button(text("Inscribe").size(9).color(MayaColors::SOLAR_GOLD))
                    .padding(Padding::from([2, 6]))
                    .on_press(LeftCodexMessage::ConfirmCreateTag.into()),
            ]
            .spacing(4)
            .align_y(Alignment::Center);

            tag_list = tag_list.push(tag_input);
        }

        if self.tags.is_empty() {
            tag_list = tag_list.push(text("No tags inscribed").size(9).color(MayaColors::GLYPH_DIM));
        } else {
            for tag in &self.tags {
                let tag_row = row![
                    text("◇").size(10).color(MayaColors::SOLAR_GOLD),
                    Space::new().width(6),
                    text(tag).size(10).color(MayaColors::SOLAR_GOLD),
                ]
                .align_y(Alignment::Center);

                let btn = button(tag_row)
                    .padding(Padding::from([2, 4]))
                    .width(Length::Fill)
                    .style(|_theme, status| {
                        let bg = match status {
                            button::Status::Hovered => Some(Color { a: 0.15, ..MayaColors::SOLAR_GOLD }.into()),
                            _ => None,
                        };
                        button::Style {
                            background: bg,
                            border: Border { radius: 2.0.into(), ..Default::default() },
                            ..Default::default()
                        }
                    })
                    .on_press(LeftCodexMessage::SelectTag(tag.clone()).into());

                tag_list = tag_list.push(tooltip(
                    btn,
                    text(format!("View tag '{}' in history", tag)).size(9),
                    tooltip::Position::Right,
                ));
            }
        }

        tag_list.into()
    }

    fn render_stash_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_row = row![
            text("📦 AK'BAL (STASHES)").size(11).color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            tooltip(
                button(text("＋").size(11).color(MayaColors::SOLAR_AMBER))
                    .padding(Padding::from([0, 4]))
                    .style(|_theme, _status| button::Style {
                        background: Some(Color::TRANSPARENT.into()),
                        ..Default::default()
                    })
                    .on_press(LeftCodexMessage::CreateStash.into()),
                "Stash dirty working tree into dark void of Ak'bal (git stash push)",
                tooltip::Position::Top,
            ),
        ]
        .align_y(Alignment::Center);

        let mut stash_list = column![title_row].spacing(2);

        if self.stashes.is_empty() {
            stash_list = stash_list.push(text("Sanctuary vault is empty").size(9).color(MayaColors::GLYPH_DIM));
        } else {
            for (idx, stash) in self.stashes.iter().enumerate() {
                let stash_row = row![
                    text("■").size(9).color(MayaColors::SOLAR_AMBER),
                    Space::new().width(6),
                    text(stash).size(9).color(MayaColors::PARCHMENT_MUTED),
                ]
                .align_y(Alignment::Center);

                let btn = button(stash_row)
                    .padding(Padding::from([2, 4]))
                    .width(Length::Fill)
                    .style(|_theme, status| {
                        let bg = match status {
                            button::Status::Hovered => Some(Color { a: 0.15, ..MayaColors::SOLAR_AMBER }.into()),
                            _ => None,
                        };
                        button::Style {
                            background: bg,
                            border: Border { radius: 2.0.into(), ..Default::default() },
                            ..Default::default()
                        }
                    })
                    .on_press(LeftCodexMessage::SelectStash(idx).into());

                stash_list = stash_list.push(tooltip(
                    btn,
                    text(format!("Inspect and pop stash entry @{{{}}} (git stash pop)", idx)).size(9),
                    tooltip::Position::Right,
                ));
            }
        }

        stash_list.into()
    }
}
