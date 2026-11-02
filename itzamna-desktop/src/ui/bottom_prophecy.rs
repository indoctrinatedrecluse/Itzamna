//! The Bottom Prophecy Tray (El Feed de Profecías)
//! Real-time panoramic console log of repository events, git executions, and Ixchel Oracle telemetry.

use iced::widget::{button, column, container, row, scrollable, text, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProphecyHeight {
    Minimized, // 28px
    Standard,  // 92px
    Expanded,  // 180px
}

#[derive(Debug, Clone)]
pub enum ProphecyMessage {
    ToggleDrawer,
    CycleHeight,
    ClearFeed,
    FilterLevel(Option<ProphecyLevel>),
}

#[derive(Debug, Clone)]
pub struct ProphecyEntry {
    pub timestamp: &'static str,
    pub glyph: &'static str,
    pub message: String,
    pub level: ProphecyLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProphecyLevel {
    Solar,   // Success / Commit
    Jade,    // Branch / Creation
    Maya,    // Network / Stream
    Xibalba, // Warning / Error
}

impl ProphecyLevel {
    pub fn badge(&self) -> &'static str {
        match self {
            Self::Solar => "SOLAR",
            Self::Jade => "JADE",
            Self::Maya => "MAYA",
            Self::Xibalba => "XIBALBA",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::Solar => MayaColors::SOLAR_GOLD,
            Self::Jade => MayaColors::JADE_GLOW,
            Self::Maya => MayaColors::MAYA_BLUE,
            Self::Xibalba => MayaColors::CINNABAR_BRIGHT,
        }
    }
}

pub struct BottomProphecy {
    pub is_expanded: bool,
    pub height_mode: ProphecyHeight,
    pub active_filter: Option<ProphecyLevel>,
    pub entries: Vec<ProphecyEntry>,
}

impl Default for BottomProphecy {
    fn default() -> Self {
        Self::new()
    }
}

impl BottomProphecy {
    pub fn new() -> Self {
        Self {
            is_expanded: true,
            height_mode: ProphecyHeight::Standard,
            active_filter: None,
            entries: vec![
                ProphecyEntry {
                    timestamp: "22:44:08",
                    glyph: "☀️",
                    message: "Solar commit sealed: 3b7f7a1 — 'feat(desktop): complete Phase 1'".into(),
                    level: ProphecyLevel::Solar,
                },
                ProphecyEntry {
                    timestamp: "22:47:44",
                    glyph: "🏛️",
                    message: "Awakening Stelae Palace (Phase 2 multi-panel layout)".into(),
                    level: ProphecyLevel::Jade,
                },
                ProphecyEntry {
                    timestamp: "22:49:05",
                    glyph: "📁",
                    message: "Background asset verified: assets/video_loop.bin stream online".into(),
                    level: ProphecyLevel::Jade,
                },
                ProphecyEntry {
                    timestamp: "22:50:00",
                    glyph: "🌙",
                    message: "Ixchel Oracle bridge listening on http://127.0.0.1:9090".into(),
                    level: ProphecyLevel::Maya,
                },
            ],
        }
    }

    pub fn push(&mut self, timestamp: &'static str, glyph: &'static str, msg: String, level: ProphecyLevel) {
        self.entries.push(ProphecyEntry {
            timestamp,
            glyph,
            message: msg,
            level,
        });
    }

    pub fn view<'a, Message: 'a + From<ProphecyMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let (height_val, cycle_icon, cycle_tooltip) = match self.height_mode {
            ProphecyHeight::Minimized => (28.0, "▲ Expand", "Expand console drawer to standard height"),
            ProphecyHeight::Standard => (96.0, "⤢ Panoramic", "Expand console drawer to full panoramic height"),
            ProphecyHeight::Expanded => (180.0, "▼ Compact", "Minimize console drawer to single bar"),
        };

        // Header Row
        let header = row![
            text("🔮 FEED DE PROFECÍAS")
                .size(10)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            container(text(format!("{} LOGS RECORDED", self.entries.len())).size(8).color(MayaColors::OBSIDIAN_VOID))
                .padding(Padding::from([1, 5]))
                .style(|_| container::Style {
                    background: Some(MayaColors::JADE_GLOW.into()),
                    border: Border { radius: 2.0.into(), ..Default::default() },
                    ..Default::default()
                }),
            Space::new().width(12),
            // Filter Pills in Header
            self.filter_pill(None, "ALL"),
            Space::new().width(4),
            self.filter_pill(Some(ProphecyLevel::Solar), "SOLAR"),
            Space::new().width(4),
            self.filter_pill(Some(ProphecyLevel::Jade), "JADE"),
            Space::new().width(4),
            self.filter_pill(Some(ProphecyLevel::Maya), "MAYA"),
            Space::new().width(4),
            self.filter_pill(Some(ProphecyLevel::Xibalba), "XIBALBA"),
            Space::new().width(Length::Fill),
            tooltip(
                button(text(cycle_icon).size(9).color(MayaColors::MAYA_BLUE))
                    .padding(Padding::from([2, 6]))
                    .style(|_, _| button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 3.0.into() },
                        ..Default::default()
                    })
                    .on_press(ProphecyMessage::CycleHeight.into()),
                cycle_tooltip,
                tooltip::Position::Top,
            ),
            Space::new().width(6),
            tooltip(
                button(text("🗑️ Clear").size(9).color(MayaColors::GLYPH_DIM))
                    .padding(Padding::from([2, 6]))
                    .style(|_, _| button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 3.0.into() },
                        ..Default::default()
                    })
                    .on_press(ProphecyMessage::ClearFeed.into()),
                "Clear console prophecy buffer",
                tooltip::Position::Top,
            ),
        ]
        .padding(Padding::from([3, 10]))
        .align_y(Alignment::Center);

        if self.height_mode == ProphecyHeight::Minimized {
            return container(header)
                .width(Length::Fill)
                .height(28)
                .style(|_theme| container::Style {
                    background: Some(Color { a: 0.96, ..MayaColors::OBSIDIAN_BASE }.into()),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                })
                .into();
        }

        // --- Filter Entries ---
        let filtered_entries: Vec<&ProphecyEntry> = self
            .entries
            .iter()
            .rev()
            .filter(|e| self.active_filter.map_or(true, |f| e.level == f))
            .collect();

        // 1. Left Telemetry Sub-Panel (20% width)
        let solar_count = self.entries.iter().filter(|e| e.level == ProphecyLevel::Solar).count();
        let jade_count = self.entries.iter().filter(|e| e.level == ProphecyLevel::Jade).count();
        let maya_count = self.entries.iter().filter(|e| e.level == ProphecyLevel::Maya).count();
        let xibalba_count = self.entries.iter().filter(|e| e.level == ProphecyLevel::Xibalba).count();

        let telemetry_panel = container(
            column![
                text("TELEMETRY VECTORS").size(8).color(MayaColors::GLYPH_DIM),
                Space::new().height(4),
                row![
                    text("☀️ Commits: ").size(8).color(MayaColors::GLYPH_DIM),
                    text(format!("{}", solar_count)).size(9).color(MayaColors::SOLAR_GOLD),
                    Space::new().width(8),
                    text("🌱 Refs: ").size(8).color(MayaColors::GLYPH_DIM),
                    text(format!("{}", jade_count)).size(9).color(MayaColors::JADE_GLOW),
                ],
                Space::new().height(2),
                row![
                    text("🌐 Conduit: ").size(8).color(MayaColors::GLYPH_DIM),
                    text(format!("{}", maya_count)).size(9).color(MayaColors::MAYA_BLUE),
                    Space::new().width(8),
                    text("⚠️ Alerts: ").size(8).color(MayaColors::GLYPH_DIM),
                    text(format!("{}", xibalba_count)).size(9).color(MayaColors::CINNABAR_BRIGHT),
                ],
            ]
            .padding(Padding::from([4, 8]))
        )
        .width(Length::Fixed(190.0))
        .style(|_| container::Style {
            background: Some(Color { a: 0.5, ..MayaColors::OBSIDIAN_SURFACE }.into()),
            border: Border {
                color: MayaColors::OBSIDIAN_BORDER,
                width: 1.0,
                radius: 3.0.into(),
            },
            ..Default::default()
        });

        // 2. Center Panoramic Log Stream (Spanning all remaining width!)
        let mut log_list = column![].spacing(3).width(Length::Fill);

        for entry in filtered_entries {
            let color = entry.level.color();
            let badge_bg = Color { a: 0.25, ..color };

            let log_row = row![
                text(format!("[{}]", entry.timestamp))
                    .size(9)
                    .color(MayaColors::GLYPH_DIM),
                Space::new().width(6),
                container(text(entry.level.badge()).size(7).color(color))
                    .padding(Padding::from([1, 4]))
                    .style(move |_| container::Style {
                        background: Some(badge_bg.into()),
                        border: Border { color, width: 0.5, radius: 2.0.into() },
                        ..Default::default()
                    }),
                Space::new().width(6),
                text(entry.glyph).size(10),
                Space::new().width(6),
                text(&entry.message)
                    .size(10)
                    .color(color)
                    .width(Length::Fill)
                    .wrapping(text::Wrapping::Word),
            ]
            .padding(Padding::from([2, 4]))
            .align_y(Alignment::Center);

            log_list = log_list.push(
                container(log_row)
                    .width(Length::Fill)
                    .style(|_| container::Style {
                        background: Some(Color { a: 0.15, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                        border: Border { radius: 2.0.into(), ..Default::default() },
                        ..Default::default()
                    })
            );
        }

        let log_stream = scrollable(log_list.padding(Padding::from([2, 6]))).width(Length::Fill);

        let panoramic_body = row![
            telemetry_panel,
            Space::new().width(6),
            log_stream,
        ]
        .padding(Padding::from([2, 8]))
        .width(Length::Fill)
        .height(Length::Fill);

        container(column![header, panoramic_body])
            .width(Length::Fill)
            .height(height_val)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.96, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn filter_pill<'a, Message: 'a + From<ProphecyMessage> + Clone>(
        &'a self,
        target: Option<ProphecyLevel>,
        label: &'static str,
    ) -> Element<'a, Message> {
        let is_active = self.active_filter == target;
        let (bg, border_color, text_color) = if is_active {
            (
                Color { a: 0.35, ..MayaColors::SOLAR_GOLD },
                MayaColors::SOLAR_GOLD,
                MayaColors::SOLAR_GOLD,
            )
        } else {
            (
                Color::TRANSPARENT,
                MayaColors::OBSIDIAN_BORDER,
                MayaColors::GLYPH_DIM,
            )
        };

        button(text(label).size(8).color(text_color))
            .padding(Padding::from([1, 5]))
            .style(move |_, _| button::Style {
                background: Some(bg.into()),
                border: Border { color: border_color, width: 0.8, radius: 2.0.into() },
                ..Default::default()
            })
            .on_press(ProphecyMessage::FilterLevel(target).into())
            .into()
    }
}
