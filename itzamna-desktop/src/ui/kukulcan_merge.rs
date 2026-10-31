//! The K'uk'ulkan Confluence (La Confluencia de K'uk'ulkan — Branch Merge & Cherry-Pick Altar)
//! Visual branch merging with Fast-Forward, No-FF, and Squash strategies, plus Cherry-Pick altar.

use iced::widget::{button, column, container, row, text, text_input, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::git::MergeStrategy;
use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum MergeMessage {
    SelectBranch(String),
    SetStrategy(MergeStrategy),
    ExecuteMerge,
    CherryPickShaChanged(String),
    ExecuteCherryPick,
}

pub struct KukulcanMergeWorkbench {
    pub available_branches: Vec<String>,
    pub selected_target_branch: Option<String>,
    pub active_head_branch: String,
    pub strategy: MergeStrategy,
    pub cherry_pick_input: String,
    pub status_feedback: Option<(String, bool)>, // (message, is_success)
}

impl Default for KukulcanMergeWorkbench {
    fn default() -> Self {
        Self::new("main".to_string(), vec!["main".to_string()])
    }
}

impl KukulcanMergeWorkbench {
    pub fn new(active_head: String, branches: Vec<String>) -> Self {
        let first_other = branches.iter().find(|b| *b != &active_head).cloned();
        Self {
            available_branches: branches,
            selected_target_branch: first_other,
            active_head_branch: active_head,
            strategy: MergeStrategy::FastForward,
            cherry_pick_input: "".to_string(),
            status_feedback: None,
        }
    }

    pub fn set_feedback(&mut self, msg: String, is_success: bool) {
        self.status_feedback = Some((msg, is_success));
    }

    pub fn view<'a, Message: 'a + From<MergeMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_bar = row![
            text("🌿 LA CONFLUENCIA DE K'UK'ULKAN (MERGE & CHERRY-PICK)")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(12),
            text(format!("Active HEAD: '{}'", self.active_head_branch))
                .size(10)
                .color(MayaColors::JADE_GLOW),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        let feedback_view = if let Some((ref msg, is_success)) = self.status_feedback {
            let color = if is_success { MayaColors::JADE_GLOW } else { MayaColors::CINNABAR_BRIGHT };
            let bg = if is_success { MayaColors::JADE_SHADOW } else { MayaColors::CINNABAR_DARK };
            container(
                row![
                    text(if is_success { "✨ " } else { "⚠️ " }).size(11),
                    text(msg).size(11).color(color),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([6, 12]))
            .width(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(Color { a: 0.35, ..bg }.into()),
                ..Default::default()
            })
        } else {
            container(Space::new())
        };

        // 1. Branch Merge Ritual Section
        let merge_section = self.render_merge_section();

        // 2. Cherry-Pick Ritual Section
        let cherry_section = self.render_cherry_pick_section();

        let body = column![
            feedback_view,
            Space::new().height(8),
            merge_section,
            Space::new().height(14),
            cherry_section,
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

    fn render_merge_section<'a, Message: 'a + From<MergeMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let section_title = row![
            text("🏺 MERGE CONFLUENCE RITUAL").size(11).color(MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            text("Meld branch lineage into sovereign HEAD").size(10).color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        // Branch selection pills
        let mut branch_selector = row![
            text("Source Branch:").size(10).color(MayaColors::STARLIGHT_WHITE)
        ].spacing(4).align_y(Alignment::Center);

        for b in &self.available_branches {
            if b == &self.active_head_branch { continue; }
            let is_sel = self.selected_target_branch.as_ref() == Some(b);
            let b_clone = b.clone();
            let pill = button(text(b).size(10).color(if is_sel { MayaColors::OBSIDIAN_VOID } else { MayaColors::STARLIGHT_WHITE }))
                .padding(Padding::from([2, 6]))
                .style(move |_theme, _status| button::Style {
                    background: Some(if is_sel { MayaColors::JADE_GLOW.into() } else { MayaColors::OBSIDIAN_SURFACE.into() }),
                    border: Border {
                        color: if is_sel { MayaColors::JADE_GLOW } else { MayaColors::OBSIDIAN_BORDER },
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(MergeMessage::SelectBranch(b_clone).into());

            branch_selector = branch_selector.push(pill);
        }

        // Strategy Selector
        let strategy_row = row![
            text("Strategy:").size(10).color(MayaColors::STARLIGHT_WHITE),
            Space::new().width(4),
            self.strategy_button(MergeStrategy::FastForward, "FAST-FORWARD (--ff)", "Fast-forward if linear history"),
            self.strategy_button(MergeStrategy::NoFastForward, "MERGE COMMIT (--no-ff)", "Always produce explicit merge commit"),
            self.strategy_button(MergeStrategy::Squash, "SQUASH (--squash)", "Condense all incoming commits into 1 single change"),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let target_name = self.selected_target_branch.clone().unwrap_or_else(|| "None selected".to_string());
        let execute_btn = tooltip(
            button(text(format!("🔥 CONSECRATE MERGE ({})", target_name)).size(11).color(MayaColors::OBSIDIAN_VOID))
                .padding(Padding::from([6, 14]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::SOLAR_GOLD.into()),
                    border: Border {
                        color: MayaColors::SOLAR_AMBER,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(MergeMessage::ExecuteMerge.into()),
            text(format!("Merge '{}' into current branch '{}' using selected strategy", target_name, self.active_head_branch)).size(11),
            tooltip::Position::Top,
        );

        container(
            column![
                section_title,
                Space::new().height(6),
                branch_selector,
                Space::new().height(6),
                strategy_row,
                Space::new().height(10),
                execute_btn,
            ]
            .spacing(4)
            .padding(Padding::from([12, 16]))
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

    fn strategy_button<'a, Message: 'a + From<MergeMessage> + Clone>(
        &'a self,
        strat: MergeStrategy,
        label: &'static str,
        desc: &'static str,
    ) -> Element<'a, Message> {
        let is_sel = self.strategy == strat;
        let (bg, txt, border) = if is_sel {
            (MayaColors::SOLAR_AMBER, MayaColors::OBSIDIAN_VOID, MayaColors::SOLAR_GOLD)
        } else {
            (MayaColors::OBSIDIAN_SURFACE, MayaColors::GLYPH_DIM, MayaColors::OBSIDIAN_BORDER)
        };

        let btn = button(text(label).size(9).color(txt))
            .padding(Padding::from([2, 6]))
            .style(move |_theme, _status| button::Style {
                background: Some(bg.into()),
                border: Border {
                    color: border,
                    width: 1.0,
                    radius: 2.0.into(),
                },
                ..Default::default()
            })
            .on_press(MergeMessage::SetStrategy(strat).into());

        tooltip(btn, text(desc).size(11), tooltip::Position::Top).into()
    }

    fn render_cherry_pick_section<'a, Message: 'a + From<MergeMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let section_title = row![
            text("🍒 CHERRY-PICK ALTAR").size(11).color(MayaColors::CINNABAR_BRIGHT),
            Space::new().width(8),
            text("Pluck an individual commit seal into active HEAD").size(10).color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        let input_row = row![
            text_input("Enter commit SHA to cherry-pick...", &self.cherry_pick_input)
                .size(10)
                .padding(Padding::from([5, 8]))
                .width(260)
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
                .on_input(|val| MergeMessage::CherryPickShaChanged(val).into()),
            Space::new().width(8),
            tooltip(
                button(text("Pluck Commit 🍒").size(10).color(MayaColors::OBSIDIAN_VOID))
                    .padding(Padding::from([5, 12]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::CINNABAR_BRIGHT.into()),
                        border: Border {
                            color: MayaColors::CINNABAR_BRIGHT,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(MergeMessage::ExecuteCherryPick.into()),
                text("Apply changes from the given commit directly onto current HEAD (git cherry-pick)").size(11),
                tooltip::Position::Top,
            ),
        ]
        .align_y(Alignment::Center);

        container(
            column![
                section_title,
                Space::new().height(6),
                input_row,
            ]
            .spacing(4)
            .padding(Padding::from([12, 16]))
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
