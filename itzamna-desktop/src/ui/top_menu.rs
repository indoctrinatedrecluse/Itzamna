//! The Council of Ahau (Top Menu Bar)
//! Mayan-themed navigation hierarchy framing the application window with
//! dynamic background mode switcher.

use iced::widget::{button, container, row, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::canvas::BackgroundMode;
use crate::theme::MayaColors;
use crate::ui::tooltip::mayan_tooltip;

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
    OpenCommandPalette,
    ToggleKeybindings,
    OpenProjectFolder,
    Pull,
    Push,
    OpenCredentials,
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
            // 1. Open Sanctuary (Project Folder)
            mayan_tooltip(
                button(
                    row![
                        text("📂").size(11),
                        Space::new().width(4),
                        text("Open Sanctuary").size(11).color(MayaColors::SOLAR_GOLD),
                        Space::new().width(4),
                        text("Ctrl+O").size(9).color(MayaColors::GLYPH_DIM),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([4, 9]))
                .style(|_theme, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::SOLAR_GOLD,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(TopMenuMessage::OpenProjectFolder.into()),
                "Open a Git repository from your local filesystem (Ctrl+O)",
                iced::widget::tooltip::Position::Bottom,
            ),

            // 2. Git Pull
            mayan_tooltip(
                button(
                    row![
                        text("⬇️").size(10),
                        Space::new().width(4),
                        text("Pull").size(11).color(MayaColors::JADE_GLOW),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([4, 8]))
                .style(|_theme, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::JADE_GLOW,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(TopMenuMessage::Pull.into()),
                "Pull and rebase commits from remote tracking branch (git pull --rebase)",
                iced::widget::tooltip::Position::Bottom,
            ),

            // 3. Git Push
            mayan_tooltip(
                button(
                    row![
                        text("⬆️").size(10),
                        Space::new().width(4),
                        text("Push").size(11).color(MayaColors::SOLAR_GOLD),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([4, 8]))
                .style(|_theme, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::SOLAR_GOLD,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(TopMenuMessage::Push.into()),
                "Push sovereign commits to remote horizon (git push origin)",
                iced::widget::tooltip::Position::Bottom,
            ),

            // 4. Git Fetch
            mayan_tooltip(
                button(
                    row![
                        text("🔄").size(10),
                        Space::new().width(4),
                        text("Fetch").size(11).color(MayaColors::MAYA_BLUE),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([4, 8]))
                .style(|_theme, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::MAYA_BLUE,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(TopMenuMessage::FetchPulse.into()),
                "Fetch remote branch updates and prune stale refs (git fetch --prune)",
                iced::widget::tooltip::Position::Bottom,
            ),

            // 5. Git Credentials
            mayan_tooltip(
                button(
                    row![
                        text("⚙️").size(10),
                        Space::new().width(4),
                        text("Identity").size(11).color(MayaColors::STARLIGHT_WHITE),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([4, 8]))
                .style(|_theme, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::SOLAR_GOLD,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(TopMenuMessage::OpenCredentials.into()),
                "Configure local repository Git credentials (user.name, user.email, remote URL)",
                iced::widget::tooltip::Position::Bottom,
            ),

            // 6. Command Palette
            mayan_tooltip(
                button(
                    row![
                        text("🔍").size(10),
                        Space::new().width(4),
                        text("Palette").size(11).color(MayaColors::MAYA_BLUE),
                        Space::new().width(4),
                        text("Ctrl+P").size(9).color(MayaColors::GLYPH_DIM),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([4, 8]))
                .style(|_theme, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::MAYA_BLUE,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(TopMenuMessage::OpenCommandPalette.into()),
                "Open Command Palette to search and execute all sacred rituals (Ctrl+P)",
                iced::widget::tooltip::Position::Bottom,
            ),

            // 7. Background Mode Cycle
            mayan_tooltip(
                button(
                    text(self.background_mode.label())
                        .size(11)
                        .color(loom_color)
                )
                .padding(Padding::from([4, 9]))
                .style(move |_theme, status| {
                    let border_color = match status {
                        button::Status::Hovered => loom_color,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        text_color: MayaColors::STARLIGHT_WHITE,
                        border: Border { color: border_color, width: 1.0, radius: 4.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(TopMenuMessage::CycleBackgroundMode.into()),
                "Cycle canvas background (Procedural / Stealth / Static / Video)",
                iced::widget::tooltip::Position::Bottom,
            ),
        ]
        .spacing(6)
        .align_y(Alignment::Center);

        let content = row![
            logo,
            Space::new().width(16),
            menu_buttons,
            Space::new().width(Length::Fill),
            quick_actions,
        ]
        .padding(Padding::from([6, 14]))
        .align_y(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.94, ..MayaColors::OBSIDIAN_BASE }.into()), // Solid opaque!
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

        let tooltip_desc = match category {
            MenuCategory::PopolVuh => "File Sanctuary: Refresh repository state from disk (git status)",
            MenuCategory::ChilamBalam => "Diff Chamber: Open granular Stone Diff workbench (git diff)",
            MenuCategory::CosmicVision => "Commit Loom: Focus central Yaxche commit DAG tree",
            MenuCategory::Yaxche => "Branch Crucible: Interactive rebase & conflict resolution (git rebase -i)",
            MenuCategory::SacredAltar => "Sacred Altar: Expand file staging & commit chamber (git add / commit)",
            MenuCategory::IxchelOracle => "Ixchel Oracle: Engineering telemetry & DORA metric radar",
        };

        let btn = button(text(label).size(12).color(txt_color))
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
            .on_press(TopMenuMessage::SelectMenu(category).into());

        mayan_tooltip(btn, tooltip_desc, iced::widget::tooltip::Position::Bottom)
    }
}
