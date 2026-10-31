//! The Council of Ahau (Top Menu Bar)
//! Mayan-themed navigation hierarchy framing the application window with
//! dynamic background mode switcher.

use iced::widget::{button, container, row, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::canvas::BackgroundMode;
use crate::theme::MayaColors;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuCategory {
    PopolVuh,    // File
    ChilamBalam, // Edit
    CosmicVision,// View
    Yaxche,      // Branches
    SacredAltar, // Stash / Stage
    IxchelOracle,// Analytics
}

#[derive(Debug, Clone)]
pub enum TopMenuMessage {
    SelectMenu(MenuCategory),
    CloseMenu,
    CycleBackgroundMode,
    OpenCodex,
    NewInscription,
    FetchPulse,
}

pub struct TopMenuBar {
    pub active_menu: Option<MenuCategory>,
    pub background_mode: BackgroundMode,
}

impl Default for TopMenuBar {
    fn default() -> Self {
        Self::new()
    }
}

impl TopMenuBar {
    pub fn new() -> Self {
        Self {
            active_menu: None,
            background_mode: BackgroundMode::ProceduralLoom,
        }
    }

    pub fn view<'a, Message: 'a + From<TopMenuMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let logo = row![
            text("🔱 ITZAMNÁ")
                .size(15)
                .color(MayaColors::SOLAR_GOLD),
            text(" · GIT STUDIO")
                .size(11)
                .color(MayaColors::JADE_GLOW),
        ]
        .align_y(Alignment::Center);

        let menu_buttons = row![
            self.menu_item("📜 Popol Vuh", MenuCategory::PopolVuh),
            self.menu_item("✒️ Chilam Balam", MenuCategory::ChilamBalam),
            self.menu_item("👁️ Cosmic Vision", MenuCategory::CosmicVision),
            self.menu_item("🌳 Yaxche", MenuCategory::Yaxche),
            self.menu_item("🏺 Sacred Altar", MenuCategory::SacredAltar),
            self.menu_item("🔮 Ixchel Oracle", MenuCategory::IxchelOracle),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let loom_color = match self.background_mode {
            BackgroundMode::ProceduralLoom => MayaColors::JADE_GLOW,
            BackgroundMode::StaticArtwork => MayaColors::SOLAR_GOLD,
            BackgroundMode::VideoLoop => MayaColors::MAYA_BLUE,
            BackgroundMode::StealthObsidian => MayaColors::PARCHMENT_MUTED,
        };

        let quick_actions = row![
            button(
                text(self.background_mode.label())
                    .size(11)
                    .color(loom_color)
            )
            .padding(Padding::from([4, 10]))
            .style(move |_theme, status| {
                let border_color = match status {
                    button::Status::Hovered => loom_color,
                    _ => MayaColors::OBSIDIAN_BORDER,
                };
                button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    text_color: MayaColors::STARLIGHT_WHITE,
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            })
            .on_press(TopMenuMessage::CycleBackgroundMode.into()),

            button(
                text("⚡ Fetch Pulse")
                    .size(11)
                    .color(MayaColors::SOLAR_AMBER)
            )
            .padding(Padding::from([4, 10]))
            .style(|_theme, status| {
                let border_color = match status {
                    button::Status::Hovered => MayaColors::SOLAR_GOLD,
                    _ => MayaColors::OBSIDIAN_BORDER,
                };
                button::Style {
                    background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                    text_color: MayaColors::SOLAR_AMBER,
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 4.0.into(),
                    },
                    ..Default::default()
                }
            })
            .on_press(TopMenuMessage::FetchPulse.into()),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let content = row![
            logo,
            Space::new().width(18),
            menu_buttons,
            Space::new().width(Length::Fill),
            quick_actions,
        ]
        .padding(Padding::from([6, 14]))
        .align_y(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.94, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn menu_item<'a, Message: 'a + From<TopMenuMessage> + Clone>(
        &'a self,
        label: &'static str,
        category: MenuCategory,
    ) -> Element<'a, Message> {
        let is_active = self.active_menu == Some(category);

        let txt_color = if is_active {
            MayaColors::SOLAR_GOLD
        } else {
            MayaColors::STARLIGHT_WHITE
        };

        button(text(label).size(12).color(txt_color))
            .padding(Padding::from([5, 9]))
            .style(move |_theme, status| {
                let (bg, border) = if is_active {
                    (
                        MayaColors::OBSIDIAN_SURFACE,
                        MayaColors::SOLAR_GOLD,
                    )
                } else {
                    match status {
                        button::Status::Hovered => (
                            MayaColors::OBSIDIAN_SURFACE,
                            MayaColors::JADE_GLOW,
                        ),
                        _ => (
                            Color::TRANSPARENT,
                            Color::TRANSPARENT,
                        ),
                    }
                };

                button::Style {
                    background: Some(bg.into()),
                    text_color: MayaColors::STARLIGHT_WHITE,
                    border: Border {
                        color: border,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    ..Default::default()
                }
            })
            .on_press(TopMenuMessage::SelectMenu(category).into())
            .into()
    }
}
