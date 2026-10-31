//! The Left Codex (El Códice)
//! Hierarchical sidebar managing real branches (Yaxche), tags (Lamat), stashes (Ak'bal), and remotes.

use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
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
}

pub struct LeftCodex {
    pub is_collapsed: bool,
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

        let branches_section = self.render_branch_section();
        let remotes_section = self.render_remote_section();
        let tags_section = self.render_tag_section();
        let stashes_section = self.render_stash_section();

        let content_list = column![
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
            .width(240)
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

    fn render_branch_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(
        &'a self,
    ) -> Element<'a, Message> {
        let section_title = row![
            text("🌳 YAXCHE (BRANCHES)")
                .size(10)
                .color(MayaColors::JADE_GLOW),
            Space::new().width(Length::Fill),
            button(text("+").size(11).color(MayaColors::JADE_GLOW))
                .padding(Padding::from([0, 4]))
                .style(|_theme, _status| button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(LeftCodexMessage::ToggleBranchInput.into()),
        ]
        .align_y(Alignment::Center);

        let mut branch_items = column![section_title].spacing(2);

        // Optional new branch prompt
        if self.is_sprouting_branch {
            let input_row = row![
                text_input("Branch name...", &self.new_branch_input)
                    .size(10)
                    .padding(Padding::from([3, 6]))
                    .on_input(|val| LeftCodexMessage::NewBranchNameChanged(val).into()),
                button(text("✓").size(10).color(MayaColors::JADE_GLOW))
                    .padding(Padding::from([2, 5]))
                    .on_press(LeftCodexMessage::ConfirmSproutBranch.into()),
            ]
            .align_y(Alignment::Center)
            .spacing(4);

            branch_items = branch_items.push(input_row);
        }

        for branch in &self.local_branches {
            let is_active = branch == &self.active_branch;
            let icon = if is_active { "👑 " } else { "🌿 " };
            let color = if is_active {
                MayaColors::SOLAR_GOLD
            } else {
                MayaColors::STARLIGHT_WHITE
            };

            let item = button(
                row![
                    text(icon).size(11),
                    text(branch).size(11).color(color),
                ]
                .align_y(Alignment::Center)
            )
            .width(Length::Fill)
            .padding(Padding::from([3, 6]))
            .style(move |_theme, status| {
                let bg = if is_active {
                    Color { a: 0.35, ..MayaColors::OBSIDIAN_SURFACE }
                } else {
                    match status {
                        button::Status::Hovered => Color { a: 0.20, ..MayaColors::OBSIDIAN_SURFACE },
                        _ => Color::TRANSPARENT,
                    }
                };
                button::Style {
                    background: Some(bg.into()),
                    border: Border {
                        color: if is_active { MayaColors::JADE_SHADOW } else { Color::TRANSPARENT },
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                }
            })
            .on_press(LeftCodexMessage::SelectBranch(branch.clone()).into());

            branch_items = branch_items.push(item);
        }

        branch_items.into()
    }

    fn render_remote_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(
        &'a self,
    ) -> Element<'a, Message> {
        let section_title = row![
            text("🌐 HORIZON (REMOTES)")
                .size(10)
                .color(MayaColors::MAYA_BLUE),
            Space::new().width(Length::Fill),
            button(text("⚡").size(10).color(MayaColors::MAYA_BLUE))
                .padding(Padding::from([0, 4]))
                .style(|_theme, _status| button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(LeftCodexMessage::FetchRemote.into()),
        ]
        .align_y(Alignment::Center);

        let mut remote_items = column![section_title].spacing(2);

        for remote in &self.remote_branches {
            let item = button(
                row![
                    text("☁️ ").size(10),
                    text(remote).size(10).color(MayaColors::PARCHMENT_MUTED),
                ]
                .align_y(Alignment::Center)
            )
            .width(Length::Fill)
            .padding(Padding::from([2, 6]))
            .style(|_theme, _status| button::Style {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            });

            remote_items = remote_items.push(item);
        }

        remote_items.into()
    }

    fn render_tag_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(
        &'a self,
    ) -> Element<'a, Message> {
        let section_title = row![
            text("🏷️ LAMAT (TAGS)")
                .size(10)
                .color(MayaColors::SOLAR_AMBER),
            Space::new().width(Length::Fill),
            button(text("+").size(10).color(MayaColors::SOLAR_AMBER))
                .padding(Padding::from([0, 4]))
                .style(|_theme, _status| button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(LeftCodexMessage::ToggleTagInput.into()),
        ]
        .align_y(Alignment::Center);

        let mut tag_items = column![section_title].spacing(2);

        if self.is_inscribing_tag {
            let input_row = row![
                text_input("Tag name...", &self.new_tag_input)
                    .size(10)
                    .padding(Padding::from([3, 6]))
                    .on_input(|val| LeftCodexMessage::NewTagNameChanged(val).into()),
                button(text("✓").size(10).color(MayaColors::SOLAR_AMBER))
                    .padding(Padding::from([2, 5]))
                    .on_press(LeftCodexMessage::ConfirmCreateTag.into()),
            ]
            .align_y(Alignment::Center)
            .spacing(4);

            tag_items = tag_items.push(input_row);
        }

        for tag in &self.tags {
            let item = button(
                row![
                    text("⭐ ").size(10),
                    text(tag).size(10).color(MayaColors::PARCHMENT_MUTED),
                ]
                .align_y(Alignment::Center)
            )
            .width(Length::Fill)
            .padding(Padding::from([2, 6]))
            .style(|_theme, _status| button::Style {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            })
            .on_press(LeftCodexMessage::SelectTag(tag.clone()).into());

            tag_items = tag_items.push(item);
        }

        tag_items.into()
    }

    fn render_stash_section<'a, Message: 'a + From<LeftCodexMessage> + Clone>(
        &'a self,
    ) -> Element<'a, Message> {
        let section_title = row![
            text("🏺 AK'B'AL (STASH CISTS)")
                .size(10)
                .color(MayaColors::CINNABAR_BRIGHT),
            Space::new().width(Length::Fill),
            button(text("+").size(11).color(MayaColors::CINNABAR_BRIGHT))
                .padding(Padding::from([0, 4]))
                .style(|_theme, _status| button::Style {
                    background: Some(Color::TRANSPARENT.into()),
                    ..Default::default()
                })
                .on_press(LeftCodexMessage::CreateStash.into()),
        ]
        .align_y(Alignment::Center);

        let mut stash_items = column![section_title].spacing(2);

        for (idx, stash) in self.stashes.iter().enumerate() {
            let item = button(
                row![
                    text("🔒 ").size(10),
                    text(stash).size(10).color(MayaColors::PARCHMENT_MUTED),
                ]
                .align_y(Alignment::Center)
            )
            .width(Length::Fill)
            .padding(Padding::from([2, 6]))
            .style(|_theme, _status| button::Style {
                background: Some(Color::TRANSPARENT.into()),
                ..Default::default()
            })
            .on_press(LeftCodexMessage::SelectStash(idx).into());

            stash_items = stash_items.push(item);
        }

        stash_items.into()
    }
}
