//! The K'uk'ulkan Rebase & Conflict Crucible (El Crisol de Rebase y Conflictos)
//! Visual interactive rebase sequencer and 3-way merge conflict resolver.

use iced::widget::{button, column, container, row, scrollable, text, text_input, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

pub use crate::git::RebaseAction;
use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub struct RebaseStep {
    pub sha: String,
    pub full_sha: String,
    pub original_message: String,
    pub current_message: String,
    pub action: RebaseAction,
}

#[derive(Debug, Clone)]
pub enum RebaseMessage {
    LoadRebaseSteps,
    SetAction(usize, RebaseAction),
    MoveUp(usize),
    MoveDown(usize),
    MessageChanged(usize, String),
    ExecuteRebase,
    SelectConflictFile(String),
    ResolveConflictOurs,
    ResolveConflictTheirs,
    ResolveConflictBoth,
    ContinueRebase,
    AbortRebase,
}

pub struct RebaseCrucible {
    pub steps: Vec<RebaseStep>,
    pub is_rebasing: bool,
    pub current_step: usize,
    pub has_conflict: bool,
    pub conflicted_files: Vec<String>,
    pub conflict_file: String,
    pub conflict_ours: String,
    pub conflict_theirs: String,
    pub rebase_feedback: String,
}

impl Default for RebaseCrucible {
    fn default() -> Self {
        Self::new()
    }
}

impl RebaseCrucible {
    pub fn new() -> Self {
        Self {
            steps: Vec::new(),
            is_rebasing: false,
            current_step: 0,
            has_conflict: false,
            conflicted_files: Vec::new(),
            conflict_file: String::new(),
            conflict_ours: String::new(),
            conflict_theirs: String::new(),
            rebase_feedback: String::new(),
        }
    }

    pub fn load_steps(&mut self, steps: Vec<RebaseStep>) {
        self.steps = steps;
        self.is_rebasing = false;
        self.has_conflict = false;
        self.conflicted_files.clear();
        self.conflict_file.clear();
        self.conflict_ours.clear();
        self.conflict_theirs.clear();
        self.rebase_feedback = format!("Loaded {} commits into rebase sequencer.", self.steps.len());
    }

    pub fn set_conflict(
        &mut self,
        file: String,
        ours: String,
        theirs: String,
        all_conflicted: Vec<String>,
        step_index: usize,
        message: String,
    ) {
        self.is_rebasing = true;
        self.current_step = step_index;
        self.has_conflict = true;
        self.conflicted_files = all_conflicted;
        self.conflict_file = file;
        self.conflict_ours = ours;
        self.conflict_theirs = theirs;
        self.rebase_feedback = message;
    }

    pub fn clear_conflict(&mut self) {
        self.has_conflict = false;
        self.conflict_file.clear();
        self.conflict_ours.clear();
        self.conflict_theirs.clear();
        self.conflicted_files.clear();
    }

    pub fn reset_rebase(&mut self) {
        self.is_rebasing = false;
        self.current_step = 0;
        self.has_conflict = false;
        self.conflicted_files.clear();
        self.conflict_file.clear();
        self.conflict_ours.clear();
        self.conflict_theirs.clear();
    }

    pub fn set_feedback(&mut self, msg: String) {
        self.rebase_feedback = msg;
    }

    pub fn view<'a, Message: 'a + From<RebaseMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let mut header_actions = row![].spacing(6).align_y(Alignment::Center);

        // Load Commits Button
        header_actions = header_actions.push(tooltip(
            button(text("⚡ LOAD COMMITS").size(10).color(MayaColors::STARLIGHT_WHITE))
                .padding(Padding::from([3, 10]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::MAYA_BLUE,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(RebaseMessage::LoadRebaseSteps.into()),
            text("Populate unpushed commits from HEAD vs upstream into rebase sequencer").size(11),
            tooltip::Position::Bottom,
        ));

        if self.is_rebasing {
            // Continue Button (enabled if no active conflict)
            if !self.has_conflict {
                header_actions = header_actions.push(tooltip(
                    button(text("▶ CONTINUE REBASE").size(10).color(MayaColors::OBSIDIAN_VOID))
                        .padding(Padding::from([3, 10]))
                        .style(|_theme, _status| button::Style {
                            background: Some(MayaColors::JADE_GLOW.into()),
                            border: Border {
                                color: MayaColors::JADE_PRIME,
                                width: 1.0,
                                radius: 3.0.into(),
                            },
                            ..Default::default()
                        })
                        .on_press(RebaseMessage::ContinueRebase.into()),
                    text("Continue cherry-picking remaining rebase steps after resolving conflict").size(11),
                    tooltip::Position::Bottom,
                ));
            }

            // Abort Button
            header_actions = header_actions.push(tooltip(
                button(text("✕ ABORT REBASE").size(10).color(MayaColors::STARLIGHT_WHITE))
                    .padding(Padding::from([3, 10]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::CINNABAR_DARK.into()),
                        border: Border {
                            color: MayaColors::CINNABAR_BRIGHT,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(RebaseMessage::AbortRebase.into()),
                text("Abort rebase and restore previous HEAD state from ORIG_HEAD").size(11),
                tooltip::Position::Bottom,
            ));
        } else if !self.steps.is_empty() {
            // Execute Rebase Button
            header_actions = header_actions.push(tooltip(
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
                text("Apply sequenced rebase plan to branch history (git rebase -i)").size(11),
                tooltip::Position::Bottom,
            ));
        }

        let header = row![
            text("⚖️ K'UK'ULKAN REBASE & CONFLICT CRUCIBLE")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(12),
            text(format!("{} Commits in Sequencer", self.steps.len()))
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            header_actions,
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        // Feedback banner if present
        let feedback_banner: Element<'a, Message> = if !self.rebase_feedback.is_empty() {
            let color = if self.has_conflict {
                MayaColors::CINNABAR_BRIGHT
            } else {
                MayaColors::JADE_GLOW
            };
            container(
                row![
                    text(if self.has_conflict { "⚠️ " } else { "ℹ️ " }).size(11),
                    text(&self.rebase_feedback).size(11).color(color),
                ]
                .align_y(Alignment::Center)
                .padding(Padding::from([4, 10])),
            )
            .width(Length::Fill)
            .style(|_theme: &iced::Theme| container::Style {
                background: Some(Color { a: 0.35, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 3.0.into(),
                },
                ..Default::default()
            })
            .into()
        } else {
            row![].into()
        };

        // Conflict section if active
        let conflict_banner = if self.has_conflict {
            self.render_conflict_resolver()
        } else {
            row![].into()
        };

        // Main body content
        let mut step_cards = column![].spacing(8);

        if self.steps.is_empty() && !self.is_rebasing {
            let empty_notice = container(
                column![
                    text("🏛️ REBASE SEQUENCER INERT").size(13).color(MayaColors::SOLAR_GOLD),
                    Space::new().height(6),
                    text("No unpushed commits currently loaded in the crucible sequencer.").size(11).color(MayaColors::PARCHMENT_MUTED),
                    Space::new().height(4),
                    text("Click '⚡ LOAD COMMITS' above to populate local commits between HEAD and upstream for interactive rebasing.").size(10).color(MayaColors::GLYPH_DIM),
                ]
                .padding(Padding::from([24, 20]))
                .align_x(Alignment::Center),
            )
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.25, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            });

            step_cards = step_cards.push(empty_notice);
        } else {
            for (idx, step) in self.steps.iter().enumerate() {
                step_cards = step_cards.push(self.render_step_card(idx, step));
            }
        }

        let body = scrollable(
            column![feedback_banner, Space::new().height(6), conflict_banner, step_cards]
                .padding(Padding::from([8, 12])),
        )
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
            tooltip(
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
                text("Move commit earlier in rebase order").size(11),
                tooltip::Position::Top,
            ),
            tooltip(
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
                text("Move commit later in rebase order").size(11),
                tooltip::Position::Top,
            ),
        ]
        .spacing(3)
        .align_y(Alignment::Center);

        let is_current = self.is_rebasing && self.current_step == idx;
        let border_color = if is_current {
            MayaColors::SOLAR_GOLD
        } else {
            MayaColors::OBSIDIAN_BORDER
        };

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
            .style(move |_theme| container::Style {
                background: Some(Color { a: 0.85, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                border: Border {
                    color: border_color,
                    width: if is_current { 1.5 } else { 1.0 },
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

        let tooltip_desc = match action {
            RebaseAction::Pick => "Keep this commit in history (pick <sha>)",
            RebaseAction::Reword => "Keep changes but prompt to edit commit message (reword <sha>)",
            RebaseAction::Squash => "Meld changes into previous commit (squash <sha>)",
            RebaseAction::Drop => "Discard this commit from history (drop <sha>)",
        };

        let btn = button(text(label).size(9).color(text_color))
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
            .on_press(RebaseMessage::SetAction(idx, action).into());

        tooltip(btn, text(tooltip_desc).size(11), tooltip::Position::Top).into()
    }

    fn render_conflict_resolver<'a, Message: 'a + From<RebaseMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title = row![
            text("⚔️ CONFLICT CRUCIBLE:").size(11).color(MayaColors::CINNABAR_BRIGHT),
            text(format!(" {}", self.conflict_file)).size(11).color(MayaColors::STARLIGHT_WHITE),
        ]
        .align_y(Alignment::Center);

        // Conflicted files tab selector if there are multiple conflicted files
        let file_selector = if self.conflicted_files.len() > 1 {
            let mut tabs = row![text("Files: ").size(10).color(MayaColors::GLYPH_DIM)].spacing(4).align_y(Alignment::Center);
            for f in &self.conflicted_files {
                let is_active = f == &self.conflict_file;
                let (bg, border) = if is_active {
                    (MayaColors::CINNABAR_DARK, MayaColors::CINNABAR_BRIGHT)
                } else {
                    (MayaColors::OBSIDIAN_SURFACE, MayaColors::OBSIDIAN_BORDER)
                };
                let f_clone = f.clone();
                let short_name = std::path::Path::new(f)
                    .file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_else(|| f.clone());
                tabs = tabs.push(
                    button(text(short_name).size(9).color(MayaColors::STARLIGHT_WHITE))
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
                        .on_press(RebaseMessage::SelectConflictFile(f_clone).into()),
                );
            }
            row![tabs].padding(Padding::from([2, 0]))
        } else {
            row![]
        };

        let ours_preview = if self.conflict_ours.is_empty() {
            "(Empty / Unchanged in Ours)".to_string()
        } else {
            self.conflict_ours.chars().take(300).collect::<String>()
        };

        let theirs_preview = if self.conflict_theirs.is_empty() {
            "(Empty / Unchanged in Theirs)".to_string()
        } else {
            self.conflict_theirs.chars().take(300).collect::<String>()
        };

        let ours_box = container(
            column![
                text("OURS (LOCAL HEAD)").size(10).color(MayaColors::JADE_GLOW),
                Space::new().height(4),
                scrollable(text(ours_preview).size(10).color(MayaColors::STARLIGHT_WHITE)).height(Length::Fixed(80.0)),
                Space::new().height(6),
                tooltip(
                    button(text("Take Ours ✓").size(10).color(MayaColors::JADE_GLOW))
                        .padding(Padding::from([3, 8]))
                        .style(|_theme, _status| button::Style {
                            background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                            border: Border {
                                color: MayaColors::JADE_GLOW,
                                width: 1.0,
                                radius: 3.0.into(),
                            },
                            ..Default::default()
                        })
                        .on_press(RebaseMessage::ResolveConflictOurs.into()),
                    text("Resolve conflict using local HEAD version (git checkout --ours)").size(11),
                    tooltip::Position::Top,
                ),
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
                scrollable(text(theirs_preview).size(10).color(MayaColors::STARLIGHT_WHITE)).height(Length::Fixed(80.0)),
                Space::new().height(6),
                tooltip(
                    button(text("Take Theirs ✓").size(10).color(MayaColors::MAYA_BLUE))
                        .padding(Padding::from([3, 8]))
                        .style(|_theme, _status| button::Style {
                            background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                            border: Border {
                                color: MayaColors::MAYA_BLUE,
                                width: 1.0,
                                radius: 3.0.into(),
                            },
                            ..Default::default()
                        })
                        .on_press(RebaseMessage::ResolveConflictTheirs.into()),
                    text("Resolve conflict using incoming rebase version (git checkout --theirs)").size(11),
                    tooltip::Position::Top,
                ),
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

        let both_button = tooltip(
            button(text("Take Both (Ours + Theirs) ⚡").size(10).color(MayaColors::SOLAR_GOLD))
                .padding(Padding::from([3, 10]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::SOLAR_GOLD,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(RebaseMessage::ResolveConflictBoth.into()),
            text("Synthesize both versions by appending Theirs to Ours").size(11),
            tooltip::Position::Top,
        );

        let action_row = row![
            both_button,
            Space::new().width(Length::Fill),
            text(format!("{} conflicted file(s)", self.conflicted_files.len())).size(10).color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        let content = column![
            title,
            Space::new().height(4),
            file_selector,
            Space::new().height(6),
            row![ours_box, Space::new().width(10), theirs_box],
            Space::new().height(8),
            action_row,
            Space::new().height(12),
        ];

        content.into()
    }
}
