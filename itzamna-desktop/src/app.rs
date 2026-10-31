//! ItzamnaApp — Supreme Mayan Application Orchestrator (Phase 3: The Tree of Life)

use iced::widget::{column, container, image, row, stack, text, Space};
use iced::{Alignment, ContentFit, Element, Length, Subscription, Task};
use std::time::Duration;

use crate::canvas::{BackgroundMode, CosmicLoom};
use crate::git::{FileStatus, GitSanctuary};
use crate::theme::MayaColors;
use crate::ui::{
    BottomProphecy, CenterLoom, CenterLoomMessage, FileEntry, GlyphRibbon, LeftCodex,
    LeftCodexMessage, MenuCategory, ProphecyLevel, ProphecyMessage, RightAltar, RightAltarMessage,
    StatusBar, TopMenuBar, TopMenuMessage,
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
    video_frame_index: usize,
    total_video_frames: usize,
}

impl Default for ItzamnaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ItzamnaApp {
    pub fn new() -> Self {
        // 1. Resolve background image (supports background.jpg or background.png)
        let static_bg_path = [
            "itzamna-desktop/assets/background.jpg",
            "assets/background.jpg",
            "itzamna-desktop/assets/background.png",
            "assets/background.png",
        ]
        .into_iter()
        .find(|p| std::path::Path::new(p).exists());

        let static_bg_handle = static_bg_path.map(image::Handle::from_path);

        // 2. Count available video loop frames
        let mut total_video_frames = 0;
        for i in 0..300 {
            let path = format!("itzamna-desktop/assets/video_frames/frame_{:03}.jpg", i);
            if std::path::Path::new(&path).exists() {
                total_video_frames += 1;
            } else {
                break;
            }
        }

        // 3. Open real Git repository
        let mut left_codex = LeftCodex::new();
        let mut right_altar = RightAltar::new();
        let mut status_bar = StatusBar::default();
        let mut center_loom = CenterLoom::new(Vec::new());
        let mut bottom_prophecy = BottomProphecy::new();

        if let Ok(sanctuary) = GitSanctuary::open(".") {
            let (head_branch, head_sha) = sanctuary.get_head_info();
            let (local_branches, remote_branches) = sanctuary.get_branches();
            let (unstaged, staged) = sanctuary.get_status_files();
            let commits = sanctuary.load_commit_dag(40);

            left_codex.active_branch = head_branch.clone();
            if !local_branches.is_empty() {
                left_codex.local_branches = local_branches;
            }
            if !remote_branches.is_empty() {
                left_codex.remote_branches = remote_branches;
            }

            status_bar.active_branch = head_branch;
            status_bar.head_sha = head_sha;
            status_bar.is_pristine = unstaged.is_empty();

            right_altar.unstaged_files = unstaged
                .into_iter()
                .map(|f: FileStatus| FileEntry {
                    path: f.path,
                    status: f.status_code,
                })
                .collect();

            right_altar.staged_files = staged
                .into_iter()
                .map(|f: FileStatus| FileEntry {
                    path: f.path,
                    status: f.status_code,
                })
                .collect();

            bottom_prophecy.push(
                "23:06:00",
                "🌳",
                format!("Awakened real Git repository: {} commits, {} unstaged files", commits.len(), right_altar.unstaged_files.len()),
                ProphecyLevel::Solar,
            );

            center_loom.set_commits(commits);
        }

        Self {
            loom: CosmicLoom::new(),
            glyph_ribbon: GlyphRibbon::new(),
            top_menu: TopMenuBar::new(),
            left_codex,
            center_loom,
            right_altar,
            bottom_prophecy,
            status_bar,
            background_mode: BackgroundMode::ProceduralLoom,
            static_bg_handle,
            video_frame_index: 0,
            total_video_frames,
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.background_mode {
            BackgroundMode::ProceduralLoom => {
                iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
            }
            BackgroundMode::VideoLoop => {
                // 24 FPS video playback loop
                iced::time::every(Duration::from_millis(41)).map(|_| Message::Tick)
            }
            BackgroundMode::StaticArtwork | BackgroundMode::StealthObsidian => {
                iced::time::every(Duration::from_millis(32)).map(|_| Message::Tick)
            }
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Tick => {
                let dt = 0.016;
                match self.background_mode {
                    BackgroundMode::ProceduralLoom => self.loom.tick(dt),
                    BackgroundMode::VideoLoop => {
                        if self.total_video_frames > 0 {
                            self.video_frame_index = (self.video_frame_index + 1) % self.total_video_frames;
                        }
                    }
                    _ => {}
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
                            MenuCategory::PopolVuh => "Popol Vuh: Council of Creation (File Menu)",
                            MenuCategory::ChilamBalam => "Chilam Balam: Inscription Settings (Edit Menu)",
                            MenuCategory::CosmicVision => "Cosmic Vision: Perspective & Themes (View Menu)",
                            MenuCategory::Yaxche => "Yaxche: Branches & World Tree (Branch Menu)",
                            MenuCategory::SacredAltar => "Sacred Altar: Stash & Staging Cist (Altar Menu)",
                            MenuCategory::IxchelOracle => "Ixchel Oracle: Divination Engine (Oracle Menu)",
                        };
                        self.bottom_prophecy.push("23:06:00", "📜", desc.into(), ProphecyLevel::Solar);
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
                    self.bottom_prophecy.push("23:06:00", "✨", msg, ProphecyLevel::Jade);
                    Task::none()
                }
                TopMenuMessage::OpenCodex => {
                    self.bottom_prophecy.push("23:06:00", "📂", "Invoked Open Codex Dialog".into(), ProphecyLevel::Maya);
                    Task::none()
                }
                TopMenuMessage::NewInscription => {
                    self.bottom_prophecy.push("23:06:00", "🏛️", "Founding New Git Sanctuary in current path".into(), ProphecyLevel::Jade);
                    Task::none()
                }
                TopMenuMessage::FetchPulse => {
                    self.bottom_prophecy.push("23:06:00", "⚡", "Sent Solar Pulse to Remote Horizon (git fetch origin)".into(), ProphecyLevel::Solar);
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
                    self.bottom_prophecy.push("23:06:00", "🌿", format!("Switched sovereign branch to '{}'", b), ProphecyLevel::Jade);
                    Task::none()
                }
                LeftCodexMessage::SelectTag(t) => {
                    self.bottom_prophecy.push("23:06:00", "🏷️", format!("Inspecting solar tag: {}", t), ProphecyLevel::Solar);
                    Task::none()
                }
                LeftCodexMessage::SelectStash(idx) => {
                    self.bottom_prophecy.push("23:06:00", "🏺", format!("Inspecting sacred stash cist: {}", idx), ProphecyLevel::Maya);
                    Task::none()
                }
                LeftCodexMessage::SproutBranch => {
                    self.bottom_prophecy.push("23:06:00", "🌱", "Sprouting new branch from HEAD...".into(), ProphecyLevel::Jade);
                    Task::none()
                }
                LeftCodexMessage::FetchRemote => {
                    self.bottom_prophecy.push("23:06:00", "🌐", "Synchronizing remote horizon...".into(), ProphecyLevel::Maya);
                    Task::none()
                }
                LeftCodexMessage::CreateStash => {
                    self.bottom_prophecy.push("23:06:00", "🔒", "Sheltering altered working tree into sacred cist".into(), ProphecyLevel::Maya);
                    Task::none()
                }
            },
            Message::CenterLoom(loom_msg) => match loom_msg {
                CenterLoomMessage::Dag(dag_msg) => match dag_msg {
                    crate::canvas::DagMessage::CommitSelected(sha) => {
                        self.center_loom.select_sha(&sha);
                        self.status_bar.head_sha = sha.clone();
                        self.bottom_prophecy.push("23:06:00", "☀️", format!("Focused commit kin signet: {}", sha), ProphecyLevel::Solar);
                        Task::none()
                    }
                },
                CenterLoomMessage::ZoomIn => {
                    self.center_loom.dag.zoom_in();
                    Task::none()
                }
                CenterLoomMessage::ZoomOut => {
                    self.center_loom.dag.zoom_out();
                    Task::none()
                }
                CenterLoomMessage::ResetZoom => {
                    self.center_loom.dag.reset_view();
                    Task::none()
                }
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
                        self.bottom_prophecy.push("23:06:00", "✨", format!("Offered file to altar: {}", path), ProphecyLevel::Jade);
                    }
                    self.status_bar.is_pristine = self.right_altar.unstaged_files.is_empty();
                    Task::none()
                }
                RightAltarMessage::UnstageFile(path) => {
                    if let Some(pos) = self.right_altar.staged_files.iter().position(|f| f.path == path) {
                        let item = self.right_altar.staged_files.remove(pos);
                        self.right_altar.unstaged_files.push(item);
                        self.bottom_prophecy.push("23:06:00", "↩️", format!("Withdrew file from altar: {}", path), ProphecyLevel::Maya);
                    }
                    self.status_bar.is_pristine = self.right_altar.unstaged_files.is_empty();
                    Task::none()
                }
                RightAltarMessage::StageAll => {
                    let mut all = std::mem::take(&mut self.right_altar.unstaged_files);
                    self.right_altar.staged_files.append(&mut all);
                    self.bottom_prophecy.push("23:06:00", "✨", "All offerings consecrated to the Altar".into(), ProphecyLevel::Jade);
                    self.status_bar.is_pristine = true;
                    Task::none()
                }
                RightAltarMessage::UnstageAll => {
                    let mut all = std::mem::take(&mut self.right_altar.staged_files);
                    self.right_altar.unstaged_files.append(&mut all);
                    self.bottom_prophecy.push("23:06:00", "↩️", "All offerings returned to working tree".into(), ProphecyLevel::Maya);
                    self.status_bar.is_pristine = self.right_altar.unstaged_files.is_empty();
                    Task::none()
                }
                RightAltarMessage::DiscardAll => {
                    self.bottom_prophecy.push("23:06:00", "⚠️", "Purification ceremony performed (changes discarded)".into(), ProphecyLevel::Xibalba);
                    Task::none()
                }
                RightAltarMessage::CommitSummaryChanged(s) => {
                    self.right_altar.commit_summary = s;
                    Task::none()
                }
                RightAltarMessage::InscribeCommit => {
                    let msg = self.right_altar.commit_summary.clone();
                    self.bottom_prophecy.push("23:06:00", "🔥", format!("Sealed in stone: '{}'", msg), ProphecyLevel::Solar);
                    self.right_altar.staged_files.clear();
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
            BackgroundMode::ProceduralLoom => self.loom.view(1920.0, 1080.0),
            BackgroundMode::StaticArtwork => {
                if let Some(handle) = &self.static_bg_handle {
                    image(handle.clone())
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(ContentFit::Cover)
                        .into()
                } else {
                    container(
                        text("🖼️ Place background.jpg in assets/ directory")
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
            BackgroundMode::VideoLoop => {
                if self.total_video_frames > 0 {
                    let frame_path = format!("itzamna-desktop/assets/video_frames/frame_{:03}.jpg", self.video_frame_index);
                    image(image::Handle::from_path(frame_path))
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .content_fit(ContentFit::Cover)
                        .into()
                } else {
                    self.loom.view(1920.0, 1080.0)
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
