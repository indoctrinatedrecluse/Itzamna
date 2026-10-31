//! The Ixchel Oracle Radar (El Radar de Ixchel)
//! Real-time telemetry dashboard presenting DORA metrics, commit velocity, and contributor graph.

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum OracleMessage {
    RefreshTelemetry,
    TriggerDeepSync,
}

pub struct OracleRadar {
    pub is_connected: bool,
    pub server_url: String,
    pub deployment_frequency: &'static str,
    pub lead_time: &'static str,
    pub change_failure_rate: &'static str,
    pub mttr: &'static str,
    pub total_insertions: usize,
    pub total_deletions: usize,
    pub top_contributor: &'static str,
    pub bus_factor: usize,
}

impl Default for OracleRadar {
    fn default() -> Self {
        Self::new()
    }
}

impl OracleRadar {
    pub fn new() -> Self {
        Self {
            is_connected: true,
            server_url: "http://127.0.0.1:7878".to_string(),
            deployment_frequency: "Daily (Elite)",
            lead_time: "2.4 Hours",
            change_failure_rate: "0.0%",
            mttr: "12 Minutes",
            total_insertions: 7840,
            total_deletions: 385,
            top_contributor: "indoctrinatedrecluse",
            bus_factor: 1,
        }
    }

    pub fn view<'a, Message: 'a + From<OracleMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let header = row![
            text("🔮 IXCHEL ORACLE RADAR")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            container(text("CONNECTED").size(9).color(MayaColors::OBSIDIAN_VOID))
                .padding(Padding::from([1, 6]))
                .style(|_theme| container::Style {
                    background: Some(MayaColors::JADE_GLOW.into()),
                    border: Border {
                        radius: 2.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            Space::new().width(Length::Fill),
            button(text("⚡ Trigger Sync").size(10).color(MayaColors::MAYA_BLUE))
                .padding(Padding::from([2, 8]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    border: Border {
                        color: MayaColors::MAYA_BLUE,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(OracleMessage::TriggerDeepSync.into()),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        // 1. DORA Metrics Quad-Cards
        let dora_cards = row![
            self.metric_card("DEPLOY FREQUENCY", self.deployment_frequency, "Daily pushes", MayaColors::JADE_GLOW),
            Space::new().width(8),
            self.metric_card("LEAD TIME", self.lead_time, "Commit to main", MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            self.metric_card("FAILURE RATE", self.change_failure_rate, "Pristine builds", MayaColors::MAYA_BLUE),
            Space::new().width(8),
            self.metric_card("TIME TO RESTORE", self.mttr, "Mean recovery", MayaColors::SOLAR_AMBER),
        ]
        .width(Length::Fill);

        // 2. Code Churn & Volume Section
        let churn_section = self.render_churn_section();

        // 3. Contributor & Bus Factor Section
        let contributor_section = self.render_contributor_section();

        let content = scrollable(
            column![
                dora_cards,
                Space::new().height(14),
                churn_section,
                Space::new().height(14),
                contributor_section,
            ]
            .padding(Padding::from([8, 12]))
        )
        .height(Length::Fill);

        container(column![header, content])
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

    fn metric_card<'a, Message: 'a>(
        &'a self,
        title: &'static str,
        value: &'static str,
        subtext: &'static str,
        accent: Color,
    ) -> Element<'a, Message> {
        let content = column![
            text(title).size(9).color(MayaColors::GLYPH_DIM),
            Space::new().height(4),
            text(value).size(15).color(accent),
            Space::new().height(2),
            text(subtext).size(9).color(MayaColors::PARCHMENT_MUTED),
        ]
        .padding(Padding::from([8, 10]));

        container(content)
            .width(Length::FillPortion(1))
            .style(move |_theme| container::Style {
                background: Some(Color { a: 0.75, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_churn_section<'a, Message: 'a>(&'a self) -> Element<'a, Message> {
        let title = text("📊 CODE VELOCITY & CHURN DISTRIBUTION")
            .size(10)
            .color(MayaColors::SOLAR_GOLD);

        let insertions_badge = row![
            text("Added: ").size(10).color(MayaColors::GLYPH_DIM),
            text(format!("+{} lines", self.total_insertions))
                .size(11)
                .color(MayaColors::JADE_GLOW),
        ]
        .align_y(Alignment::Center);

        let deletions_badge = row![
            text("Deleted: ").size(10).color(MayaColors::GLYPH_DIM),
            text(format!("-{} lines", self.total_deletions))
                .size(11)
                .color(MayaColors::CINNABAR_BRIGHT),
        ]
        .align_y(Alignment::Center);

        let ratio_bar = row![
            container(Space::new())
                .width(Length::FillPortion(88))
                .height(6)
                .style(|_| container::Style { background: Some(MayaColors::JADE_GLOW.into()), ..Default::default() }),
            container(Space::new())
                .width(Length::FillPortion(7))
                .height(6)
                .style(|_| container::Style { background: Some(MayaColors::MAYA_BLUE.into()), ..Default::default() }),
            container(Space::new())
                .width(Length::FillPortion(5))
                .height(6)
                .style(|_| container::Style { background: Some(MayaColors::SOLAR_AMBER.into()), ..Default::default() }),
        ];

        let legend = row![
            text("■ Rust (88%)").size(9).color(MayaColors::JADE_GLOW),
            Space::new().width(12),
            text("■ WGSL (7%)").size(9).color(MayaColors::MAYA_BLUE),
            Space::new().width(12),
            text("■ Markdown (5%)").size(9).color(MayaColors::SOLAR_AMBER),
        ]
        .align_y(Alignment::Center);

        container(
            column![
                title,
                Space::new().height(8),
                row![insertions_badge, Space::new().width(18), deletions_badge].align_y(Alignment::Center),
                Space::new().height(8),
                ratio_bar,
                Space::new().height(6),
                legend,
            ]
            .padding(Padding::from([10, 12]))
        )
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Color { a: 0.75, ..MayaColors::OBSIDIAN_SURFACE }.into()),
            border: Border {
                color: MayaColors::OBSIDIAN_BORDER,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn render_contributor_section<'a, Message: 'a>(&'a self) -> Element<'a, Message> {
        let title = text("👑 SOVEREIGN AUTHOR & BUS FACTOR")
            .size(10)
            .color(MayaColors::SOLAR_GOLD);

        let content = row![
            text("Sole Architect: ").size(10).color(MayaColors::GLYPH_DIM),
            text(self.top_contributor).size(11).color(MayaColors::STARLIGHT_WHITE),
            Space::new().width(16),
            text("Bus Factor: ").size(10).color(MayaColors::GLYPH_DIM),
            text(format!("{}", self.bus_factor)).size(11).color(MayaColors::SOLAR_AMBER),
            Space::new().width(16),
            text("Ownership: 100%").size(10).color(MayaColors::JADE_GLOW),
        ]
        .align_y(Alignment::Center);

        container(
            column![
                title,
                Space::new().height(6),
                content,
            ]
            .padding(Padding::from([10, 12]))
        )
        .width(Length::Fill)
        .style(|_theme| container::Style {
            background: Some(Color { a: 0.75, ..MayaColors::OBSIDIAN_SURFACE }.into()),
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
