//! The K'uk'ulkan Rebase & Conflict Crucible (El Crisol de Rebase y Conflictos)
//! Visual interactive rebase sequencer and 3-way merge conflict resolver.

use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RebaseAction {
    Pick,
    Reword,
    Squash,
    Drop,
}

#[derive(Debug, Clone)]
pub struct RebaseStep {
    pub sha: String,
    pub original_message: String,
    pub current_message: String,
    pub action: RebaseAction,
}

#[derive(Debug, Clone)]
pub enum RebaseMessage {
    SetAction(usize, RebaseAction),
    MoveUp(usize),
    MoveDown(usize),
    MessageChanged(usize, String),
    ExecuteRebase,
    ResolveConflictOurs,
    ResolveConflictTheirs,
    ResolveConflictBoth,
}

pub struct RebaseCrucible {
    pub steps: Vec<RebaseStep>,
    pub has_conflict: bool,
    pub conflict_file: String,
    pub conflict_ours: String,
    pub conflict_theirs: String,
}

impl Default for RebaseCrucible {
    fn default() -> Self {
        Self::new()
    }
}

impl RebaseCrucible {
    pub fn new() -> Self {
        Self {
            steps: vec![
                RebaseStep {
                    sha: "4076417".into(),
                    original_message: "feat: initial commit — architecture specification".into(),
                    current_message: "feat: initial commit — architecture specification".into(),
                    action: RebaseAction::Pick,
                },
                RebaseStep {
                    sha: "3b7f7a1".into(),
                    original_message: "feat(desktop): complete Phase 1 — Cosmic Shell".into(),
                    current_message: "feat(desktop): complete Phase 1 — Cosmic Shell".into(),
                    action: RebaseAction::Pick,
                },
                RebaseStep {
                    sha: "cbdda19".into(),
                    original_message: "feat(desktop): complete Phase 2 — Stelae Palace".into(),
                    current_message: "feat(desktop): complete Phase 2 — Stelae Palace".into(),
                    action: RebaseAction::Pick,
                },
                RebaseStep {
                    sha: "6f97399".into(),
                    original_message: "feat(desktop): complete Phase 3 — Yaxche DAG".into(),
                    current_message: "feat(desktop): complete Phase 3 — Yaxche DAG".into(),
                    action: RebaseAction::Pick,
                },
            ],
            has_conflict: false,
            conflict_file: "src/app.rs".into(),
            conflict_ours: "let background_mode = BackgroundMode::VideoLoop;".into(),
            conflict_theirs: "let background_mode = BackgroundMode::ProceduralLoom;".into(),
        }
    }

    pub fn view<'a, Message: 'a + From<RebaseMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let header = row![
            text("⚖️ K'UK'ULKAN REBASE & CONFLICT CRUCIBLE")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(12),
            text(format!("{} Commits in Sequencer", self.steps.len()))
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            button(text("🔥 EXECUTE REBASE").size(10).color(MayaColors::OBSIDIAN_VOID))
                .padding(Padding::from([3, 10]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::SOLAR_GOLD.into()),
                    border: Border {
                        color: MayaColors::SOLAR_AMBER,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(RebaseMessage::ExecuteRebase.into()),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        // Conflict section if active
        let conflict_banner = if self.has_conflict {
            self.render_conflict_resolver()
        } else {
            row![].into()
        };

        let mut step_cards = column![].spacing(8);

        for (idx, step) in self.steps.iter().enumerate() {
            step_cards = step_cards.push(self.render_step_card(idx, step));
        }

        let body = scrollable(column![conflict_banner, step_cards].padding(Padding::from([8, 12])))
            .height(Length::Fill);

        container(column![header, body])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.55, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_step_card<'a, Message: 'a + From<RebaseMessage> + Clone>(
        &'a self,
        idx: usize,
        step: &'a RebaseStep,
    ) -> Element<'a, Message> {
        let action_color = match step.action {
            RebaseAction::Pick => MayaColors::JADE_GLOW,
            RebaseAction::Reword => MayaColors::SOLAR_AMBER,
            RebaseAction::Squash => MayaColors::MAYA_BLUE,
            RebaseAction::Drop => MayaColors::CINNABAR_BRIGHT,
        };

        let action_buttons = row![
            self.action_pill(idx, "PICK", RebaseAction::Pick, step.action == RebaseAction::Pick),
            self.action_pill(idx, "REWORD", RebaseAction::Reword, step.action == RebaseAction::Reword),
            self.action_pill(idx, "SQUASH", RebaseAction::Squash, step.action == RebaseAction::Squash),
            self.action_pill(idx, "DROP", RebaseAction::Drop, step.action == RebaseAction::Drop),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let order_buttons = row![
            button(text("▲").size(10).color(MayaColors::STARLIGHT_WHITE))
                .padding(Padding::from([2, 5]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 2.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(RebaseMessage::MoveUp(idx).into()),
            button(text("▼").size(10).color(MayaColors::STARLIGHT_WHITE))
                .padding(Padding::from([2, 5]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 2.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(RebaseMessage::MoveDown(idx).into()),
        ]
        .spacing(3)
        .align_y(Alignment::Center);

        let row_content = row![
            order_buttons,
            Space::new().width(6),
            action_buttons,
            Space::new().width(8),
            text(&step.sha).size(11).color(action_color),
            Space::new().width(8),
            text_input("Commit message...", &step.current_message)
                .size(11)
                .padding(Padding::from([4, 8]))
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
                .on_input(move |val| RebaseMessage::MessageChanged(idx, val).into()),
        ]
        .padding(Padding::from([6, 10]))
        .align_y(Alignment::Center);

        container(row_content)
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.85, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn action_pill<'a, Message: 'a + From<RebaseMessage> + Clone>(
        &'a self,
        idx: usize,
        label: &'static str,
        action: RebaseAction,
        is_selected: bool,
    ) -> Element<'a, Message> {
        let (bg, text_color, border) = if is_selected {
            match action {
                RebaseAction::Pick => (MayaColors::JADE_PRIME, MayaColors::OBSIDIAN_VOID, MayaColors::JADE_GLOW),
                RebaseAction::Reword => (MayaColors::SOLAR_AMBER, MayaColors::OBSIDIAN_VOID, MayaColors::SOLAR_GOLD),
                RebaseAction::Squash => (MayaColors::MAYA_BLUE, MayaColors::OBSIDIAN_VOID, MayaColors::MAYA_BLUE),
                RebaseAction::Drop => (MayaColors::CINNABAR_BRIGHT, MayaColors::STARLIGHT_WHITE, MayaColors::CINNABAR_BRIGHT),
            }
        } else {
            (MayaColors::OBSIDIAN_BASE, MayaColors::GLYPH_DIM, MayaColors::OBSIDIAN_BORDER)
        };

        button(text(label).size(9).color(text_color))
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
            .on_press(RebaseMessage::SetAction(idx, action).into())
            .into()
    }

    fn render_conflict_resolver<'a, Message: 'a + From<RebaseMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title = row![
            text("⚔️ CONFLICT CRUCIBLE:").size(11).color(MayaColors::CINNABAR_BRIGHT),
            text(format!(" {}", self.conflict_file)).size(11).color(MayaColors::STARLIGHT_WHITE),
        ]
        .align_y(Alignment::Center);

        let ours_box = container(
            column![
                text("OURS (LOCAL HEAD)").size(10).color(MayaColors::JADE_GLOW),
                Space::new().height(4),
                text(&self.conflict_ours).size(11).color(MayaColors::STARLIGHT_WHITE),
                Space::new().height(6),
                button(text("Take Ours ✓").size(10).color(MayaColors::JADE_GLOW))
                    .padding(Padding::from([2, 6]))
                    .on_press(RebaseMessage::ResolveConflictOurs.into()),
            ]
            .padding(Padding::from([8, 10]))
        )
        .width(Length::FillPortion(1))
        .style(|_theme| container::Style {
            background: Some(Color { a: 0.35, ..MayaColors::JADE_SHADOW }.into()),
            border: Border {
                color: MayaColors::JADE_GLOW,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        });

        let theirs_box = container(
            column![
                text("THEIRS (INCOMING REBASE)").size(10).color(MayaColors::MAYA_BLUE),
                Space::new().height(4),
                text(&self.conflict_theirs).size(11).color(MayaColors::STARLIGHT_WHITE),
                Space::new().height(6),
                button(text("Take Theirs ✓").size(10).color(MayaColors::MAYA_BLUE))
                    .padding(Padding::from([2, 6]))
                    .on_press(RebaseMessage::ResolveConflictTheirs.into()),
            ]
            .padding(Padding::from([8, 10]))
        )
        .width(Length::FillPortion(1))
        .style(|_theme| container::Style {
            background: Some(Color { a: 0.35, ..MayaColors::MAYA_DEEP }.into()),
            border: Border {
                color: MayaColors::MAYA_BLUE,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        });

        let content = column![
            title,
            Space::new().height(6),
            row![ours_box, Space::new().width(10), theirs_box],
            Space::new().height(12),
        ];

        content.into()
    }
}
