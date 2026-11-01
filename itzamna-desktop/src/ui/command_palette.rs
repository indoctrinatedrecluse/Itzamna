//! Command Palette & Sacred Glyph Keybindings Chamber
//! Quick-action fuzzy launcher (Ctrl+P / Ctrl+K) and interactive shortcut altar (F1 / ?)

use iced::widget::{button, column, container, row, scrollable, text, text_input, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum PaletteMessage {
    Open,
    Close,
    QueryChanged(String),
    SelectPrevious,
    SelectNext,
    ExecuteSelected,
    ExecuteCommand(&'static str),
    ToggleHelp,
    CloseHelp,
}

#[derive(Debug, Clone)]
pub struct PaletteCommand {
    pub id: &'static str,
    pub title: &'static str,
    pub glyph: &'static str,
    pub category: &'static str,
    pub shortcut: &'static str,
}

pub struct CommandPalette {
    pub is_open: bool,
    pub is_help_open: bool,
    pub search_query: String,
    pub selected_index: usize,
    pub all_commands: Vec<PaletteCommand>,
}

impl Default for CommandPalette {
    fn default() -> Self {
        Self::new()
    }
}

impl CommandPalette {
    pub fn new() -> Self {
        let all_commands = vec![
            PaletteCommand { id: "tab_editor", title: "Katun Code Editor (Inscribe & Edit Relics)", glyph: "✍️", category: "Chambers", shortcut: "Ctrl+E" },
            PaletteCommand { id: "tab_workflows", title: "Celestial Workflows (GitHub Actions via gh CLI)", glyph: "🚀", category: "Chambers", shortcut: "Ctrl+W" },
            PaletteCommand { id: "tab_dag", title: "Tree of Life (Yaxché Commit DAG)", glyph: "🌳", category: "Chambers", shortcut: "Ctrl+1" },
            PaletteCommand { id: "tab_diff", title: "Altar of Creation (Surgical Diff & Hunk Staging)", glyph: "🔪", category: "Chambers", shortcut: "Ctrl+2" },
            PaletteCommand { id: "tab_rebase", title: "K'uk'ulkan Crucible (Visual Interactive Rebase)", glyph: "🔄", category: "Chambers", shortcut: "Ctrl+3" },
            PaletteCommand { id: "tab_conflict", title: "Three-Way Conflict Altar (Ours vs. Theirs)", glyph: "⚖️", category: "Chambers", shortcut: "Ctrl+4" },
            PaletteCommand { id: "tab_blame", title: "The Ahau Lineage (Line-by-Line git blame Heatmap)", glyph: "👑", category: "Chambers", shortcut: "Ctrl+5" },
            PaletteCommand { id: "tab_log", title: "The Popol Vuh Chronicle (Deep git log History)", glyph: "📜", category: "Chambers", shortcut: "Ctrl+6" },
            PaletteCommand { id: "tab_reflog", title: "The Xibalba Mirror (Ancestral reflog & Time Travel)", glyph: "🪞", category: "Chambers", shortcut: "Ctrl+7" },
            PaletteCommand { id: "tab_merge", title: "K'uk'ulkan Confluence (git merge & cherry-pick)", glyph: "🌿", category: "Chambers", shortcut: "Ctrl+8" },
            PaletteCommand { id: "tab_clean", title: "The Chac Purifier (git clean & Multi-Stash Vault)", glyph: "🌧️", category: "Chambers", shortcut: "Ctrl+9" },
            PaletteCommand { id: "tab_remote", title: "The Overseer Portals (git remote push/pull/prune)", glyph: "🌐", category: "Chambers", shortcut: "Ctrl+0" },
            PaletteCommand { id: "tab_oracle", title: "Ixchel Oracle Radar (DORA Telemetry & Conduit)", glyph: "🔮", category: "Chambers", shortcut: "Ctrl+I" },
            PaletteCommand { id: "action_open_folder", title: "Open Project Sanctuary (Folder Picker)", glyph: "📂", category: "Sanctuary", shortcut: "Ctrl+O" },
            PaletteCommand { id: "action_pull", title: "Pull & Rebase from Remote Horizon (git pull --rebase)", glyph: "⬇️", category: "Sync", shortcut: "" },
            PaletteCommand { id: "action_push", title: "Push Commits to Remote Horizon (git push origin)", glyph: "⬆️", category: "Sync", shortcut: "" },
            PaletteCommand { id: "action_credentials", title: "Local Git Identity & Remote Origins (.git/config)", glyph: "⚙️", category: "Sanctuary", shortcut: "" },
            PaletteCommand { id: "action_sync", title: "Trigger Ixchel DORA & Velocity Sync Ritual", glyph: "⚡", category: "Rituals", shortcut: "" },
            PaletteCommand { id: "action_branch", title: "Sprout New Branch (git branch)", glyph: "🌱", category: "Rituals", shortcut: "" },
            PaletteCommand { id: "action_tag", title: "Inscribe Solar Tag Seal (git tag)", glyph: "🏷️", category: "Rituals", shortcut: "" },
            PaletteCommand { id: "action_stash", title: "Shelter Working Tree into Ak'bal Vault (git stash)", glyph: "📦", category: "Rituals", shortcut: "" },
            PaletteCommand { id: "action_cycle_bg", title: "Cycle Loom Background Mode (Video/Procedural/Stealth)", glyph: "🎨", category: "Cosmos", shortcut: "Ctrl+L" },
            PaletteCommand { id: "action_toggle_codex", title: "Toggle Left Codex Sidebar Collapse", glyph: "📜", category: "View", shortcut: "Ctrl+B" },
            PaletteCommand { id: "action_shortcuts", title: "Reveal Sacred Keyboard Shortcuts Cheat Sheet", glyph: "⌨️", category: "Help", shortcut: "F1 / ?" },
        ];

        Self {
            is_open: false,
            is_help_open: false,
            search_query: "".to_string(),
            selected_index: 0,
            all_commands,
        }
    }

    pub fn filtered_commands(&self) -> Vec<&PaletteCommand> {
        let q = self.search_query.to_lowercase();
        if q.is_empty() {
            self.all_commands.iter().collect()
        } else {
            self.all_commands
                .iter()
                .filter(|c| {
                    c.title.to_lowercase().contains(&q)
                        || c.category.to_lowercase().contains(&q)
                        || c.shortcut.to_lowercase().contains(&q)
                })
                .collect()
        }
    }

    pub fn view<'a, Message: 'a + From<PaletteMessage> + Clone>(&'a self) -> Option<Element<'a, Message>> {
        if self.is_help_open {
            return Some(self.view_help_modal());
        }

        if !self.is_open {
            return None;
        }

        let search_box = row![
            text("🔍").size(14),
            Space::new().width(8),
            text_input("Type a sacred ritual or chamber (e.g. 'blame', 'rebase', 'merge', 'sync')...", &self.search_query)
                .on_input(|s| PaletteMessage::QueryChanged(s).into())
                .size(13)
                .padding(Padding::from([6, 8]))
                .width(Length::Fill),
            button(text("ESC").size(9).color(MayaColors::GLYPH_DIM))
                .padding(Padding::from([4, 6]))
                .style(|_, _| button::Style {
                    background: Some(MayaColors::OBSIDIAN_BASE.into()),
                    border: Border { radius: 3.0.into(), ..Default::default() },
                    ..Default::default()
                })
                .on_press(PaletteMessage::Close.into()),
        ]
        .padding(Padding::from([8, 12]))
        .align_y(Alignment::Center);

        let filtered = self.filtered_commands();
        let mut list_col = column![].spacing(3);

        for (idx, cmd) in filtered.iter().enumerate() {
            let is_selected = idx == self.selected_index;
            let (bg_color, border_color, text_color) = if is_selected {
                (
                    Color { a: 0.35, ..MayaColors::JADE_GLOW },
                    MayaColors::JADE_GLOW,
                    MayaColors::STARLIGHT_WHITE,
                )
            } else {
                (
                    Color { a: 0.15, ..MayaColors::OBSIDIAN_SURFACE },
                    Color::TRANSPARENT,
                    MayaColors::PARCHMENT_MUTED,
                )
            };

            let cmd_row = row![
                text(cmd.glyph).size(13),
                Space::new().width(10),
                column![
                    row![
                        text(cmd.title).size(11).color(text_color),
                        Space::new().width(8),
                        container(text(cmd.category).size(8).color(MayaColors::OBSIDIAN_VOID))
                            .padding(Padding::from([1, 4]))
                            .style(|_| container::Style {
                                background: Some(MayaColors::SOLAR_GOLD.into()),
                                border: Border { radius: 2.0.into(), ..Default::default() },
                                ..Default::default()
                            }),
                    ].align_y(Alignment::Center),
                ],
                Space::new().width(Length::Fill),
                text(cmd.shortcut).size(9).color(MayaColors::MAYA_BLUE),
            ]
            .padding(Padding::from([6, 10]))
            .align_y(Alignment::Center);

            let cmd_id = cmd.id;
            let btn = button(cmd_row)
                .width(Length::Fill)
                .style(move |_, _| button::Style {
                    background: Some(bg_color.into()),
                    border: Border { color: border_color, width: 1.0, radius: 4.0.into() },
                    ..Default::default()
                })
                .on_press(PaletteMessage::ExecuteCommand(cmd_id).into());

            list_col = list_col.push(btn);
        }

        let list_scroll = scrollable(list_col.padding(Padding::from([4, 10]))).height(Length::Fixed(280.0));

        let palette_box = container(
            column![
                search_box,
                container(Space::new())
                    .height(1)
                    .width(Length::Fill)
                    .style(|_| container::Style { background: Some(MayaColors::OBSIDIAN_BORDER.into()), ..Default::default() }),
                list_scroll,
            ]
        )
        .width(560)
        .style(|_| container::Style {
            background: Some(Color { a: 0.96, ..MayaColors::OBSIDIAN_BASE }.into()),
            border: Border {
                color: MayaColors::SOLAR_GOLD,
                width: 1.5,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        Some(
            container(palette_box)
                .width(Length::Fill)
                .height(Length::Fill)
                .align_x(Alignment::Center)
                .align_y(Alignment::Start)
                .padding(Padding { top: 60.0, right: 0.0, bottom: 0.0, left: 0.0 })
                .style(|_| container::Style {
                    background: Some(Color { a: 0.60, ..MayaColors::OBSIDIAN_VOID }.into()),
                    ..Default::default()
                })
                .into()
        )
    }

    fn view_help_modal<'a, Message: 'a + From<PaletteMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let shortcuts = vec![
            ("Ctrl + P / Ctrl + K", "Open Command Palette (All chambers & rituals)"),
            ("Ctrl + O", "Open Project Sanctuary (Folder Picker)"),
            ("Ctrl + E", "Katun Code Editor (Inscribe & Edit Relics)"),
            ("Ctrl + S", "Save Active Relic in Editor"),
            ("Ctrl + W", "Celestial Workflows (GitHub Actions via gh)"),
            ("F1 / ?", "Show / Hide this Sacred Keymap Altar"),
            ("Ctrl + 1 .. 9", "Instant jump to Workbench Tabs 1 through 9"),
            ("Ctrl + 0", "Jump to Overseer Portals (Remotes)"),
            ("Ctrl + I", "Jump to Ixchel Oracle Radar"),
            ("Ctrl + B", "Toggle Left Codex (Branches/Files) sidebar"),
            ("Ctrl + L", "Cycle Loom Canvas mode (Video / Procedural / Stealth)"),
            ("J / K / Down / Up", "Navigate commits in DAG or files in Staging"),
            ("Space", "Stage or unstage currently selected file"),
            ("Ctrl + Enter", "Consecrate and commit staged changes to DAG"),
            ("Escape", "Dismiss Command Palette or Modal Altar"),
        ];

        let mut rows_col = column![].spacing(6);
        for (key, desc) in shortcuts {
            rows_col = rows_col.push(
                row![
                    container(text(key).size(10).color(MayaColors::SOLAR_GOLD))
                        .padding(Padding::from([2, 6]))
                        .width(Length::Fixed(150.0))
                        .style(|_| container::Style {
                            background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                            border: Border { color: MayaColors::SOLAR_GOLD, width: 1.0, radius: 3.0.into() },
                            ..Default::default()
                        }),
                    Space::new().width(12),
                    text(desc).size(10).color(MayaColors::STARLIGHT_WHITE),
                ]
                .align_y(Alignment::Center)
            );
        }

        let modal_box = container(
            column![
                row![
                    text("⌨️ SACRED GLYPH KEYBOARD MAP").size(12).color(MayaColors::SOLAR_GOLD),
                    Space::new().width(Length::Fill),
                    button(text("✖").size(11).color(MayaColors::GLYPH_DIM))
                        .padding(Padding::from([2, 6]))
                        .on_press(PaletteMessage::CloseHelp.into()),
                ].align_y(Alignment::Center),
                Space::new().height(10),
                rows_col,
                Space::new().height(12),
                row![
                    Space::new().width(Length::Fill),
                    button(text("Return to Sanctuary").size(10).color(MayaColors::OBSIDIAN_VOID))
                        .padding(Padding::from([4, 12]))
                        .style(|_, _| button::Style {
                            background: Some(MayaColors::SOLAR_GOLD.into()),
                            border: Border { radius: 3.0.into(), ..Default::default() },
                            ..Default::default()
                        })
                        .on_press(PaletteMessage::CloseHelp.into()),
                ]
            ]
            .padding(Padding::from([14, 18]))
        )
        .width(520)
        .style(|_| container::Style {
            background: Some(Color { a: 0.97, ..MayaColors::OBSIDIAN_BASE }.into()),
            border: Border {
                color: MayaColors::JADE_GLOW,
                width: 1.5,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        container(modal_box)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(|_| container::Style {
                background: Some(Color { a: 0.70, ..MayaColors::OBSIDIAN_VOID }.into()),
                ..Default::default()
            })
            .into()
    }
}
