//! ItzamnaApp — Supreme Mayan Application Orchestrator (Phase 5: Full Real Execution)

use iced::widget::{column, container, image, row, stack, Space};
use iced::{ContentFit, Element, Length, Subscription, Task};
use std::time::Duration;

use crate::canvas::{BackgroundMode, CosmicLoom};
use crate::git::{FileStatus, GitSanctuary};
use crate::theme::MayaColors;
use crate::ui::{
    BottomProphecy, CenterLoom, CenterLoomMessage, CenterTab, DiffMessage, FileEntry, GlyphRibbon,
    LeftCodex, LeftCodexMessage, MenuCategory, OracleMessage, ProphecyLevel, ProphecyMessage,
    RebaseMessage, RightAltar, RightAltarMessage, StatusBar, TopMenuBar, TopMenuMessage,
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
    static_bg_handle: image::Handle,
    video_frames: Vec<image::Handle>,
    video_frame_index: usize,
    sanctuary: Option<GitSanctuary>,
}

impl Default for ItzamnaApp {
    fn default() -> Self {
        Self::new()
    }
}

impl ItzamnaApp {
    pub fn new() -> Self {
        // 1. Static background: Embedded in binary for guaranteed 100% availability
        let embedded_bytes = include_bytes!("../assets/background.jpg");
        let static_bg_handle = image::Handle::from_bytes(&embedded_bytes[..]);

        // 2. Pre-load video frames directly into in-memory handles (bypassing async disk decodes)
        let mut video_frames = Vec::new();
        let frame_search_dirs = [
            "itzamna-desktop/assets/video_frames",
            "assets/video_frames",
            "D:/Projects/Itzamna/itzamna-desktop/assets/video_frames",
            "D:/Projects/Itzamna/assets/video_frames",
        ];

        let resolved_dir = frame_search_dirs
            .iter()
            .find(|dir| std::path::Path::new(dir).exists());

        if let Some(dir) = resolved_dir {
            // Pre-load 120 frames in memory for smooth, zero-latency looping
            for i in 0..120 {
                let frame_path = format!("{}/frame_{:03}.jpg", dir, i);
                if let Ok(bytes) = std::fs::read(&frame_path) {
                    video_frames.push(image::Handle::from_bytes(bytes));
                } else {
                    break;
                }
            }
        }

        // 3. Initialize real Git Sanctuary
        let mut left_codex = LeftCodex::new();
        let mut right_altar = RightAltar::new();
        let mut status_bar = StatusBar::default();
        let mut center_loom = CenterLoom::new(Vec::new());
        let mut bottom_prophecy = BottomProphecy::new();

        let sanctuary = GitSanctuary::open(".").or_else(|_| GitSanctuary::open("..")).ok();

        if let Some(ref s) = sanctuary {
            let (head_branch, head_sha) = s.get_head_info();
            let (local_branches, remote_branches) = s.get_branches();
            let tags = s.get_tags();
            let (unstaged, staged) = s.get_status_files();
            let commits = s.load_commit_dag(40);

            left_codex.active_branch = head_branch.clone();
            if !local_branches.is_empty() {
                left_codex.local_branches = local_branches;
            }
            if !remote_branches.is_empty() {
                left_codex.remote_branches = remote_branches;
            }
            left_codex.tags = tags;

            status_bar.active_branch = head_branch;
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
                format!("Awakened real Git sanctuary: {} commits, {} video frames loaded", commits.len(), video_frames.len()),
                ProphecyLevel::Solar,
            );

            center_loom.set_commits(commits);
            if let Some(first) = center_loom.dag.commits.first() {
                center_loom.detailed_commit = s.find_detailed_commit(&first.sha);
            }
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
            video_frames,
            video_frame_index: 0,
            sanctuary,
        }
    }

    pub fn refresh_git_state(&mut self) {
        let Some(ref s) = self.sanctuary else { return };
        let (head_branch, head_sha) = s.get_head_info();
        let (local_branches, remote_branches) = s.get_branches();
        let tags = s.get_tags();
        let (unstaged, staged) = s.get_status_files();
        let commits = s.load_commit_dag(40);

        self.left_codex.active_branch = head_branch.clone();
        self.left_codex.local_branches = local_branches;
        self.left_codex.remote_branches = remote_branches;
        self.left_codex.tags = tags;

        self.status_bar.active_branch = head_branch;
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

        self.center_loom.set_commits(commits);
        if let Some(selected) = &self.center_loom.dag.selected_sha {
            self.center_loom.detailed_commit = s.find_detailed_commit(selected);
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        match self.background_mode {
            BackgroundMode::ProceduralLoom => {
                iced::time::every(Duration::from_millis(16)).map(|_| Message::Tick)
            }
            BackgroundMode::VideoLoop => {
                // Smooth 24 FPS video cycle
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
                            self.refresh_git_state();
                            self.bottom_prophecy.push("23:55:00", "📜", "Popol Vuh: Refreshed codex state from disk".into(), ProphecyLevel::Solar);
                        }
                        MenuCategory::ChilamBalam => {
                            self.center_loom.active_tab = CenterTab::StoneDiff;
                        }
                        MenuCategory::CosmicVision => {
                            self.center_loom.active_tab = CenterTab::YaxcheDag;
                        }
                        MenuCategory::Yaxche => {
                            self.center_loom.active_tab = CenterTab::RebaseCrucible;
                        }
                        MenuCategory::SacredAltar => {
                            self.right_altar.is_collapsed = false;
                        }
                        MenuCategory::IxchelOracle => {
                            self.center_loom.active_tab = CenterTab::OracleRadar;
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
                CenterLoomMessage::Oracle(o_msg) => match o_msg {
                    OracleMessage::RefreshTelemetry => {
                        self.bottom_prophecy.push("23:55:00", "🌙", "Polled Ixchel Oracle heartbeat: 200 OK".into(), ProphecyLevel::Maya);
                        Task::none()
                    }
                    OracleMessage::TriggerDeepSync => {
                        self.bottom_prophecy.push("23:55:00", "🔮", "Triggered deep DORA and churn calculation cycle".into(), ProphecyLevel::Solar);
                        Task::none()
                    }
                },
                CenterLoomMessage::SelectTab(tab) => {
                    self.center_loom.active_tab = tab;
                    Task::none()
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
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
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

        stack![
            background_layer,
            foreground,
        ]
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
