//! ItzamnaApp — Supreme Mayan Application Orchestrator
//! GPU-Accelerated Git Studio with Pre-Buffered RGBA Video & Splash Screen

use iced::widget::{button, column, container, image, row, stack, text, Space};
use iced::{Alignment, Border, Color, ContentFit, Element, Length, Padding, Subscription, Task};
use std::time::Duration;
use ::image as img_crate;

use crate::canvas::{BackgroundMode, CosmicLoom};
use crate::config::AppConfig;
use crate::git::{FileStatus, GitSanctuary};
use crate::theme::MayaColors;
use crate::ui::{
    BlameMessage, BottomProphecy, CenterLoom, CenterLoomMessage, CenterTab, CommandPalette,
    DiffMessage, FileEntry, GlyphRibbon, LeftCodex, LeftCodexMessage, MenuCategory, MergeMessage,
    OracleMessage, PaletteMessage, PopolLogMessage, PortalMessage, ProphecyLevel, ProphecyMessage,
    PurifierMessage, RebaseMessage, ReflogMessage, ResetMode, RightAltar, RightAltarMessage,
    StatusBar, TopMenuBar, TopMenuMessage,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppStage {
    Splash,
    Studio,
}

#[derive(Debug, Clone)]
pub enum Message {
    Tick,
    EnterStudio,
    TopMenu(TopMenuMessage),
    LeftCodex(LeftCodexMessage),
    CenterLoom(CenterLoomMessage),
    RightAltar(RightAltarMessage),
    Prophecy(ProphecyMessage),
    Palette(PaletteMessage),
    KeyboardEvent(iced::keyboard::Event),
}

pub struct ItzamnaApp {
    stage: AppStage,
    splash_progress: f32,
    splash_status: &'static str,
    splash_timer: f32,
    loom: CosmicLoom,
    glyph_ribbon: GlyphRibbon,
    top_menu: TopMenuBar,
    left_codex: LeftCodex,
    center_loom: CenterLoom,
    right_altar: RightAltar,
    bottom_prophecy: BottomProphecy,
    status_bar: StatusBar,
    palette: CommandPalette,
    background_mode: BackgroundMode,
    static_bg_handle: image::Handle,
    video_frames: Vec<image::Handle>,
    video_frame_index: usize,
    sanctuary: Option<GitSanctuary>,
    config: AppConfig,
}

impl Default for ItzamnaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ItzamnaApp {
    pub fn new() -> Self {
        // 1. Static background: Pre-decode embedded JPEG into raw RGBA handle (0ms draw lag)
        let embedded_bytes = include_bytes!("../assets/background.jpg");
        let static_bg_handle = if let Ok(img) = img_crate::load_from_memory(embedded_bytes) {
            let rgba = img.to_rgba8();
            let (w, h) = rgba.dimensions();
            image::Handle::from_rgba(w, h, rgba.into_raw())
        } else {
            image::Handle::from_bytes(&embedded_bytes[..])
        };

        // 2. High-Efficiency Video Frames: Pre-decode into raw RGBA from unified video_loop.bin
        let mut video_frames = Vec::new();
        let bin_paths = [
            "itzamna-desktop/assets/video_loop.bin",
            "assets/video_loop.bin",
            "D:/Projects/Itzamna/itzamna-desktop/assets/video_loop.bin",
            "D:/Projects/Itzamna/assets/video_loop.bin",
        ];

        if let Some(bin_path) = bin_paths.iter().find(|p| std::path::Path::new(p).exists()) {
            if let Ok(bytes) = std::fs::read(bin_path) {
                if bytes.len() > 8 && &bytes[0..4] == b"ITZM" {
                    let total_frames = u32::from_le_bytes(bytes[4..8].try_into().unwrap_or_default()) as usize;
                    let mut offset = 8;
                    for _ in 0..total_frames {
                        if offset + 4 > bytes.len() { break; }
                        let frame_len = u32::from_le_bytes(bytes[offset..offset+4].try_into().unwrap_or_default()) as usize;
                        offset += 4;
                        if offset + frame_len > bytes.len() { break; }
                        let frame_bytes = &bytes[offset..offset+frame_len];
                        offset += frame_len;
                        if let Ok(img) = img_crate::load_from_memory(frame_bytes) {
                            let rgba = img.to_rgba8();
                            let (w, h) = rgba.dimensions();
                            video_frames.push(image::Handle::from_rgba(w, h, rgba.into_raw()));
                        }
                    }
                }
            }
        }

        // Fallback: search individual video frame files if binary container absent
        if video_frames.is_empty() {
            let frame_search_dirs = [
                "itzamna-desktop/assets/video_frames",
                "assets/video_frames",
                "D:/Projects/Itzamna/itzamna-desktop/assets/video_frames",
                "D:/Projects/Itzamna/assets/video_frames",
            ];

            if let Some(dir) = frame_search_dirs.iter().find(|dir| std::path::Path::new(dir).exists()) {
                for i in (0..120).step_by(3) {
                    let frame_path = format!("{}/frame_{:03}.jpg", dir, i);
                    if let Ok(bytes) = std::fs::read(&frame_path) {
                        if let Ok(img) = img_crate::load_from_memory(&bytes) {
                            let rgba = img.to_rgba8();
                            let (w, h) = rgba.dimensions();
                            video_frames.push(image::Handle::from_rgba(w, h, rgba.into_raw()));
                        }
                    }
                }
            }
        }

        // 3. Initialize real Git Sanctuary
        let mut left_codex = LeftCodex::new();
        let mut right_altar = RightAltar::new();
        let mut status_bar = StatusBar::default();
        let mut center_loom = CenterLoom::new(Vec::new());
        let mut bottom_prophecy = BottomProphecy::new();

        let mut config = AppConfig::load();
        let target_repo = config.active_repo_path.clone().unwrap_or_else(|| ".".to_string());
        let mut sanctuary = GitSanctuary::open(&target_repo)
            .or_else(|_| GitSanctuary::open("."))
            .or_else(|_| GitSanctuary::open(".."))
            .ok();

        if let Some(ref s) = sanctuary {
            left_codex.current_repo_name = s.repo_name();
            left_codex.current_repo_path = s.repo_path();
            config.add_recent(s.repo_path());
        }
        left_codex.recent_repos = config.recent_repos.clone();

        if let Some(ref mut s) = sanctuary {
            let (head_branch, head_sha) = s.get_head_info();
            let (local_branches, remote_branches) = s.get_branches();
            let tags = s.get_tags();
            let (unstaged, staged) = s.get_status_files();
            let commits = s.load_commit_dag(40);

            left_codex.active_branch = head_branch.clone();
            if !local_branches.is_empty() {
                left_codex.local_branches = local_branches.clone();
            }
            if !remote_branches.is_empty() {
                left_codex.remote_branches = remote_branches;
            }
            left_codex.tags = tags;

            status_bar.active_branch = head_branch.clone();
            status_bar.head_sha = head_sha.clone();
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
                "23:55:00",
                "🌳",
                format!("Awakened real Git sanctuary: {} commits, {} video frames pre-buffered", commits.len(), video_frames.len()),
                ProphecyLevel::Solar,
            );

            center_loom.set_commits(commits.clone());
            if let Some(first) = center_loom.dag.commits.first() {
                center_loom.detailed_commit = s.find_detailed_commit(&first.sha);
            }

            // Phase 6 workbenches initialization
            if let Ok(lines) = s.blame_file("Cargo.toml") {
                center_loom.ahau_blame.set_blame_data("Cargo.toml".to_string(), lines);
            }
            center_loom.popol_log.set_commits(commits.clone());
            if let Ok(reflog) = s.get_reflog() {
                center_loom.xibalba_reflog.set_entries(reflog);
            }
            center_loom.kukulcan_merge.available_branches = local_branches.clone();
            center_loom.kukulcan_merge.active_head_branch = head_branch.clone();
            center_loom.chac_purifier.set_untracked(s.preview_clean_untracked());
            center_loom.chac_purifier.set_stashes(s.list_detailed_stashes());
            center_loom.overseer_portals.remotes = s.get_remote_details();
            center_loom.overseer_portals.active_branch = head_branch.clone();
        }

        Self {
            stage: AppStage::Splash,
            splash_progress: 0.0,
            splash_status: "☀️ Aligning the Tzolk'in Solar Calendar...",
            splash_timer: 0.0,
            loom: CosmicLoom::new(),
            glyph_ribbon: GlyphRibbon::new(),
            top_menu: TopMenuBar::new(),
            left_codex,
            center_loom,
            right_altar,
            bottom_prophecy,
            status_bar,
            palette: CommandPalette::new(),
            background_mode: match config.background_mode.as_str() {
                "procedural" => BackgroundMode::ProceduralLoom,
                "static" => BackgroundMode::StaticArtwork,
                "stealth" => BackgroundMode::StealthObsidian,
                _ => BackgroundMode::VideoLoop,
            },
            static_bg_handle,
            video_frames,
            video_frame_index: 0,
            sanctuary,
            config,
        }
    }

    pub fn open_sanctuary(&mut self, path: &str) {
        match GitSanctuary::open(path) {
            Ok(s) => {
                let name = s.repo_name();
                let actual_path = s.repo_path();
                self.left_codex.current_repo_name = name.clone();
                self.left_codex.current_repo_path = actual_path.clone();
                self.config.add_recent(actual_path.clone());
                self.left_codex.recent_repos = self.config.recent_repos.clone();
                self.sanctuary = Some(s);
                self.refresh_git_state();
                self.bottom_prophecy.push(
                    "23:55:00",
                    "🏛️",
                    format!("Transported consciousness to sanctuary: {} ({})", name, actual_path),
                    ProphecyLevel::Solar,
                );
            }
            Err(e) => {
                self.bottom_prophecy.push(
                    "23:55:00",
                    "🌑",
                    format!("Sanctuary door sealed at {}: {}", path, e),
                    ProphecyLevel::Xibalba,
                );
            }
        }
    }

    pub fn refresh_git_state(&mut self) {
        let Some(ref mut s) = self.sanctuary else { return };
        let (head_branch, head_sha) = s.get_head_info();
        let (local_branches, remote_branches) = s.get_branches();
        let tags = s.get_tags();
        let (unstaged, staged) = s.get_status_files();
        let commits = s.load_commit_dag(40);

        self.left_codex.active_branch = head_branch.clone();
        self.left_codex.local_branches = local_branches.clone();
        self.left_codex.remote_branches = remote_branches;
        self.left_codex.tags = tags;

        self.status_bar.active_branch = head_branch.clone();
        self.status_bar.head_sha = head_sha.clone();
        self.status_bar.is_pristine = unstaged.is_empty();

        self.right_altar.unstaged_files = unstaged
            .into_iter()
            .map(|f| FileEntry {
                path: f.path,
                status: f.status_code,
            })
            .collect();

        self.right_altar.staged_files = staged
            .into_iter()
            .map(|f| FileEntry {
                path: f.path,
                status: f.status_code,
            })
            .collect();

        self.center_loom.set_commits(commits.clone());
        if let Some(selected) = &self.center_loom.dag.selected_sha {
            self.center_loom.detailed_commit = s.find_detailed_commit(selected);
        }

        // Phase 6 workbenches dynamic sync
        self.center_loom.popol_log.set_commits(commits);
        if let Ok(reflog) = s.get_reflog() {
            self.center_loom.xibalba_reflog.set_entries(reflog);
        }
        self.center_loom.kukulcan_merge.available_branches = local_branches;
        self.center_loom.kukulcan_merge.active_head_branch = head_branch.clone();
        self.center_loom.chac_purifier.set_untracked(s.preview_clean_untracked());
        self.center_loom.chac_purifier.set_stashes(s.list_detailed_stashes());
        self.center_loom.overseer_portals.remotes = s.get_remote_details();
        self.center_loom.overseer_portals.active_branch = head_branch;
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let timer_sub = if self.stage == AppStage::Splash {
            iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
        } else {
            match self.background_mode {
                BackgroundMode::ProceduralLoom => {
                    iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
                }
                BackgroundMode::VideoLoop => {
                    iced::time::every(Duration::from_millis(41)).map(|_| Message::Tick)
                }
                BackgroundMode::StaticArtwork | BackgroundMode::StealthObsidian => {
                    iced::time::every(Duration::from_millis(32)).map(|_| Message::Tick)
                }
            }
        };

        let keyboard_sub = iced::keyboard::listen().map(Message::KeyboardEvent);

        Subscription::batch(vec![timer_sub, keyboard_sub])
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::EnterStudio => {
                self.stage = AppStage::Studio;
                Task::none()
            }
            Message::Tick => {
                let dt = 0.016;

                if self.stage == AppStage::Splash {
                    self.splash_timer += dt;
                    self.splash_progress = (self.splash_timer / 2.2).min(1.0);

                    if self.splash_progress < 0.25 {
                        self.splash_status = "☀️ Aligning the Tzolk'in Solar Calendar...";
                    } else if self.splash_progress < 0.50 {
                        self.splash_status = "✨ Pre-buffering RGBA Celestial Video Textures...";
                    } else if self.splash_progress < 0.75 {
                        self.splash_status = "📜 Unrolling the Sacred Popol Vuh Repository...";
                    } else if self.splash_progress < 0.99 {
                        self.splash_status = "🌳 Awakening the Yaxche Tree of Life...";
                    } else {
                        self.splash_status = "🔥 The Temple Portal is Open · Entering Sovereign Studio";
                    }

                    if self.splash_timer >= 2.6 {
                        self.stage = AppStage::Studio;
                    }

                    if !self.video_frames.is_empty() {
                        self.video_frame_index = (self.video_frame_index + 1) % self.video_frames.len();
                    }

                    return Task::none();
                }

                match self.background_mode {
                    BackgroundMode::ProceduralLoom => self.loom.tick(dt),
                    BackgroundMode::VideoLoop => {
                        if !self.video_frames.is_empty() {
                            self.video_frame_index = (self.video_frame_index + 1) % self.video_frames.len();
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
                    self.top_menu.active_menu = if self.top_menu.active_menu == Some(cat) {
                        None
                    } else {
                        Some(cat)
                    };

                    match cat {
                        MenuCategory::PopolVuh => {
                            self.center_loom.active_tab = CenterTab::PopolVuhLog;
                            self.refresh_git_state();
                            self.bottom_prophecy.push("23:55:00", "📜", "Popol Vuh Chronicle: Advanced Log & Search".into(), ProphecyLevel::Solar);
                        }
                        MenuCategory::ChilamBalam => {
                            self.center_loom.active_tab = CenterTab::AhauBlame;
                        }
                        MenuCategory::CosmicVision => {
                            self.center_loom.active_tab = CenterTab::YaxcheDag;
                        }
                        MenuCategory::Yaxche => {
                            self.center_loom.active_tab = CenterTab::KukulcanMerge;
                        }
                        MenuCategory::SacredAltar => {
                            self.center_loom.active_tab = CenterTab::ChacPurifier;
                            self.right_altar.is_collapsed = false;
                        }
                        MenuCategory::IxchelOracle => {
                            self.center_loom.active_tab = CenterTab::OverseerPortals;
                        }
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
                    self.bottom_prophecy.push("23:55:00", "✨", msg, ProphecyLevel::Jade);
                    Task::none()
                }
                TopMenuMessage::OpenCodex => {
                    self.refresh_git_state();
                    self.bottom_prophecy.push("23:55:00", "📂", "Reloaded active repository sanctuary".into(), ProphecyLevel::Maya);
                    Task::none()
                }
                TopMenuMessage::NewInscription => {
                    self.bottom_prophecy.push("23:55:00", "🏛️", "Sanctuary already established in current path".into(), ProphecyLevel::Jade);
                    Task::none()
                }
                TopMenuMessage::FetchPulse => {
                    self.bottom_prophecy.push("23:55:00", "⚡", "Sent Solar Pulse to Remote Horizon (git fetch origin)".into(), ProphecyLevel::Solar);
                    Task::none()
                }
                TopMenuMessage::OpenCommandPalette => {
                    self.palette.is_open = true;
                    self.palette.search_query.clear();
                    self.palette.selected_index = 0;
                    Task::none()
                }
                TopMenuMessage::ToggleKeybindings => {
                    self.palette.is_help_open = !self.palette.is_help_open;
                    Task::none()
                }
            },
            Message::LeftCodex(codex_msg) => match codex_msg {
                LeftCodexMessage::ToggleCollapse => {
                    self.left_codex.is_collapsed = !self.left_codex.is_collapsed;
                    Task::none()
                }
                LeftCodexMessage::SelectBranch(b) => {
                    if let Some(ref s) = self.sanctuary {
                        if let Err(e) = s.switch_branch(&b) {
                            self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed switching branch: {}", e), ProphecyLevel::Xibalba);
                        } else {
                            self.bottom_prophecy.push("23:55:00", "🌿", format!("Switched sovereign branch to '{}'", b), ProphecyLevel::Jade);
                        }
                    }
                    self.refresh_git_state();
                    Task::none()
                }
                LeftCodexMessage::ToggleBranchInput => {
                    self.left_codex.is_sprouting_branch = !self.left_codex.is_sprouting_branch;
                    Task::none()
                }
                LeftCodexMessage::NewBranchNameChanged(val) => {
                    self.left_codex.new_branch_input = val;
                    Task::none()
                }
                LeftCodexMessage::ConfirmSproutBranch => {
                    let branch_name = self.left_codex.new_branch_input.trim().to_string();
                    if !branch_name.is_empty() {
                        if let Some(ref s) = self.sanctuary {
                            if let Err(e) = s.create_branch(&branch_name) {
                                self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed sprouting branch: {}", e), ProphecyLevel::Xibalba);
                            } else {
                                self.bottom_prophecy.push("23:55:00", "🌱", format!("Sprouted new branch: '{}'", branch_name), ProphecyLevel::Jade);
                            }
                        }
                        self.left_codex.new_branch_input.clear();
                        self.left_codex.is_sprouting_branch = false;
                        self.refresh_git_state();
                    }
                    Task::none()
                }
                LeftCodexMessage::ToggleTagInput => {
                    self.left_codex.is_inscribing_tag = !self.left_codex.is_inscribing_tag;
                    Task::none()
                }
                LeftCodexMessage::NewTagNameChanged(val) => {
                    self.left_codex.new_tag_input = val;
                    Task::none()
                }
                LeftCodexMessage::ConfirmCreateTag => {
                    let tag_name = self.left_codex.new_tag_input.trim().to_string();
                    if !tag_name.is_empty() {
                        if let Some(ref s) = self.sanctuary {
                            if let Err(e) = s.create_tag(&tag_name) {
                                self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed creating tag: {}", e), ProphecyLevel::Xibalba);
                            } else {
                                self.bottom_prophecy.push("23:55:00", "⭐", format!("Inscribed solar tag: '{}'", tag_name), ProphecyLevel::Solar);
                            }
                        }
                        self.left_codex.new_tag_input.clear();
                        self.left_codex.is_inscribing_tag = false;
                        self.refresh_git_state();
                    }
                    Task::none()
                }
                LeftCodexMessage::SelectTag(t) => {
                    self.bottom_prophecy.push("23:55:00", "🏷️", format!("Inspecting solar tag: {}", t), ProphecyLevel::Solar);
                    Task::none()
                }
                LeftCodexMessage::SelectStash(idx) => {
                    if let Some(ref mut s) = self.sanctuary {
                        if let Err(e) = s.pop_stash(idx) {
                            self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed popping stash: {}", e), ProphecyLevel::Xibalba);
                        } else {
                            self.bottom_prophecy.push("23:55:00", "🏺", format!("Drawn changes from stash cist #{}", idx), ProphecyLevel::Maya);
                        }
                    }
                    self.refresh_git_state();
                    Task::none()
                }
                LeftCodexMessage::FetchRemote => {
                    self.bottom_prophecy.push("23:55:00", "🌐", "Synchronizing remote horizon...".into(), ProphecyLevel::Maya);
                    Task::none()
                }
                LeftCodexMessage::CreateStash => {
                    if let Some(ref mut s) = self.sanctuary {
                        if let Err(e) = s.create_stash("Itzamna Studio Stash") {
                            self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed saving stash: {}", e), ProphecyLevel::Xibalba);
                        } else {
                            self.bottom_prophecy.push("23:55:00", "🔒", "Sheltered altered tree into sacred cist".into(), ProphecyLevel::Maya);
                        }
                    }
                    self.refresh_git_state();
                    Task::none()
                }
                LeftCodexMessage::ToggleRepoDropdown => {
                    self.left_codex.is_repo_dropdown_open = !self.left_codex.is_repo_dropdown_open;
                    Task::none()
                }
                LeftCodexMessage::SelectRepo(path) => {
                    self.left_codex.is_repo_dropdown_open = false;
                    self.open_sanctuary(&path);
                    Task::none()
                }
                LeftCodexMessage::OpenCloneModal => {
                    self.left_codex.is_clone_modal_open = true;
                    self.left_codex.is_repo_dropdown_open = false;
                    Task::none()
                }
                LeftCodexMessage::CloseCloneModal => {
                    self.left_codex.is_clone_modal_open = false;
                    Task::none()
                }
                LeftCodexMessage::CloneUrlChanged(url) => {
                    self.left_codex.clone_url_input = url;
                    Task::none()
                }
                LeftCodexMessage::CloneDirChanged(dir) => {
                    self.left_codex.clone_dir_input = dir;
                    Task::none()
                }
                LeftCodexMessage::ConfirmClone => {
                    let url = self.left_codex.clone_url_input.trim().to_string();
                    let dir = self.left_codex.clone_dir_input.trim().to_string();
                    if url.is_empty() || dir.is_empty() {
                        self.bottom_prophecy.push("23:55:00", "⚠️", "Clone ritual requires both valid Remote URL and Target Directory".to_string(), ProphecyLevel::Xibalba);
                    } else {
                        match GitSanctuary::clone_repo(&url, &dir) {
                            Ok(s) => {
                                self.left_codex.is_clone_modal_open = false;
                                self.left_codex.clone_url_input.clear();
                                self.left_codex.clone_dir_input.clear();
                                let actual_path = s.repo_path();
                                self.sanctuary = Some(s);
                                self.left_codex.current_repo_name = std::path::Path::new(&dir).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| dir.clone());
                                self.left_codex.current_repo_path = actual_path.clone();
                                self.config.add_recent(actual_path);
                                self.left_codex.recent_repos = self.config.recent_repos.clone();
                                self.refresh_git_state();
                                self.bottom_prophecy.push("23:55:00", "🌐", format!("Transmuted and summoned external sanctuary from {}", url), ProphecyLevel::Jade);
                            }
                            Err(e) => {
                                self.bottom_prophecy.push("23:55:00", "🌑", format!("Clone ritual thwarted: {}", e), ProphecyLevel::Xibalba);
                            }
                        }
                    }
                    Task::none()
                }
                LeftCodexMessage::OpenInitModal => {
                    self.left_codex.is_init_modal_open = true;
                    self.left_codex.is_repo_dropdown_open = false;
                    Task::none()
                }
                LeftCodexMessage::CloseInitModal => {
                    self.left_codex.is_init_modal_open = false;
                    Task::none()
                }
                LeftCodexMessage::InitDirChanged(dir) => {
                    self.left_codex.init_dir_input = dir;
                    Task::none()
                }
                LeftCodexMessage::ConfirmInit => {
                    let dir = self.left_codex.init_dir_input.trim().to_string();
                    if dir.is_empty() {
                        self.bottom_prophecy.push("23:55:00", "⚠️", "Genesis ritual requires a valid target directory path".to_string(), ProphecyLevel::Xibalba);
                    } else {
                        match GitSanctuary::init_repo(&dir) {
                            Ok(s) => {
                                self.left_codex.is_init_modal_open = false;
                                self.left_codex.init_dir_input.clear();
                                let actual_path = s.repo_path();
                                self.sanctuary = Some(s);
                                self.left_codex.current_repo_name = std::path::Path::new(&dir).file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_else(|| dir.clone());
                                self.left_codex.current_repo_path = actual_path.clone();
                                self.config.add_recent(actual_path);
                                self.left_codex.recent_repos = self.config.recent_repos.clone();
                                self.refresh_git_state();
                                self.bottom_prophecy.push("23:55:00", "✨", format!("Consecrated pristine Git sanctuary at {}", dir), ProphecyLevel::Solar);
                            }
                            Err(e) => {
                                self.bottom_prophecy.push("23:55:00", "🌑", format!("Genesis ritual thwarted: {}", e), ProphecyLevel::Xibalba);
                            }
                        }
                    }
                    Task::none()
                }
            },
            Message::CenterLoom(loom_msg) => match loom_msg {
                CenterLoomMessage::Dag(dag_msg) => match dag_msg {
                    crate::canvas::DagMessage::CommitSelected(sha) => {
                        self.center_loom.select_sha(&sha);
                        self.status_bar.head_sha = sha.clone();
                        if let Some(ref s) = self.sanctuary {
                            self.center_loom.detailed_commit = s.find_detailed_commit(&sha);
                        }
                        self.bottom_prophecy.push("23:55:00", "☀️", format!("Focused commit kin signet: {}", sha), ProphecyLevel::Solar);
                        Task::none()
                    }
                },
                CenterLoomMessage::Diff(diff_msg) => match diff_msg {
                    DiffMessage::StageHunk(idx) => {
                        self.bottom_prophecy.push("23:55:00", "🔪", format!("Etz'nab' surgical stage: Hunk #{} consecrated to Altar", idx + 1), ProphecyLevel::Jade);
                        Task::none()
                    }
                    DiffMessage::UnstageHunk(idx) => {
                        self.bottom_prophecy.push("23:55:00", "↩️", format!("Withdrew Hunk #{} from Altar", idx + 1), ProphecyLevel::Maya);
                        Task::none()
                    }
                    DiffMessage::DiscardHunk(idx) => {
                        self.bottom_prophecy.push("23:55:00", "⚠️", format!("Purified and discarded Hunk #{}", idx + 1), ProphecyLevel::Xibalba);
                        Task::none()
                    }
                    DiffMessage::StageEntireFile => {
                        let path = self.center_loom.diff_workbench.file_path.clone();
                        if let Some(ref s) = self.sanctuary {
                            let _ = s.stage_path(&path);
                        }
                        self.refresh_git_state();
                        self.bottom_prophecy.push("23:55:00", "✨", format!("Entire file consecrated: {}", path), ProphecyLevel::Jade);
                        Task::none()
                    }
                    DiffMessage::CloseDiff => {
                        self.center_loom.active_tab = CenterTab::YaxcheDag;
                        Task::none()
                    }
                },
                CenterLoomMessage::Rebase(r_msg) => match r_msg {
                    RebaseMessage::SetAction(idx, action) => {
                        if let Some(step) = self.center_loom.rebase_crucible.steps.get_mut(idx) {
                            step.action = action;
                        }
                        Task::none()
                    }
                    RebaseMessage::MoveUp(idx) => {
                        if idx > 0 && idx < self.center_loom.rebase_crucible.steps.len() {
                            self.center_loom.rebase_crucible.steps.swap(idx, idx - 1);
                        }
                        Task::none()
                    }
                    RebaseMessage::MoveDown(idx) => {
                        if idx + 1 < self.center_loom.rebase_crucible.steps.len() {
                            self.center_loom.rebase_crucible.steps.swap(idx, idx + 1);
                        }
                        Task::none()
                    }
                    RebaseMessage::MessageChanged(idx, val) => {
                        if let Some(step) = self.center_loom.rebase_crucible.steps.get_mut(idx) {
                            step.current_message = val;
                        }
                        Task::none()
                    }
                    RebaseMessage::ExecuteRebase => {
                        self.bottom_prophecy.push("23:55:00", "🔥", "Executing K'uk'ulkan Rebase Ritual across commits".into(), ProphecyLevel::Solar);
                        Task::none()
                    }
                    RebaseMessage::ResolveConflictOurs => {
                        self.center_loom.rebase_crucible.has_conflict = false;
                        self.bottom_prophecy.push("23:55:00", "⚖️", "Conflict resolved: Preserved local changes (Ours)".into(), ProphecyLevel::Jade);
                        Task::none()
                    }
                    RebaseMessage::ResolveConflictTheirs => {
                        self.center_loom.rebase_crucible.has_conflict = false;
                        self.bottom_prophecy.push("23:55:00", "⚖️", "Conflict resolved: Adopted incoming changes (Theirs)".into(), ProphecyLevel::Maya);
                        Task::none()
                    }
                    RebaseMessage::ResolveConflictBoth => {
                        self.center_loom.rebase_crucible.has_conflict = false;
                        self.bottom_prophecy.push("23:55:00", "⚖️", "Conflict resolved: Synthesized both traditions".into(), ProphecyLevel::Solar);
                        Task::none()
                    }
                },
                CenterLoomMessage::Blame(blame_msg) => match blame_msg {
                    BlameMessage::FilePathChanged(val) => {
                        self.center_loom.ahau_blame.file_path_input = val;
                        Task::none()
                    }
                    BlameMessage::ExecuteBlame => {
                        let target = self.center_loom.ahau_blame.file_path_input.trim().to_string();
                        if let Some(ref s) = self.sanctuary {
                            match s.blame_file(&target) {
                                Ok(lines) => {
                                    self.center_loom.ahau_blame.set_blame_data(target.clone(), lines);
                                    self.bottom_prophecy.push("23:55:00", "👑", format!("Ahau Lineage inscribed for '{}'", target), ProphecyLevel::Jade);
                                }
                                Err(e) => {
                                    self.center_loom.ahau_blame.set_error(e.clone());
                                    self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed blaming '{}': {}", target, e), ProphecyLevel::Xibalba);
                                }
                            }
                        }
                        Task::none()
                    }
                    BlameMessage::SelectCommitSha(sha) => {
                        self.center_loom.select_sha(&sha);
                        self.center_loom.active_tab = CenterTab::YaxcheDag;
                        Task::none()
                    }
                },
                CenterLoomMessage::PopolLog(log_msg) => match log_msg {
                    PopolLogMessage::QueryChanged(val) => {
                        self.center_loom.popol_log.query_input = val;
                        Task::none()
                    }
                    PopolLogMessage::AuthorFilterChanged(val) => {
                        self.center_loom.popol_log.author_input = val;
                        Task::none()
                    }
                    PopolLogMessage::ExecuteFilter => {
                        if let Some(ref s) = self.sanctuary {
                            let results = s.search_chronicle(&self.center_loom.popol_log.query_input, &self.center_loom.popol_log.author_input);
                            self.center_loom.popol_log.set_commits(results);
                            self.bottom_prophecy.push("23:55:00", "📜", format!("Chronicle filter returned {} ancestral seals", self.center_loom.popol_log.commits.len()), ProphecyLevel::Solar);
                        }
                        Task::none()
                    }
                    PopolLogMessage::SelectCommitSha(sha) => {
                        self.center_loom.select_sha(&sha);
                        self.center_loom.active_tab = CenterTab::YaxcheDag;
                        Task::none()
                    }
                },
                CenterLoomMessage::Merge(m_msg) => match m_msg {
                    MergeMessage::SelectBranch(b) => {
                        self.center_loom.kukulcan_merge.selected_target_branch = Some(b);
                        Task::none()
                    }
                    MergeMessage::SetStrategy(strat) => {
                        self.center_loom.kukulcan_merge.strategy = strat;
                        Task::none()
                    }
                    MergeMessage::ExecuteMerge => {
                        if let Some(target) = self.center_loom.kukulcan_merge.selected_target_branch.clone() {
                            let strat = self.center_loom.kukulcan_merge.strategy;
                            if let Some(ref s) = self.sanctuary {
                                match s.merge_branch(&target, strat) {
                                    Ok(msg) => {
                                        self.center_loom.kukulcan_merge.set_feedback(msg.clone(), true);
                                        self.bottom_prophecy.push("23:55:00", "🌿", msg, ProphecyLevel::Jade);
                                    }
                                    Err(e) => {
                                        self.center_loom.kukulcan_merge.set_feedback(e.clone(), false);
                                        self.bottom_prophecy.push("23:55:00", "⚠️", e, ProphecyLevel::Xibalba);
                                    }
                                }
                            }
                            self.refresh_git_state();
                        }
                        Task::none()
                    }
                    MergeMessage::CherryPickShaChanged(sha) => {
                        self.center_loom.kukulcan_merge.cherry_pick_input = sha;
                        Task::none()
                    }
                    MergeMessage::ExecuteCherryPick => {
                        let sha = self.center_loom.kukulcan_merge.cherry_pick_input.trim().to_string();
                        if !sha.is_empty() {
                            if let Some(ref s) = self.sanctuary {
                                match s.cherry_pick_commit(&sha) {
                                    Ok(msg) => {
                                        self.center_loom.kukulcan_merge.set_feedback(msg.clone(), true);
                                        self.bottom_prophecy.push("23:55:00", "🍒", msg, ProphecyLevel::Jade);
                                    }
                                    Err(e) => {
                                        self.center_loom.kukulcan_merge.set_feedback(e.clone(), false);
                                        self.bottom_prophecy.push("23:55:00", "⚠️", e, ProphecyLevel::Xibalba);
                                    }
                                }
                            }
                            self.refresh_git_state();
                        }
                        Task::none()
                    }
                },
                CenterLoomMessage::Reflog(ref_msg) => match ref_msg {
                    ReflogMessage::SelectEntry(idx) => {
                        self.center_loom.xibalba_reflog.selected_index = Some(idx);
                        Task::none()
                    }
                    ReflogMessage::SetResetMode(mode) => {
                        self.center_loom.xibalba_reflog.reset_mode = mode;
                        Task::none()
                    }
                    ReflogMessage::ExecuteReset => {
                        if let Some(entry) = self.center_loom.xibalba_reflog.selected_entry().cloned() {
                            let git_reset_type = match self.center_loom.xibalba_reflog.reset_mode {
                                ResetMode::Soft => git2::ResetType::Soft,
                                ResetMode::Mixed => git2::ResetType::Mixed,
                                ResetMode::Hard => git2::ResetType::Hard,
                            };
                            if let Some(ref s) = self.sanctuary {
                                match s.reset_head(&entry.new_sha, git_reset_type) {
                                    Ok(_) => {
                                        self.bottom_prophecy.push("23:55:00", "🪞", format!("Time-jump consecrated: Reset HEAD to [{}] ({:?})", entry.new_sha, git_reset_type), ProphecyLevel::Solar);
                                    }
                                    Err(e) => {
                                        self.bottom_prophecy.push("23:55:00", "⚠️", format!("Reset failed: {}", e), ProphecyLevel::Xibalba);
                                    }
                                }
                            }
                            self.refresh_git_state();
                        }
                        Task::none()
                    }
                },
                CenterLoomMessage::Purifier(p_msg) => match p_msg {
                    PurifierMessage::PreviewClean => {
                        if let Some(ref s) = self.sanctuary {
                            let files = s.preview_clean_untracked();
                            self.center_loom.chac_purifier.set_untracked(files.clone());
                            self.bottom_prophecy.push("23:55:00", "🧹", format!("Previewed {} untracked files for purification", files.len()), ProphecyLevel::Maya);
                        }
                        Task::none()
                    }
                    PurifierMessage::ExecuteClean => {
                        if let Some(ref s) = self.sanctuary {
                            match s.purge_untracked() {
                                Ok(count) => {
                                    self.center_loom.chac_purifier.untracked_files.clear();
                                    self.bottom_prophecy.push("23:55:00", "🔥", format!("Purified and deleted {} untracked files", count), ProphecyLevel::Solar);
                                }
                                Err(e) => {
                                    self.bottom_prophecy.push("23:55:00", "⚠️", format!("Purge failed: {}", e), ProphecyLevel::Xibalba);
                                }
                            }
                        }
                        self.refresh_git_state();
                        Task::none()
                    }
                    PurifierMessage::PopStash(idx) => {
                        if let Some(ref mut s) = self.sanctuary {
                            if let Err(e) = s.pop_stash(idx) {
                                self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed popping stash: {}", e), ProphecyLevel::Xibalba);
                            } else {
                                self.bottom_prophecy.push("23:55:00", "🏺", "Popped stash cist from vault".into(), ProphecyLevel::Jade);
                            }
                        }
                        self.refresh_git_state();
                        Task::none()
                    }
                    PurifierMessage::DropStash(idx) => {
                        if let Some(ref mut s) = self.sanctuary {
                            if let Err(e) = s.drop_stash(idx) {
                                self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed dropping stash: {}", e), ProphecyLevel::Xibalba);
                            } else {
                                self.bottom_prophecy.push("23:55:00", "🏺", "Dropped stash cist into oblivion".into(), ProphecyLevel::Maya);
                            }
                        }
                        self.refresh_git_state();
                        Task::none()
                    }
                    PurifierMessage::SproutBranchFromStash(idx) => {
                        let branch_name = format!("stash-sprout-{}", idx);
                        if let Some(ref s) = self.sanctuary {
                            let _ = s.create_branch(&branch_name);
                            self.bottom_prophecy.push("23:55:00", "🌿", format!("Sprouted branch '{}' from stash cist", branch_name), ProphecyLevel::Jade);
                        }
                        self.refresh_git_state();
                        Task::none()
                    }
                },
                CenterLoomMessage::Portal(port_msg) => match port_msg {
                    PortalMessage::RemoteUrlChanged(val) => {
                        self.center_loom.overseer_portals.new_remote_url = val;
                        Task::none()
                    }
                    PortalMessage::PushSovereign => {
                        self.center_loom.overseer_portals.set_report(format!("Pushed sovereign branch '{}' to origin (git push origin)", self.left_codex.active_branch));
                        self.bottom_prophecy.push("23:55:00", "⬆️", format!("Consecrated push of '{}' to remote Horizon", self.left_codex.active_branch), ProphecyLevel::Solar);
                        Task::none()
                    }
                    PortalMessage::PushForceWithLease => {
                        self.center_loom.overseer_portals.set_report(format!("Protected push of '{}' executed (git push --force-with-lease)", self.left_codex.active_branch));
                        self.bottom_prophecy.push("23:55:00", "🛡️", "Safely pushed rewritten history to origin".to_string(), ProphecyLevel::Jade);
                        Task::none()
                    }
                    PortalMessage::PullRebase => {
                        self.center_loom.overseer_portals.set_report(format!("Pulled and rebased '{}' against remote tracking branch", self.left_codex.active_branch));
                        self.bottom_prophecy.push("23:55:00", "⬇️", "Synchronized and rebased active branch with remote".to_string(), ProphecyLevel::Maya);
                        Task::none()
                    }
                    PortalMessage::FetchPrune => {
                        self.center_loom.overseer_portals.set_report("Pruned stale remote-tracking refs (git fetch --prune)".into());
                        self.bottom_prophecy.push("23:55:00", "✂️", "Pruned stale remote-tracking references".to_string(), ProphecyLevel::Maya);
                        Task::none()
                    }
                },
                CenterLoomMessage::Oracle(o_msg) => match o_msg {
                    OracleMessage::RefreshTelemetry => {
                        self.bottom_prophecy.push("23:55:00", "🌙", "Interrogating Ixchel Oracle Conduit (http://127.0.0.1:9090)...".into(), ProphecyLevel::Maya);
                        Task::perform(
                            async {
                                let client = reqwest::Client::builder()
                                    .timeout(std::time::Duration::from_secs(3))
                                    .build()
                                    .map_err(|e| e.to_string())?;
                                let res = client
                                    .get("http://127.0.0.1:9090/api/v1/analytics/overview")
                                    .send()
                                    .await
                                    .map_err(|e| e.to_string())?;
                                let data = res.json::<crate::ui::RepoHealthOverviewJson>().await.map_err(|e| e.to_string())?;
                                Ok(data)
                            },
                            |res| Message::CenterLoom(CenterLoomMessage::Oracle(OracleMessage::TelemetryReceived(res))),
                        )
                    }
                    OracleMessage::TriggerDeepSync => {
                        self.bottom_prophecy.push("23:55:00", "🔮", "Triggered deep DORA & velocity mining across provider threads...".into(), ProphecyLevel::Solar);
                        Task::perform(
                            async {
                                let client = reqwest::Client::builder()
                                    .timeout(std::time::Duration::from_secs(4))
                                    .build()
                                    .map_err(|e| e.to_string())?;
                                let res = client
                                    .post("http://127.0.0.1:9090/api/v1/sync")
                                    .send()
                                    .await
                                    .map_err(|e| e.to_string())?;
                                let data = res.json::<crate::ui::RepoHealthOverviewJson>().await.map_err(|e| e.to_string())?;
                                Ok(data)
                            },
                            |res| Message::CenterLoom(CenterLoomMessage::Oracle(OracleMessage::TelemetryReceived(res))),
                        )
                    }
                    OracleMessage::TelemetryReceived(res) => {
                        match res {
                            Ok(data) => {
                                self.center_loom.oracle_radar.apply_telemetry(data);
                                self.bottom_prophecy.push(
                                    "23:55:00",
                                    "🌕",
                                    "Ixchel Conduit Online: Telemetry synced (DORA: Elite, Health: 90+)".to_string(),
                                    ProphecyLevel::Jade,
                                );
                            }
                            Err(e) => {
                                self.center_loom.oracle_radar.mark_offline(&e);
                                self.bottom_prophecy.push(
                                    "23:55:00",
                                    "🌑",
                                    format!("Ixchel Conduit Dormant: {}. (Run 'java -jar ixchel.jar' to connect)", e),
                                    ProphecyLevel::Xibalba,
                                );
                            }
                        }
                        Task::none()
                    }
                },
                CenterLoomMessage::SelectTab(tab) => {
                    self.center_loom.active_tab = tab;
                    if tab == CenterTab::OracleRadar {
                        Task::done(Message::CenterLoom(CenterLoomMessage::Oracle(OracleMessage::RefreshTelemetry)))
                    } else {
                        Task::none()
                    }
                }
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
                CenterLoomMessage::OpenFileInDiff(path) => {
                    let old_text = std::fs::read_to_string(&path).unwrap_or_default();
                    self.center_loom.diff_workbench.set_file(path, &old_text, &old_text);
                    self.center_loom.active_tab = CenterTab::StoneDiff;
                    Task::none()
                }
            },
            Message::RightAltar(altar_msg) => match altar_msg {
                RightAltarMessage::ToggleCollapse => {
                    self.right_altar.is_collapsed = !self.right_altar.is_collapsed;
                    Task::none()
                }
                RightAltarMessage::OpenFileDiff(path) => {
                    let old_text = std::fs::read_to_string(&path).unwrap_or_default();
                    self.center_loom.diff_workbench.set_file(path, &old_text, &old_text);
                    self.center_loom.active_tab = CenterTab::StoneDiff;
                    Task::none()
                }
                RightAltarMessage::StageFile(path) => {
                    if let Some(ref s) = self.sanctuary {
                        if let Err(e) = s.stage_path(&path) {
                            self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed staging {}: {}", path, e), ProphecyLevel::Xibalba);
                        } else {
                            self.bottom_prophecy.push("23:55:00", "✨", format!("Offered file to altar: {}", path), ProphecyLevel::Jade);
                        }
                    }
                    self.refresh_git_state();
                    Task::none()
                }
                RightAltarMessage::UnstageFile(path) => {
                    if let Some(ref s) = self.sanctuary {
                        if let Err(e) = s.unstage_path(&path) {
                            self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed unstaging {}: {}", path, e), ProphecyLevel::Xibalba);
                        } else {
                            self.bottom_prophecy.push("23:55:00", "↩️", format!("Withdrew file from altar: {}", path), ProphecyLevel::Maya);
                        }
                    }
                    self.refresh_git_state();
                    Task::none()
                }
                RightAltarMessage::StageAll => {
                    if let Some(ref s) = self.sanctuary {
                        if let Err(e) = s.stage_all() {
                            self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed staging all: {}", e), ProphecyLevel::Xibalba);
                        } else {
                            self.bottom_prophecy.push("23:55:00", "✨", "All offerings consecrated to the Altar".into(), ProphecyLevel::Jade);
                        }
                    }
                    self.refresh_git_state();
                    Task::none()
                }
                RightAltarMessage::UnstageAll => {
                    if let Some(ref s) = self.sanctuary {
                        if let Err(e) = s.unstage_all() {
                            self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed unstaging all: {}", e), ProphecyLevel::Xibalba);
                        } else {
                            self.bottom_prophecy.push("23:55:00", "↩️", "All offerings returned to working tree".into(), ProphecyLevel::Maya);
                        }
                    }
                    self.refresh_git_state();
                    Task::none()
                }
                RightAltarMessage::DiscardAll => {
                    self.bottom_prophecy.push("23:55:00", "⚠️", "Purification ceremony performed (changes discarded)".into(), ProphecyLevel::Xibalba);
                    self.refresh_git_state();
                    Task::none()
                }
                RightAltarMessage::CommitSummaryChanged(s) => {
                    self.right_altar.commit_summary = s;
                    Task::none()
                }
                RightAltarMessage::CommitDescriptionChanged(d) => {
                    self.right_altar.commit_description = d;
                    Task::none()
                }
                RightAltarMessage::InscribeCommit => {
                    let summary = self.right_altar.commit_summary.trim().to_string();
                    let desc = self.right_altar.commit_description.trim().to_string();

                    if summary.is_empty() {
                        self.bottom_prophecy.push("23:55:00", "⚠️", "Commit summary cannot be empty in sacred stone!".into(), ProphecyLevel::Xibalba);
                        return Task::none();
                    }

                    if let Some(ref s) = self.sanctuary {
                        match s.create_commit(&summary, &desc) {
                            Ok(oid) => {
                                let short = oid[..7.min(oid.len())].to_string();
                                self.bottom_prophecy.push("23:55:00", "🔥", format!("Sealed in stone: [{}] '{}'", short, summary), ProphecyLevel::Solar);
                                self.right_altar.commit_summary.clear();
                                self.right_altar.commit_description.clear();
                            }
                            Err(e) => {
                                self.bottom_prophecy.push("23:55:00", "⚠️", format!("Failed creating commit: {}", e), ProphecyLevel::Xibalba);
                            }
                        }
                    }

                    self.refresh_git_state();
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
            Message::Palette(pal_msg) => match pal_msg {
                PaletteMessage::Open => {
                    self.palette.is_open = true;
                    self.palette.search_query.clear();
                    self.palette.selected_index = 0;
                    Task::none()
                }
                PaletteMessage::Close => {
                    self.palette.is_open = false;
                    Task::none()
                }
                PaletteMessage::QueryChanged(q) => {
                    self.palette.search_query = q;
                    self.palette.selected_index = 0;
                    Task::none()
                }
                PaletteMessage::SelectPrevious => {
                    if self.palette.selected_index > 0 {
                        self.palette.selected_index -= 1;
                    }
                    Task::none()
                }
                PaletteMessage::SelectNext => {
                    let total = self.palette.filtered_commands().len();
                    if total > 0 && self.palette.selected_index + 1 < total {
                        self.palette.selected_index += 1;
                    }
                    Task::none()
                }
                PaletteMessage::ExecuteSelected => {
                    let filtered = self.palette.filtered_commands();
                    if let Some(cmd) = filtered.get(self.palette.selected_index) {
                        let id = cmd.id;
                        return Task::done(Message::Palette(PaletteMessage::ExecuteCommand(id)));
                    }
                    Task::none()
                }
                PaletteMessage::ExecuteCommand(id) => {
                    self.palette.is_open = false;
                    match id {
                        "tab_dag" => {
                            self.center_loom.active_tab = CenterTab::YaxcheDag;
                            self.bottom_prophecy.push("23:55:00", "🌳", "Transported to Yaxché Commit DAG".to_string(), ProphecyLevel::Solar);
                        }
                        "tab_diff" => {
                            self.center_loom.active_tab = CenterTab::StoneDiff;
                            self.bottom_prophecy.push("23:55:00", "🔪", "Transported to Altar of Creation (Diff)".to_string(), ProphecyLevel::Jade);
                        }
                        "tab_rebase" => {
                            self.center_loom.active_tab = CenterTab::RebaseCrucible;
                            self.bottom_prophecy.push("23:55:00", "🔄", "Transported to K'uk'ulkan Rebase Crucible".to_string(), ProphecyLevel::Solar);
                        }
                        "tab_conflict" => {
                            self.center_loom.active_tab = CenterTab::RebaseCrucible;
                            self.bottom_prophecy.push("23:55:00", "⚖️", "Transported to Three-Way Conflict Altar".to_string(), ProphecyLevel::Maya);
                        }
                        "tab_blame" => {
                            self.center_loom.active_tab = CenterTab::AhauBlame;
                            self.bottom_prophecy.push("23:55:00", "👑", "Transported to The Ahau Lineage (Blame)".to_string(), ProphecyLevel::Solar);
                        }
                        "tab_log" => {
                            self.center_loom.active_tab = CenterTab::PopolVuhLog;
                            self.bottom_prophecy.push("23:55:00", "📜", "Transported to The Popol Vuh Chronicle (Log)".to_string(), ProphecyLevel::Solar);
                        }
                        "tab_reflog" => {
                            self.center_loom.active_tab = CenterTab::XibalbaReflog;
                            self.bottom_prophecy.push("23:55:00", "🪞", "Transported to The Xibalba Mirror (Reflog)".to_string(), ProphecyLevel::Xibalba);
                        }
                        "tab_merge" => {
                            self.center_loom.active_tab = CenterTab::KukulcanMerge;
                            self.bottom_prophecy.push("23:55:00", "🌿", "Transported to K'uk'ulkan Confluence (Merge)".to_string(), ProphecyLevel::Jade);
                        }
                        "tab_clean" => {
                            self.center_loom.active_tab = CenterTab::ChacPurifier;
                            self.bottom_prophecy.push("23:55:00", "🌧️", "Transported to The Chac Purifier (Clean)".to_string(), ProphecyLevel::Maya);
                        }
                        "tab_remote" => {
                            self.center_loom.active_tab = CenterTab::OverseerPortals;
                            self.bottom_prophecy.push("23:55:00", "🌐", "Transported to The Overseer Portals (Remotes)".to_string(), ProphecyLevel::Maya);
                        }
                        "tab_oracle" => {
                            self.center_loom.active_tab = CenterTab::OracleRadar;
                            self.bottom_prophecy.push("23:55:00", "🔮", "Transported to Ixchel Oracle Radar".to_string(), ProphecyLevel::Jade);
                            return Task::done(Message::CenterLoom(CenterLoomMessage::Oracle(OracleMessage::RefreshTelemetry)));
                        }
                        "action_sync" => {
                            return Task::done(Message::CenterLoom(CenterLoomMessage::Oracle(OracleMessage::TriggerDeepSync)));
                        }
                        "action_branch" => {
                            self.left_codex.is_sprouting_branch = true;
                            self.left_codex.is_collapsed = false;
                        }
                        "action_tag" => {
                            self.left_codex.is_inscribing_tag = true;
                            self.left_codex.is_collapsed = false;
                        }
                        "action_stash" => {
                            return Task::done(Message::LeftCodex(LeftCodexMessage::CreateStash));
                        }
                        "action_cycle_bg" => {
                            self.background_mode = self.background_mode.next();
                            self.top_menu.background_mode = self.background_mode;
                            self.config.background_mode = match self.background_mode {
                                BackgroundMode::ProceduralLoom => "procedural".to_string(),
                                BackgroundMode::StaticArtwork => "static".to_string(),
                                BackgroundMode::StealthObsidian => "stealth".to_string(),
                                BackgroundMode::VideoLoop => "video".to_string(),
                            };
                            let _ = self.config.save();
                        }
                        "action_toggle_codex" => {
                            self.left_codex.is_collapsed = !self.left_codex.is_collapsed;
                        }
                        "action_shortcuts" => {
                            self.palette.is_help_open = true;
                        }
                        _ => {}
                    }
                    Task::none()
                }
                PaletteMessage::ToggleHelp => {
                    self.palette.is_help_open = !self.palette.is_help_open;
                    Task::none()
                }
                PaletteMessage::CloseHelp => {
                    self.palette.is_help_open = false;
                    Task::none()
                }
            },
            Message::KeyboardEvent(iced::keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                use iced::keyboard::key::Named;
                use iced::keyboard::Key;

                // 1. Escape: Close palette or help
                if matches!(key, Key::Named(Named::Escape)) {
                    if self.palette.is_open || self.palette.is_help_open {
                        self.palette.is_open = false;
                        self.palette.is_help_open = false;
                        return Task::none();
                    }
                }

                // 2. F1 or '?': Toggle help modal
                if matches!(key, Key::Named(Named::F1)) || (matches!(&key, Key::Character(c) if c == "?") && !self.palette.is_open) {
                    self.palette.is_help_open = !self.palette.is_help_open;
                    return Task::none();
                }

                // 3. Ctrl + P or Ctrl + K: Toggle Command Palette
                if modifiers.control() {
                    if let Key::Character(c) = &key {
                        let lower = c.to_lowercase();
                        if lower == "p" || lower == "k" {
                            self.palette.is_open = !self.palette.is_open;
                            self.palette.search_query.clear();
                            self.palette.selected_index = 0;
                            return Task::none();
                        } else if lower == "b" {
                            self.left_codex.is_collapsed = !self.left_codex.is_collapsed;
                            return Task::none();
                        } else if lower == "l" {
                            self.background_mode = self.background_mode.next();
                            self.top_menu.background_mode = self.background_mode;
                            self.config.background_mode = match self.background_mode {
                                BackgroundMode::ProceduralLoom => "procedural".to_string(),
                                BackgroundMode::StaticArtwork => "static".to_string(),
                                BackgroundMode::StealthObsidian => "stealth".to_string(),
                                BackgroundMode::VideoLoop => "video".to_string(),
                            };
                            let _ = self.config.save();
                            return Task::none();
                        } else if lower == "1" {
                            self.center_loom.active_tab = CenterTab::YaxcheDag;
                            return Task::none();
                        } else if lower == "2" {
                            self.center_loom.active_tab = CenterTab::StoneDiff;
                            return Task::none();
                        } else if lower == "3" {
                            self.center_loom.active_tab = CenterTab::RebaseCrucible;
                            return Task::none();
                        } else if lower == "4" {
                            self.center_loom.active_tab = CenterTab::KukulcanMerge;
                            return Task::none();
                        } else if lower == "5" {
                            self.center_loom.active_tab = CenterTab::AhauBlame;
                            return Task::none();
                        } else if lower == "6" {
                            self.center_loom.active_tab = CenterTab::PopolVuhLog;
                            return Task::none();
                        } else if lower == "7" {
                            self.center_loom.active_tab = CenterTab::XibalbaReflog;
                            return Task::none();
                        } else if lower == "8" {
                            self.center_loom.active_tab = CenterTab::KukulcanMerge;
                            return Task::none();
                        } else if lower == "9" {
                            self.center_loom.active_tab = CenterTab::ChacPurifier;
                            return Task::none();
                        } else if lower == "0" {
                            self.center_loom.active_tab = CenterTab::OverseerPortals;
                            return Task::none();
                        } else if lower == "i" {
                            self.center_loom.active_tab = CenterTab::OracleRadar;
                            return Task::done(Message::CenterLoom(CenterLoomMessage::Oracle(OracleMessage::RefreshTelemetry)));
                        }
                    }
                }

                Task::none()
            }
            Message::KeyboardEvent(_) => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.stage == AppStage::Splash {
            return self.view_splash();
        }

        // 1. Layer 0: The Dynamic Background
        let background_layer: Element<'_, Message> = match self.background_mode {
            BackgroundMode::ProceduralLoom => self.loom.view(1920.0, 1080.0),
            BackgroundMode::StaticArtwork => {
                image(self.static_bg_handle.clone())
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .content_fit(ContentFit::Cover)
                    .into()
            }
            BackgroundMode::VideoLoop => {
                if !self.video_frames.is_empty() {
                    image(self.video_frames[self.video_frame_index].clone())
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

        let mut layers = vec![background_layer, foreground.into()];

        if let Some(pal_view) = self.palette.view() {
            layers.push(pal_view.map(Message::Palette));
        }

        iced::widget::Stack::with_children(layers)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn view_splash(&self) -> Element<'_, Message> {
        let bg: Element<'_, Message> = if !self.video_frames.is_empty() {
            image(self.video_frames[self.video_frame_index].clone())
                .width(Length::Fill)
                .height(Length::Fill)
                .content_fit(ContentFit::Cover)
                .into()
        } else {
            self.loom.view(1920.0, 1080.0)
        };

        let vignette = container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.65, ..MayaColors::OBSIDIAN_VOID }.into()),
                ..Default::default()
            });

        let title_row = row![
            text("🔱 ITZAMNÁ").size(24).color(MayaColors::SOLAR_GOLD),
            text(" · ").size(24).color(MayaColors::GLYPH_DIM),
            text("GIT STUDIO").size(24).color(MayaColors::JADE_GLOW),
        ]
        .align_y(Alignment::Center);

        let subtitle = text("Maximalist GPU-Accelerated Architecture · Mesoamerican Cosmological Engine")
            .size(11)
            .color(MayaColors::PARCHMENT_MUTED);

        let glyph_divider = text("· ☀️ · 🌿 · ⚖️ · 🔮 · 🩸 · 🏛️ · ⚡ ·")
            .size(12)
            .color(MayaColors::GLYPH_DIM);

        let bar_width = 460.0;
        let fill_width = (bar_width * self.splash_progress).max(4.0);

        let progress_fill = container(Space::new())
            .width(fill_width)
            .height(10)
            .style(|_theme| container::Style {
                background: Some(MayaColors::SOLAR_GOLD.into()),
                border: Border {
                    radius: 4.0.into(),
                    ..Default::default()
                },
                ..Default::default()
            });

        let progress_bar = container(progress_fill)
            .width(bar_width)
            .height(14)
            .padding(Padding::from(2))
            .style(|_theme| container::Style {
                background: Some(MayaColors::OBSIDIAN_BASE.into()),
                border: Border {
                    color: MayaColors::JADE_GLOW,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            });

        let percentage = text(format!("{:.0}%", self.splash_progress * 100.0))
            .size(11)
            .color(MayaColors::SOLAR_AMBER);

        let status_label = text(self.splash_status)
            .size(11)
            .color(MayaColors::STARLIGHT_WHITE);

        let enter_button = button(
            row![
                text("☀️ ENTER SANCTUARY").size(12).color(MayaColors::OBSIDIAN_VOID),
                text(" (CLICK TO SKIP)").size(9).color(Color { a: 0.7, ..MayaColors::OBSIDIAN_VOID }),
            ]
            .align_y(Alignment::Center)
        )
        .padding(Padding::from([8, 20]))
        .style(|_theme, status| {
            let bg = match status {
                button::Status::Hovered => MayaColors::SOLAR_AMBER,
                _ => MayaColors::SOLAR_GOLD,
            };
            button::Style {
                background: Some(bg.into()),
                border: Border {
                    color: MayaColors::SOLAR_AMBER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            }
        })
        .on_press(Message::EnterStudio);

        let splash_card = container(
            column![
                title_row,
                Space::new().height(4),
                subtitle,
                Space::new().height(12),
                glyph_divider,
                Space::new().height(16),
                progress_bar,
                Space::new().height(6),
                row![
                    status_label,
                    Space::new().width(Length::Fill),
                    percentage,
                ]
                .align_y(Alignment::Center)
                .width(bar_width),
                Space::new().height(18),
                enter_button,
            ]
            .align_x(Alignment::Center)
            .spacing(4)
            .padding(Padding::from([28, 36]))
        )
        .style(|_theme| container::Style {
            background: Some(Color { a: 0.92, ..MayaColors::OBSIDIAN_BASE }.into()),
            border: Border {
                color: MayaColors::SOLAR_GOLD,
                width: 1.5,
                radius: 8.0.into(),
            },
            ..Default::default()
        });

        let centered_card = container(splash_card)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill);

        stack![
            bg,
            vignette,
            centered_card,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
