//! The Overseer Portals (Los Portales de los Vigilantes — Remote Operations Chamber)
//! Manage Git remotes, Push, Push with Force-with-lease, Pull, and Remote Prune.

use iced::widget::{button, column, container, row, text, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum PortalMessage {
    RemoteUrlChanged(String),
    PushSovereign,
    PushForceWithLease,
    PullRebase,
    FetchPrune,
}

pub struct OverseerPortalsWorkbench {
    pub remotes: Vec<(String, String)>,
    pub new_remote_url: String,
    pub active_branch: String,
    pub last_action_report: Option<String>,
}

impl Default for OverseerPortalsWorkbench {
    fn default() -> Self {
        Self::new("main".to_string(), vec![("origin".to_string(), "https://github.com/indoctrinatedrecluse/Itzamna.git".to_string())])
    }
}

impl OverseerPortalsWorkbench {
    pub fn new(active_branch: String, remotes: Vec<(String, String)>) -> Self {
        Self {
            remotes,
            new_remote_url: "".to_string(),
            active_branch,
            last_action_report: None,
        }
    }

    pub fn set_report(&mut self, report: String) {
        self.last_action_report = Some(report);
    }

    pub fn view<'a, Message: 'a + From<PortalMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_bar = row![
            text("🌐 LOS PORTALES DE LOS VIGILANTES (REMOTE OPERATIONS)")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(12),
            text(format!("{} Configured Portals · Active HEAD: '{}'", self.remotes.len(), self.active_branch))
                .size(10)
                .color(MayaColors::MAYA_BLUE),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        let report_banner = if let Some(ref rep) = self.last_action_report {
            container(
                row![
                    text("⚡ ").size(11),
                    text(rep).size(11).color(MayaColors::SOLAR_AMBER),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([6, 12]))
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.35, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                ..Default::default()
            })
        } else {
            container(Space::new())
        };

        let remotes_section = self.render_remotes_list();
        let actions_section = self.render_actions_grid();

        let body = column![
            report_banner,
            Space::new().height(8),
            remotes_section,
            Space::new().height(14),
            actions_section,
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

    fn render_remotes_list<'a, Message: 'a + From<PortalMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title = row![
            text("☁️ RECOGNISED HORIZONS (REMOTES)").size(11).color(MayaColors::MAYA_BLUE),
            Space::new().width(8),
            text("Configured remote repository endpoints").size(10).color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        let mut list = column![].spacing(4);

        for (name, url) in &self.remotes {
            let item = row![
                text("🌐").size(11),
                Space::new().width(6),
                text(name).size(11).color(MayaColors::SOLAR_GOLD),
                Space::new().width(12),
                text(url).size(10).color(MayaColors::STARLIGHT_WHITE),
            ]
            .align_y(Alignment::Center)
            .padding(Padding::from([4, 8]));

            list = list.push(
                container(item)
                    .width(Length::Fill)
                    .style(|_theme| container::Style {
                        background: Some(Color { a: 0.20, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                        border: Border {
                            color: MayaColors::OBSIDIAN_BORDER,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
            );
        }

        container(
            column![
                title,
                Space::new().height(6),
                list,
            ]
            .padding(Padding::from([10, 14]))
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

    fn render_actions_grid<'a, Message: 'a + From<PortalMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title = row![
            text("⚡ CEREMONIAL ACTIONS").size(11).color(MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            text("Synchronize with remote Git realms").size(10).color(MayaColors::GLYPH_DIM),
        ]
        .align_y(Alignment::Center);

        let push_btn = tooltip(
            button(
                row![
                    text("⬆️ PUSH HEAD").size(11).color(MayaColors::OBSIDIAN_VOID),
                    Space::new().width(4),
                    text("(git push)").size(9).color(Color { a: 0.8, ..MayaColors::OBSIDIAN_VOID }),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([7, 14]))
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => MayaColors::JADE_PRIME,
                    _ => MayaColors::JADE_GLOW,
                };
                button::Style {
                    background: Some(bg.into()),
                    border: Border {
                        color: MayaColors::JADE_GLOW,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            })
            .on_press(PortalMessage::PushSovereign.into()),
            text("Push commits on active branch to origin (git push origin <branch>)").size(11),
            tooltip::Position::Top,
        );

        let push_lease_btn = tooltip(
            button(
                row![
                    text("🛡️ PUSH (FORCE-WITH-LEASE)").size(10).color(MayaColors::OBSIDIAN_VOID),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([7, 12]))
            .style(|_theme, status| {
                let bg = match status {
                    button::Status::Hovered => MayaColors::SOLAR_GOLD,
                    _ => MayaColors::SOLAR_AMBER,
                };
                button::Style {
                    background: Some(bg.into()),
                    border: Border {
                        color: MayaColors::SOLAR_GOLD,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            })
            .on_press(PortalMessage::PushForceWithLease.into()),
            text("Safely push rewritten history only if remote ref hasn't changed (--force-with-lease)").size(11),
            tooltip::Position::Top,
        );

        let pull_btn = tooltip(
            button(
                row![
                    text("⬇️ PULL & REBASE").size(11).color(MayaColors::STARLIGHT_WHITE),
                    Space::new().width(4),
                    text("(git pull -r)").size(9).color(MayaColors::GLYPH_DIM),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([7, 14]))
            .style(|_theme, _status| button::Style {
                background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                border: Border {
                    color: MayaColors::MAYA_BLUE,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .on_press(PortalMessage::PullRebase.into()),
            text("Fetch remote commits and rebase current branch onto remote tracking branch").size(11),
            tooltip::Position::Top,
        );

        let prune_btn = tooltip(
            button(
                row![
                    text("✂️ REMOTE PRUNE").size(10).color(MayaColors::PARCHMENT_MUTED),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([7, 12]))
            .style(|_theme, _status| button::Style {
                background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .on_press(PortalMessage::FetchPrune.into()),
            text("Delete stale remote-tracking branches that no longer exist on remote (git fetch --prune)").size(11),
            tooltip::Position::Top,
        );

        let action_row = row![
            push_btn,
            Space::new().width(8),
            push_lease_btn,
            Space::new().width(8),
            pull_btn,
            Space::new().width(8),
            prune_btn,
        ]
        .align_y(Alignment::Center);

        container(
            column![
                title,
                Space::new().height(10),
                action_row,
            ]
            .padding(Padding::from([10, 14]))
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
