//! The Ixchel Oracle Radar (El Radar de Ixchel)
//! Real-time telemetry dashboard presenting DORA metrics, commit velocity, and contributor graph,
//! directly bridged to the Ixchel Scala 3 / ZIO 2 backend.

use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};
use serde::Deserialize;

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum OracleMessage {
    RefreshTelemetry,
    TriggerDeepSync,
    TelemetryReceived(Result<RepoHealthOverviewJson, String>),
}

// --- Ixchel JSON DTOs ---

#[derive(Debug, Clone, Deserialize)]
pub struct DoraMetricsJson {
    #[serde(rename = "deploymentFrequency")]
    pub deployment_frequency: f64,
    #[serde(rename = "leadTimeHours")]
    pub lead_time_hours: f64,
    #[serde(rename = "changeFailureRate")]
    pub change_failure_rate: f64,
    #[serde(rename = "mttrMinutes")]
    pub mttr_minutes: f64,
    pub tier: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChurnMetricsJson {
    #[serde(rename = "linesAdded")]
    pub lines_added: u64,
    #[serde(rename = "linesDeleted")]
    pub lines_deleted: u64,
    #[serde(rename = "linesModified")]
    pub lines_modified: u64,
    #[serde(rename = "netGrowth")]
    pub net_growth: i64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ContributorInsightJson {
    pub name: String,
    pub email: String,
    #[serde(rename = "commitCount")]
    pub commit_count: usize,
    #[serde(rename = "ownershipPercent")]
    pub ownership_percent: f64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BusFactorJson {
    pub score: usize,
    #[serde(rename = "riskLevel")]
    pub risk_level: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RepoHealthOverviewJson {
    #[serde(rename = "healthScore")]
    pub health_score: u32,
    pub dora: DoraMetricsJson,
    pub churn: ChurnMetricsJson,
    #[serde(rename = "topContributors")]
    pub top_contributors: Vec<ContributorInsightJson>,
    #[serde(rename = "busFactor")]
    pub bus_factor: BusFactorJson,
}

pub struct OracleRadar {
    pub is_connected: bool,
    pub server_url: String,
    pub health_score: u32,
    pub deployment_frequency: String,
    pub lead_time: String,
    pub change_failure_rate: String,
    pub mttr: String,
    pub total_insertions: u64,
    pub total_deletions: u64,
    pub top_contributor: String,
    pub bus_factor: usize,
    pub bus_factor_risk: String,
    pub contributors: Vec<ContributorInsightJson>,
    pub status_text: String,
}

impl Default for OracleRadar {
    fn default() -> Self {
        Self::new()
    }
}

impl OracleRadar {
    pub fn new() -> Self {
        Self {
            is_connected: false,
            server_url: "http://127.0.0.1:9090".to_string(),
            health_score: 92,
            deployment_frequency: "1.5 / day (Elite)".to_string(),
            lead_time: "2.4 Hours".to_string(),
            change_failure_rate: "2.4%".to_string(),
            mttr: "18.0 Mins".to_string(),
            total_insertions: 2865,
            total_deletions: 465,
            top_contributor: "Ahau Architect".to_string(),
            bus_factor: 2,
            bus_factor_risk: "Moderate".to_string(),
            contributors: Vec::new(),
            status_text: "Standby — Connecting to Ixchel Scala/ZIO Conduit...".to_string(),
        }
    }

    pub fn apply_telemetry(&mut self, data: RepoHealthOverviewJson) {
        self.is_connected = true;
        self.health_score = data.health_score;
        self.deployment_frequency = format!("{:.1} / day ({})", data.dora.deployment_frequency, data.dora.tier);
        self.lead_time = format!("{:.1} Hours", data.dora.lead_time_hours);
        self.change_failure_rate = format!("{:.1}%", data.dora.change_failure_rate);
        self.mttr = format!("{:.1} Mins", data.dora.mttr_minutes);
        self.total_insertions = data.churn.lines_added;
        self.total_deletions = data.churn.lines_deleted;
        if let Some(top) = data.top_contributors.first() {
            self.top_contributor = top.name.clone();
        }
        self.bus_factor = data.bus_factor.score;
        self.bus_factor_risk = data.bus_factor.risk_level.clone();
        self.contributors = data.top_contributors;
        self.status_text = "Celestial Conduit Synced via Ixchel (127.0.0.1:9090)".to_string();
    }

    pub fn mark_offline(&mut self, err: &str) {
        self.is_connected = false;
        self.status_text = format!("Conduit Dormant: {}", err);
    }

    pub fn view<'a, Message: 'a + From<OracleMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let (status_color, status_badge_bg, status_badge_text) = if self.is_connected {
            (MayaColors::JADE_GLOW, MayaColors::JADE_GLOW, "🌕 CONDUIT ONLINE: 127.0.0.1:9090")
        } else {
            (MayaColors::CINNABAR_BRIGHT, MayaColors::CINNABAR_DARK, "🌑 CONDUIT DORMANT")
        };

        let header = row![
            text("🔮 IXCHEL ORACLE RADAR")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            container(text(status_badge_text).size(9).color(MayaColors::OBSIDIAN_VOID))
                .padding(Padding::from([2, 8]))
                .style(move |_theme| container::Style {
                    background: Some(status_badge_bg.into()),
                    border: Border {
                        radius: 3.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            Space::new().width(12),
            text(format!("Celestial Health: {} / 100", self.health_score))
                .size(10)
                .color(status_color),
            Space::new().width(Length::Fill),
            button(text("⚡ Trigger Sync Ritual").size(10).color(MayaColors::MAYA_BLUE))
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
                .on_press(OracleMessage::TriggerDeepSync.into()),
            Space::new().width(6),
            button(text("🔄 Refresh Telemetry").size(10).color(MayaColors::SOLAR_GOLD))
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
                .on_press(OracleMessage::RefreshTelemetry.into()),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        // 1. DORA Metrics Quad-Cards
        let dora_cards = row![
            self.metric_card("DEPLOY FREQUENCY", &self.deployment_frequency, "Daily releases", MayaColors::JADE_GLOW),
            Space::new().width(8),
            self.metric_card("LEAD TIME", &self.lead_time, "Commit to main", MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            self.metric_card("FAILURE RATE", &self.change_failure_rate, "Pristine builds", MayaColors::MAYA_BLUE),
            Space::new().width(8),
            self.metric_card("TIME TO RESTORE", &self.mttr, "Mean recovery", MayaColors::SOLAR_AMBER),
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
        value: &str,
        subtext: &'static str,
        accent: Color,
    ) -> Element<'a, Message> {
        let content = column![
            text(title).size(9).color(MayaColors::GLYPH_DIM),
            Space::new().height(4),
            text(value.to_string()).size(14).color(accent),
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
        let title = text("📊 CODE VELOCITY & CHURN DISTRIBUTION (MINED BY IXCHEL)")
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
                .width(Length::FillPortion(85))
                .height(6)
                .style(|_| container::Style { background: Some(MayaColors::JADE_GLOW.into()), ..Default::default() }),
            container(Space::new())
                .width(Length::FillPortion(10))
                .height(6)
                .style(|_| container::Style { background: Some(MayaColors::MAYA_BLUE.into()), ..Default::default() }),
            container(Space::new())
                .width(Length::FillPortion(5))
                .height(6)
                .style(|_| container::Style { background: Some(MayaColors::SOLAR_AMBER.into()), ..Default::default() }),
        ];

        let legend = row![
            text("■ Rust Desktop (85%)").size(9).color(MayaColors::JADE_GLOW),
            Space::new().width(12),
            text("■ Scala 3 Backend (10%)").size(9).color(MayaColors::MAYA_BLUE),
            Space::new().width(12),
            text("■ WGSL / Docs (5%)").size(9).color(MayaColors::SOLAR_AMBER),
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
        let title = text("👑 SOVEREIGN AUTHORS & BUS FACTOR MATRIX")
            .size(10)
            .color(MayaColors::SOLAR_GOLD);

        let bus_color = match self.bus_factor_risk.as_str() {
            "Low" => MayaColors::JADE_GLOW,
            "Moderate" => MayaColors::SOLAR_AMBER,
            _ => MayaColors::CINNABAR_BRIGHT,
        };

        let bus_summary = row![
            text("Primary Architect: ").size(10).color(MayaColors::GLYPH_DIM),
            text(&self.top_contributor).size(11).color(MayaColors::STARLIGHT_WHITE),
            Space::new().width(16),
            text("Bus Factor: ").size(10).color(MayaColors::GLYPH_DIM),
            text(format!("{}", self.bus_factor)).size(11).color(bus_color),
            Space::new().width(12),
            text(format!("Risk: {}", self.bus_factor_risk)).size(10).color(bus_color),
        ]
        .align_y(Alignment::Center);

        let mut contrib_rows = column![].spacing(6);
        for c in self.contributors.iter().take(4) {
            contrib_rows = contrib_rows.push(
                row![
                    text(format!("👤 {}", c.name)).size(10).color(MayaColors::STARLIGHT_WHITE).width(Length::FillPortion(3)),
                    text(format!("{} commits", c.commit_count)).size(9).color(MayaColors::GLYPH_DIM).width(Length::FillPortion(2)),
                    text(format!("{:.1}% ownership", c.ownership_percent)).size(9).color(MayaColors::JADE_GLOW).width(Length::FillPortion(2)),
                ]
                .align_y(Alignment::Center)
                .padding(Padding::from([3, 6]))
            );
        }

        container(
            column![
                title,
                Space::new().height(6),
                bus_summary,
                Space::new().height(10),
                contrib_rows,
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
