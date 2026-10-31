//! The Obsidian Altar Bar (Footer Status Bar)
//! Displays active sovereign branch, commit signet, workspace purity, and Ixchel Oracle heartbeat.

use iced::widget::{container, row, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub struct StatusBar {
    pub active_branch: String,
    pub head_sha: String,
    pub is_pristine: bool,
    pub ixchel_status: &'static str,
    pub cycle_time: f32,
}

impl Default for StatusBar {
    fn default() -> Self {
        Self {
            active_branch: "main".to_string(),
            head_sha: "4076417".to_string(),
            is_pristine: true,
            ixchel_status: "Dormant (Local Oracle)",
            cycle_time: 0.0,
        }
    }
}

impl StatusBar {
    pub fn tick(&mut self, dt: f32) {
        self.cycle_time += dt;
    }

    pub fn view<'a, Message: 'a>(&'a self) -> Element<'a, Message> {
        let branch_badge = row![
            text("🌿 YAXCHE:").size(10).color(MayaColors::GLYPH_DIM),
            text(format!(" {}", self.active_branch))
                .size(11)
                .color(MayaColors::JADE_GLOW),
        ]
        .align_y(Alignment::Center);

        let head_badge = row![
            text("☀️ KIN:").size(10).color(MayaColors::GLYPH_DIM),
            text(format!(" {}", self.head_sha))
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
        ]
        .align_y(Alignment::Center);

        let purity_badge = if self.is_pristine {
            text("✨ Codex: Pristine")
                .size(11)
                .color(MayaColors::JADE_PRIME)
        } else {
            text("⚠️ Codex: Altered")
                .size(11)
                .color(MayaColors::CINNABAR_BRIGHT)
        };

        let ixchel_badge = row![
            text("🌙 IXCHEL:").size(10).color(MayaColors::GLYPH_DIM),
            text(format!(" {}", self.ixchel_status))
                .size(11)
                .color(MayaColors::MAYA_BLUE),
        ]
        .align_y(Alignment::Center);

        let solar_cycle = format!("⏳ Kin Cycle: {:.1}s", self.cycle_time);
        let cycle_text = text(solar_cycle).size(10).color(MayaColors::GLYPH_DIM);

        let content = row![
            branch_badge,
            Space::new().width(16),
            head_badge,
            Space::new().width(16),
            purity_badge,
            Space::new().width(Length::Fill),
            ixchel_badge,
            Space::new().width(16),
            cycle_text,
        ]
        .padding(Padding::from([4, 14]))
        .align_y(Alignment::Center);

        container(content)
            .width(Length::Fill)
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
}
