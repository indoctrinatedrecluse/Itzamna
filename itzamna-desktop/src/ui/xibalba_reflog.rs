//! The Xibalba Mirror (El Espejo de Xibalbá — Visual Git Reflog & Time Travel)
//! Complete history of all HEAD movements with interactive Reset Dials (Soft/Mixed/Hard).

use iced::widget::{button, column, container, row, scrollable, text, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::git::ReflogEntryInfo;
use crate::theme::MayaColors;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResetMode {
    Soft,
    Mixed,
    Hard,
}

#[derive(Debug, Clone)]
pub enum ReflogMessage {
    SelectEntry(usize),
    SetResetMode(ResetMode),
    ExecuteReset,
}

pub struct XibalbaReflogWorkbench {
    pub entries: Vec<ReflogEntryInfo>,
    pub selected_index: Option<usize>,
    pub reset_mode: ResetMode,
}

impl Default for XibalbaReflogWorkbench {
    fn default() -> Self {
        Self::new()
    }
}

impl XibalbaReflogWorkbench {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            selected_index: None,
            reset_mode: ResetMode::Mixed,
        }
    }

    pub fn set_entries(&mut self, entries: Vec<ReflogEntryInfo>) {
        if self.selected_index.is_none() && !entries.is_empty() {
            self.selected_index = Some(0);
        }
        self.entries = entries;
    }

    pub fn selected_entry(&self) -> Option<&ReflogEntryInfo> {
        self.selected_index.and_then(|idx| self.entries.get(idx))
    }

    pub fn view<'a, Message: 'a + From<ReflogMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_bar = row![
            text("🪞 EL ESPEJO DE XIBALBÁ (GIT REFLOG & TIME TRAVEL)")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(12),
            text(format!("{} Ancestral Movements", self.entries.len()))
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            text("Time-Travel Control:").size(10).color(MayaColors::PARCHMENT_MUTED),
            Space::new().width(6),
            self.render_reset_mode_pill(ResetMode::Soft, "SOFT (Keep Index)", "Reset HEAD but keep staged files & working tree (--soft)"),
            self.render_reset_mode_pill(ResetMode::Mixed, "MIXED (Default)", "Reset HEAD & index, keep working directory changes (--mixed)"),
            self.render_reset_mode_pill(ResetMode::Hard, "HARD (Purge)", "Reset HEAD, index, and working tree to this snapshot (--hard)"),
            Space::new().width(8),
            tooltip(
                button(text("⚡ TIME JUMP").size(10).color(MayaColors::OBSIDIAN_VOID))
                    .padding(Padding::from([3, 10]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::CINNABAR_BRIGHT.into()),
                        border: Border {
                            color: MayaColors::SOLAR_GOLD,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(ReflogMessage::ExecuteReset.into()),
                text("Reset active HEAD to the selected reflog movement").size(11),
                tooltip::Position::Bottom,
            ),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center)
        .spacing(4);

        let mut list_col = column![].spacing(4);

        for (idx, entry) in self.entries.iter().enumerate() {
            let is_selected = self.selected_index == Some(idx);
            list_col = list_col.push(self.render_entry_row(idx, entry, is_selected));
        }

        let body = scrollable(list_col.padding(Padding::from([8, 12])))
            .height(Length::Fill);

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

    fn render_reset_mode_pill<'a, Message: 'a + From<ReflogMessage> + Clone>(
        &'a self,
        mode: ResetMode,
        label: &'static str,
        tooltip_desc: &'static str,
    ) -> Element<'a, Message> {
        let is_active = self.reset_mode == mode;
        let (bg, text_color, border) = if is_active {
            match mode {
                ResetMode::Soft => (MayaColors::JADE_PRIME, MayaColors::OBSIDIAN_VOID, MayaColors::JADE_GLOW),
                ResetMode::Mixed => (MayaColors::SOLAR_AMBER, MayaColors::OBSIDIAN_VOID, MayaColors::SOLAR_GOLD),
                ResetMode::Hard => (MayaColors::CINNABAR_BRIGHT, MayaColors::STARLIGHT_WHITE, MayaColors::CINNABAR_BRIGHT),
            }
        } else {
            (MayaColors::OBSIDIAN_SURFACE, MayaColors::GLYPH_DIM, MayaColors::OBSIDIAN_BORDER)
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
            .on_press(ReflogMessage::SetResetMode(mode).into());

        tooltip(btn, text(tooltip_desc).size(11), tooltip::Position::Bottom).into()
    }

    fn render_entry_row<'a, Message: 'a + From<ReflogMessage> + Clone>(
        &'a self,
        idx: usize,
        entry: &'a ReflogEntryInfo,
        is_selected: bool,
    ) -> Element<'a, Message> {
        let action_color = match entry.action_type {
            "checkout" => MayaColors::MAYA_BLUE,
            "commit" => MayaColors::JADE_GLOW,
            "rebase" => MayaColors::SOLAR_GOLD,
            "reset" => MayaColors::CINNABAR_BRIGHT,
            _ => MayaColors::PARCHMENT_MUTED,
        };

        let row_content = row![
            text(format!("HEAD@{{{}}}", entry.index)).size(10).color(MayaColors::GLYPH_DIM),
            Space::new().width(8),
            text(&entry.new_sha).size(10).color(action_color),
            Space::new().width(8),
            container(text(entry.action_type.to_uppercase()).size(8).color(MayaColors::OBSIDIAN_VOID))
                .padding(Padding::from([1, 4]))
                .style(move |_theme| container::Style {
                    background: Some(action_color.into()),
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            Space::new().width(10),
            text(&entry.message).size(11).color(MayaColors::STARLIGHT_WHITE),
            Space::new().width(Length::Fill),
            text(&entry.committer_name).size(9).color(MayaColors::PARCHMENT_MUTED),
        ]
        .padding(Padding::from([6, 10]))
        .align_y(Alignment::Center);

        let border_color = if is_selected {
            MayaColors::SOLAR_GOLD
        } else {
            MayaColors::OBSIDIAN_BORDER
        };

        button(row_content)
            .width(Length::Fill)
            .padding(0)
            .style(move |_theme, _status| button::Style {
                background: Some(Color { a: if is_selected { 0.40 } else { 0.15 }, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                border: Border {
                    color: border_color,
                    width: 1.0,
                    radius: 3.0.into(),
                },
                ..Default::default()
            })
            .on_press(ReflogMessage::SelectEntry(idx).into())
            .into()
    }
}
