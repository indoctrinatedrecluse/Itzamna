//! ItzamnaApp — Supreme Mayan Application Orchestrator (Phase 2: The Stelae Palace)

use iced::widget::{column, container, image, row, stack, text, Space};
use iced::{Alignment, ContentFit, Element, Length, Subscription, Task};
use std::time::Duration;

use crate::canvas::{BackgroundMode, CosmicLoom};
use crate::theme::MayaColors;
use crate::ui::{
    BottomProphecy, CenterLoom, CenterLoomMessage, GlyphRibbon, LeftCodex, LeftCodexMessage,
    MenuCategory, ProphecyLevel, ProphecyMessage, RightAltar, RightAltarMessage, StatusBar,
    TopMenuBar, TopMenuMessage,
};

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    TopMenu(TopMenuMessage),
    LeftCodex(LeftCodexMessage),
    CenterLoom(CenterLoomMessage),
    RightAltar(RightAltarMessage),
    Prophecy(ProphecyMessage),
}

pub struct ItzamnaApp {
    loom: CosmicLoom,
    glyph_ribbon: GlyphRibbon,
    top_menu: TopMenuBar,
    left_codex: LeftCodex,
    center_loom: CenterLoom,
    right_altar: RightAltar,
    bottom_prophecy: BottomProphecy,
    status_bar: StatusBar,
    background_mode: BackgroundMode,
    static_bg_handle: Option<image::Handle>,
}

impl Default for ItzamnaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ItzamnaApp {
    pub fn new() -> Self {
        let static_bg_handle = if std::path::Path::new("itzamna-desktop/assets/background.png").exists() {
            Some(image::Handle::from_path("itzamna-desktop/assets/background.png"))
        } else if std::path::Path::new("assets/background.png").exists() {
            Some(image::Handle::from_path("assets/background.png"))
        } else {
            None
        };

        Self {
            loom: CosmicLoom::new(),
            glyph_ribbon: GlyphRibbon::new(),
            top_menu: TopMenuBar::new(),
            left_codex: LeftCodex::new(),
            center_loom: CenterLoom::new(),
            right_altar: RightAltar::new(),
            bottom_prophecy: BottomProphecy::new(),
            status_bar: StatusBar::default(),
            background_mode: BackgroundMode::ProceduralLoom,
            static_bg_handle,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        // Ticks when either in Procedural or Video motion modes
        match self.background_mode {
            BackgroundMode::ProceduralLoom | BackgroundMode::VideoLoop => {
                iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
            }
            BackgroundMode::StaticArtwork | BackgroundMode::StealthObsidian => {
                // Ticks at low rate for ribbon and status clock
                iced::time::every(Duration::from_millis(32)).map(|_| Message::Tick)
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                let dt = 0.016;
                if self.background_mode == BackgroundMode::ProceduralLoom {
                    self.loom.tick(dt);
                } else if self.background_mode == BackgroundMode::VideoLoop {
                    self.loom.tick(dt * 2.2); // Accelerated celestial motion loop
                }

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
                        let desc = match cat {
                            MenuCategory::PopolVuh => "Popol Vuh: Council of Creation (File Menu opened)",
                            MenuCategory::ChilamBalam => "Chilam Balam: Inscription Settings (Edit Menu opened)",
                            MenuCategory::CosmicVision => "Cosmic Vision: Perspective & Themes (View Menu opened)",
                            MenuCategory::Yaxche => "Yaxche: Branches & World Tree (Branch Menu opened)",
                            MenuCategory::SacredAltar => "Sacred Altar: Stash & Staging Cist (Altar Menu opened)",
                            MenuCategory::IxchelOracle => "Ixchel Oracle: Divination Engine (Oracle Menu opened)",
                        };
                        self.bottom_prophecy.push("22:51:00", "📜", desc.into(), ProphecyLevel::Solar);
                    }
                    Task::none()
                }
                TopMenuMessage::CloseMenu => {
                    self.top_menu.active_menu = None;
                    Task::none()
                }
                TopMenuMessage::CycleBackgroundMode => {
                    self.background_mode = self.background_mode.next();
                    self.top_menu.background_mode = self.background_mode;
                    let msg = format!("Loom Mode Shifted: {}", self.background_mode.description());
                    self.bottom_prophecy.push("22:51:00", "✨", msg, ProphecyLevel::Jade);
                    Task::none()
                }
                TopMenuMessage::OpenCodex => {
                    self.bottom_prophecy.push("22:51:00", "📂", "Invoked Open Codex Dialog".into(), ProphecyLevel::Maya);
                    Task::none()
                }
                TopMenuMessage::NewInscription => {
                    self.bottom_prophecy.push("22:51:00", "🏛️", "Founding New Git Sanctuary in current path".into(), ProphecyLevel::Jade);
                    Task::none()
                }
                TopMenuMessage::FetchPulse => {
                    self.bottom_prophecy.push("22:51:00", "⚡", "Sent Solar Pulse to Remote Horizon (git fetch origin)".into(), ProphecyLevel::Solar);
                    Task::none()
                }
            },
            Message::LeftCodex(codex_msg) => match codex_msg {
                LeftCodexMessage::ToggleCollapse => {
                    self.left_codex.is_collapsed = !self.left_codex.is_collapsed;
                    Task::none()
                }
                LeftCodexMessage::SelectBranch(b) => {
                    self.left_codex.active_branch = b.clone();
                    self.status_bar.active_branch = b.clone();
                    self.bottom_prophecy.push("22:51:00", "🌿", format!("Switched sovereign branch to '{}'", b), ProphecyLevel::Jade);
                    Task::none()
                }
                LeftCodexMessage::SelectTag(t) => {
                    self.bottom_prophecy.push("22:51:00", "🏷️", format!("Inspecting solar tag: {}", t), ProphecyLevel::Solar);
                    Task::none()
                }
                LeftCodexMessage::SelectStash(idx) => {
                    self.bottom_prophecy.push("22:51:00", "🏺", format!("Inspecting sacred stash cist: {}", idx), ProphecyLevel::Maya);
                    Task::none()
                }
                LeftCodexMessage::SproutBranch => {
                    self.bottom_prophecy.push("22:51:00", "🌱", "Sprouting new branch from HEAD...".into(), ProphecyLevel::Jade);
                    Task::none()
                }
                LeftCodexMessage::FetchRemote => {
                    self.bottom_prophecy.push("22:51:00", "🌐", "Synchronizing remote horizon...".into(), ProphecyLevel::Maya);
                    Task::none()
                }
                LeftCodexMessage::CreateStash => {
                    self.bottom_prophecy.push("22:51:00", "🔒", "Sheltering altered working tree into sacred cist".into(), ProphecyLevel::Maya);
                    Task::none()
                }
            },
            Message::CenterLoom(loom_msg) => match loom_msg {
                CenterLoomMessage::SelectCommit(sha) => {
                    self.center_loom.selected_sha = Some(sha.clone());
                    self.status_bar.head_sha = sha.clone();
                    self.bottom_prophecy.push("22:51:00", "☀️", format!("Focused commit kin signet: {}", sha), ProphecyLevel::Solar);
                    Task::none()
                }
                CenterLoomMessage::FilterChanged(_f) => Task::none(),
            },
            Message::RightAltar(altar_msg) => match altar_msg {
                RightAltarMessage::ToggleCollapse => {
                    self.right_altar.is_collapsed = !self.right_altar.is_collapsed;
                    Task::none()
                }
                RightAltarMessage::StageFile(path) => {
                    if let Some(pos) = self.right_altar.unstaged_files.iter().position(|f| f.path == path) {
                        let item = self.right_altar.unstaged_files.remove(pos);
                        self.right_altar.staged_files.push(item);
                        self.bottom_prophecy.push("22:51:00", "✨", format!("Offered file to altar: {}", path), ProphecyLevel::Jade);
                    }
                    self.status_bar.is_pristine = self.right_altar.unstaged_files.is_empty();
                    Task::none()
                }
                RightAltarMessage::UnstageFile(path) => {
                    if let Some(pos) = self.right_altar.staged_files.iter().position(|f| f.path == path) {
                        let item = self.right_altar.staged_files.remove(pos);
                        self.right_altar.unstaged_files.push(item);
                        self.bottom_prophecy.push("22:51:00", "↩️", format!("Withdrew file from altar: {}", path), ProphecyLevel::Maya);
                    }
                    self.status_bar.is_pristine = self.right_altar.unstaged_files.is_empty();
                    Task::none()
                }
                RightAltarMessage::StageAll => {
                    let mut all = std::mem::take(&mut self.right_altar.unstaged_files);
                    self.right_altar.staged_files.append(&mut all);
                    self.bottom_prophecy.push("22:51:00", "✨", "All offerings consecrated to the Altar".into(), ProphecyLevel::Jade);
                    self.status_bar.is_pristine = true;
                    Task::none()
                }
                RightAltarMessage::UnstageAll => {
                    let mut all = std::mem::take(&mut self.right_altar.staged_files);
                    self.right_altar.unstaged_files.append(&mut all);
                    self.bottom_prophecy.push("22:51:00", "↩️", "All offerings returned to working tree".into(), ProphecyLevel::Maya);
                    self.status_bar.is_pristine = self.right_altar.unstaged_files.is_empty();
                    Task::none()
                }
                RightAltarMessage::DiscardAll => {
                    self.bottom_prophecy.push("22:51:00", "⚠️", "Purification ceremony performed (changes discarded)".into(), ProphecyLevel::Xibalba);
                    Task::none()
                }
                RightAltarMessage::CommitSummaryChanged(s) => {
                    self.right_altar.commit_summary = s;
                    Task::none()
                }
                RightAltarMessage::InscribeCommit => {
                    let msg = self.right_altar.commit_summary.clone();
                    self.bottom_prophecy.push("22:51:00", "🔥", format!("Sealed in stone: '{}'", msg), ProphecyLevel::Solar);
                    self.right_altar.staged_files.clear();
                    self.status_bar.head_sha = "5a9e312".to_string();
                    Task::none()
                }
            },
            Message::Prophecy(p_msg) => match p_msg {
                ProphecyMessage::ToggleDrawer => {
                    self.bottom_prophecy.is_expanded = !self.bottom_prophecy.is_expanded;
                    Task::none()
                }
                ProphecyMessage::ClearFeed => {
                    self.bottom_prophecy.entries.clear();
                    Task::none()
                }
            },
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        // 1. Layer 0: The Dynamic Background
        let background_layer: Element<'_, Message> = match self.background_mode {
            BackgroundMode::ProceduralLoom | BackgroundMode::VideoLoop => {
                self.loom.view(1920.0, 1080.0)
            }
            BackgroundMode::StaticArtwork => {
                if let Some(handle) = &self.static_bg_handle {
                    image(handle.clone())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(ContentFit::Cover)
                        .into()
                } else {
                    container(
                        text("🖼️ Place your Google Flow background in assets/background.png")
                            .size(13)
                            .color(MayaColors::GLYPH_DIM),
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .align_x(Alignment::Center)
                    .align_y(Alignment::Center)
                    .into()
                }
            }
            BackgroundMode::StealthObsidian => container(Space::new())
                .width(Length::Fill)
                .height(Length::Fill)
                .style(|_theme| container::Style {
                    background: Some(MayaColors::OBSIDIAN_VOID.into()),
                    ..Default::default()
                })
                .into(),
        };

        // 2. Layer 1: The Stelae Palace Quadrants
        let top_navigation = self.top_menu.view().map(Message::TopMenu);
        let ribbon_canvas: Element<'_, Message> = self.glyph_ribbon.view(1920.0, 40.0);

        // Center Quadrants: Left Codex + Center Timeline + Right Altar
        let left_codex_view = self.left_codex.view().map(Message::LeftCodex);
        let center_loom_view = self.center_loom.view().map(Message::CenterLoom);
        let right_altar_view = self.right_altar.view().map(Message::RightAltar);

        let central_quadrants = row![
            left_codex_view,
            center_loom_view,
            right_altar_view,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        let prophecy_tray = self.bottom_prophecy.view().map(Message::Prophecy);
        let status_bar_view = self.status_bar.view();

        let foreground = column![
            top_navigation,
            ribbon_canvas,
            central_quadrants,
            prophecy_tray,
            status_bar_view,
        ]
        .width(Length::Fill)
        .height(Length::Fill);

        stack![
            background_layer,
            foreground,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
