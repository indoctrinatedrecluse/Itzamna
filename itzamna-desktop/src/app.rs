//! ItzamnaApp — Supreme Mayan Application Orchestrator

use iced::widget::{button, column, container, row, stack, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding, Subscription, Task};
use std::time::Duration;

use crate::canvas::CosmicLoom;
use crate::theme::MayaColors;
use crate::ui::{GlyphRibbon, MenuCategory, StatusBar, TopMenuBar, TopMenuMessage};

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    TopMenu(TopMenuMessage),
    OpenCodexDialog,
    InitNewKingdom,
    CloneChronicle,
}

pub struct ItzamnaApp {
    loom: CosmicLoom,
    glyph_ribbon: GlyphRibbon,
    top_menu: TopMenuBar,
    status_bar: StatusBar,
    active_repo_path: Option<String>,
    status_notification: Option<String>,
}

impl Default for ItzamnaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ItzamnaApp {
    pub fn new() -> Self {
        Self {
            loom: CosmicLoom::new(),
            glyph_ribbon: GlyphRibbon::new(),
            top_menu: TopMenuBar::new(),
            status_bar: StatusBar::default(),
            active_repo_path: Some("D:\\Projects\\Itzamna".to_string()),
            status_notification: Some("Obsidian Loom Initialized · Phase 1 Online".to_string()),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Continuous 60fps frame ticker for ambient background and looping glyph marquee
        if self.top_menu.cosmic_loom_active {
            iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                let dt = 0.016;
                self.loom.tick(dt);
                self.glyph_ribbon.tick(dt);
                self.status_bar.tick(dt);
                Task::none()
            }
            Message::TopMenu(menu_msg) => match menu_msg {
                TopMenuMessage::SelectMenu(cat) => {
                    if self.top_menu.active_menu == Some(cat) {
                        self.top_menu.active_menu = None;
                    } else {
                        self.top_menu.active_menu = Some(cat);
                        self.status_notification = Some(match cat {
                            MenuCategory::PopolVuh => "Popol Vuh: Council of Creation (File)".to_string(),
                            MenuCategory::ChilamBalam => "Chilam Balam: Sacred Inscriptions (Edit)".to_string(),
                            MenuCategory::CosmicVision => "Cosmic Vision: Celestial Perspective (View)".to_string(),
                            MenuCategory::Yaxche => "Yaxche: The Sacred World Tree (Branches)".to_string(),
                            MenuCategory::SacredAltar => "Sacred Altar: Stash & Staging Cist (Altar)".to_string(),
                            MenuCategory::IxchelOracle => "Ixchel Oracle: Divination Engine (Analytics)".to_string(),
                        });
                    }
                    Task::none()
                }
                TopMenuMessage::CloseMenu => {
                    self.top_menu.active_menu = None;
                    Task::none()
                }
                TopMenuMessage::ToggleCosmicLoom => {
                    self.top_menu.cosmic_loom_active = !self.top_menu.cosmic_loom_active;
                    self.status_notification = Some(if self.top_menu.cosmic_loom_active {
                        "Cosmic Loom Reignited · Star Cycles Rotating".to_string()
                    } else {
                        "Cosmic Loom Paused in Stasis".to_string()
                    });
                    Task::none()
                }
                TopMenuMessage::OpenCodex => {
                    self.status_notification = Some("Invoking Open Codex Dialog...".to_string());
                    Task::none()
                }
                TopMenuMessage::NewInscription => {
                    self.status_notification = Some("Founding New Repository Sanctuary...".to_string());
                    Task::none()
                }
                TopMenuMessage::FetchPulse => {
                    self.status_notification = Some("Sending Solar Pulse to Remote Horizon (git fetch)...".to_string());
                    Task::none()
                }
            },
            Message::OpenCodexDialog => {
                self.status_notification = Some("Select an ancient repository to awaken...".to_string());
                Task::none()
            }
            Message::InitNewKingdom => {
                self.status_notification = Some("Initiating git init ceremony in current domain...".to_string());
                Task::none()
            }
            Message::CloneChronicle => {
                self.status_notification = Some("Channeling remote chronicle into local stone...".to_string());
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // 1. Layer 0: The Full-Window Animated Cosmic Loom Canvas
        let background_canvas: Element<'_, Message> = self.loom.view(1920.0, 1080.0);

        // 2. Layer 1: The UI Shell Overlay
        let top_navigation = self.top_menu.view().map(Message::TopMenu);

        // Animated Mayan Cartouche Loop directly framing the menu
        let ribbon_canvas: Element<'_, Message> = self.glyph_ribbon.view(1920.0, 40.0);

        // Center Welcome Sanctuary / Altar Card
        let center_sanctuary = self.render_sanctuary_overlay();

        // Footer Obsidian Status Bar
        let footer_status = self.status_bar.view();

        // Composite Foreground Layout
        let foreground = column![
            top_navigation,
            ribbon_canvas,
            Space::new().height(Length::FillPortion(1)),
            row![
                Space::new().width(Length::FillPortion(1)),
                center_sanctuary,
                Space::new().width(Length::FillPortion(1)),
            ]
            .align_y(Alignment::Center),
            Space::new().height(Length::FillPortion(1)),
            footer_status,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        // Combine into translucent stack
        stack![
            background_canvas,
            foreground,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    fn render_sanctuary_overlay<'a>(&'a self) -> Element<'a, Message> {
        let repo_display = self
            .active_repo_path
            .as_deref()
            .unwrap_or("No Codex Awakened");

        let header = column![
            text("☀️ ITZAMNÁ STUDIO")
                .size(24)
                .color(MayaColors::SOLAR_GOLD),
            text("SUPREME MAYAN GIT COCKPIT · PHASE 1")
                .size(11)
                .color(MayaColors::JADE_GLOW),
        ]
        .spacing(2)
        .align_x(Alignment::Center);

        let quote = text(
            "\"This is the account of when all was in suspense, all calm, in silence;\nall motionless, quiet, and the expanse of the sky was empty.\"\n— Popol Vuh",
        )
        .size(11)
        .color(MayaColors::PARCHMENT_MUTED);

        let active_repo_badge = row![
            text("CURRENT SANCTUARY: ").size(10).color(MayaColors::GLYPH_DIM),
            text(repo_display).size(11).color(MayaColors::STARLIGHT_WHITE),
        ]
        .align_y(Alignment::Center);

        let action_buttons = row![
            self.action_button("📜 Open Codex", Message::OpenCodexDialog, MayaColors::JADE_GLOW),
            self.action_button("🏛️ Found Kingdom (Init)", Message::InitNewKingdom, MayaColors::SOLAR_GOLD),
            self.action_button("🌐 Channel Remote (Clone)", Message::CloneChronicle, MayaColors::MAYA_BLUE),
        ]
        .spacing(12)
        .align_y(Alignment::Center);

        let notification_text = if let Some(notif) = &self.status_notification {
            text(notif).size(11).color(MayaColors::SOLAR_AMBER)
        } else {
            text("All 20 Sacred Glyphs Orbiting · Loom Aligned").size(11).color(MayaColors::GLYPH_DIM)
        };

        let card_content = column![
            header,
            Space::new().height(14),
            quote,
            Space::new().height(16),
            active_repo_badge,
            Space::new().height(18),
            action_buttons,
            Space::new().height(14),
            notification_text,
        ]
        .spacing(4)
        .padding(Padding::from([26, 32]))
        .align_x(Alignment::Center);

        container(card_content)
            .width(620)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.85, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::JADE_SHADOW,
                    width: 1.5,
                    radius: 8.0.into(),
                },
                shadow: iced::Shadow {
                    color: Color { a: 0.5, ..Color::BLACK },
                    offset: iced::Vector::new(0.0, 10.0),
                    blur_radius: 28.0,
                },
                ..Default::default()
            })
            .into()
    }

    fn action_button<'a>(
        &'a self,
        label: &'static str,
        on_press: Message,
        accent: Color,
    ) -> Element<'a, Message> {
        button(text(label).size(12).color(MayaColors::STARLIGHT_WHITE))
            .padding(Padding::from([8, 16]))
            .style(move |_theme, status| {
                let border_color = match status {
                    button::Status::Hovered => accent,
                    _ => MayaColors::OBSIDIAN_BORDER,
                };
                let bg_color = match status {
                    button::Status::Hovered => Color { a: 0.9, ..MayaColors::OBSIDIAN_SURFACE },
                    _ => Color { a: 0.6, ..MayaColors::OBSIDIAN_SURFACE },
                };

                button::Style {
                    background: Some(bg_color.into()),
                    text_color: MayaColors::STARLIGHT_WHITE,
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            })
            .on_press(on_press)
            .into()
    }
}
