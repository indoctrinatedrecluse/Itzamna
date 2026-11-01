# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

---

## [0.1.0] - 2026-11-01

### Added
- **Built-in K'atun Code Editor (`ui/code_editor.rs`):**
  - In-app text editor with syntax highlighting, line numbers, and modification state tracking.
  - Dedicated "Save (`Ctrl+S`)" and "Save & Consecrate (Save & Stage `git add`)" actions for instant micro-edits.
- **Workspace Relics File Explorer (`ui/left_codex.rs`):**
  - Working tree file browser embedded in the Left Codex with file-type glyphs and click-to-edit integration.
  - Instant toggle between Git Glyphs (Branches/Tags/Stashes) and Workspace Relics (Files).
- **Sacred Commit Amendment Rituals (`git commit --amend`):**
  - Interactive "Amend HEAD Inscription" toggle in the Altar of Creation pre-populating summary and body.
  - "Amend Kin" button in the Commit DAG and Popol Vuh Chronicle to reword or restage past commits.
  - Post-commit follow-up banner with one-click "Push Consecrated Commits to Remote Horizon".
- **Sanctuary Git Credentials Modal (`ui/git_credentials.rs`):**
  - Local repository Git configuration editor (`user.name`, `user.email`, `remote.origin.url`) writing directly to `.git/config`.
- **Celestial Workflows Altar (`ui/workflow_runs.rs`):**
  - Live GitHub Actions CI/CD pipeline viewer powered by GitHub CLI (`gh run list`).
  - Color-coded conclusion status badges (`Success`, `Failure`, `In Progress`, `Cancelled`), event triggers, and commit SHA links.
  - One-click GitHub workflow run URL launcher and optional `winget` installation prompt if `gh` CLI is absent.
- **Top Menu Navigation & Synchronous Git Rites:**
  - `📂 Open Sanctuary`: Native OS folder picker (`rfd::FileDialog`) to open any Git repository.
  - `⬇️ Pull`: Instant `git pull --rebase` against upstream branch.
  - `⬆️ Push`: Immediate `git push origin` for local branches.
  - `⚙️ Identity`: Local Git credentials inspector and editor.
- **100% Solid Opaque Mayan Tooltips (`ui/tooltip.rs`):**
  - Replaced translucent default tooltips with obsidian-void backdrops and solar-gold borders for high readability against dynamic canvases.
- **Enhanced UI Contrast & Opacity:**
  - Elevated panel base opacity across all chambers to ~0.94, guaranteeing sharp text and control contrast while subtly revealing cosmic video motion.
- **Installer Non-Blocking Video Window:**
  - Repositioned `video_player.ps1` to the top-right screen corner (`460x260`), leaving the Inno Setup directory selection wizard completely unobstructed.
  - Added optional GitHub CLI (`gh`) winget installation task to the installer wizard.

- **Dual-Project Sovereign Architecture:**
  - **Itzamná Desktop Studio (`itzamna-desktop`):** Built with Rust 2024 and Iced 0.13, delivering hardware-accelerated GPU graphics, responsive layout widgets, and deep `git2` integration.
  - **Ixchel Oracle Daemon (`ixchel`):** Built with Scala 3.3, ZIO 2, and ZIO HTTP, exposing high-performance reactive REST telemetry and DORA analytics on port 9090.
- **Mesoamerican Cosmological Theme & Design System:**
  - Bespoke color palette: Obsidian Void (`#090A0E`), Panel Base (`#101219`), Solar Gold (`#FFD700`), Luminous Jade (`#00F299`), Cinnabar Warning (`#FF4759`), and Celestial Maya Blue (`#48CAE4`).
  - Thematic nomenclature adhering to Mayan cosmology across all tools, modals, and status reports.
- **Cinematic Background Engine & Splash Screen:**
  - Multi-mode background switcher: Procedural Cosmic Loom (GPU Canvas), Static Artwork (`background.jpg`), Stealth Obsidian, and Live Video Loop (`background.mp4`).
  - Pre-cached video frame streaming via `video_loop.bin` delivering instantaneous, zero-lag 60 FPS playback.
  - Consecrated startup splash screen featuring live background video playback, solar gold progress bar, Mayan glyph dividers, and click-to-skip functionality.
  - Sacred application icon (`icon.ico`, `icon.png`) integrated into Windows taskbar, window chrome, and installer.
- **Panoramic Command Console (`El Feed de Profecías`):**
  - Full viewport width command console eliminating unused space.
  - Three coordinated console sections:
    1. **Telemetry Vectors Panel (Left):** Real-time counters for Commits, References, Ixchel Conduit, and Alerts.
    2. **Panoramic Log Stream (Center):** Full-width timestamped event log with level pills (`SOLAR`, `JADE`, `MAYA`, `XIBALBA`), celestial glyphs, and auto-wrapping messages.
    3. **Height Modes & Filter Tools (Right):** Dynamic height toggles for Minimized (28px), Standard (96px), and Expanded (180px), combined with instant severity filters and feed purge controls.
- **Expansive 3-Column Commit Inspection Chamber:**
  - Dynamically balanced DAG (`FillPortion(3)`) and Inspector (`FillPortion(2)`) with collapsible 28px hint mode when unselected.
  - **Header Strip:** Kin signet, commit summary, author, relative time, git churn metrics (`+X / -Y`), and collapse toggle (`✕`).
  - **Column 1 — Ancestral Lineage & ODB Records:** Exact author timestamp, author and committer emails, tree OID, and interactive parent commit navigation buttons (`⬆️ parent_sha`).
  - **Column 2 — Sacred Inscription & Prophetic Record:** Dedicated reader auto-parsing full commit messages using native Mayan Markdown.
  - **Column 3 — Altered Relics & Tree Deltas:** Complete scrollable list of changed files with `[A]` (Jade), `[M]` (Amber), `[D]` (Cinnabar), and `[R]` (Blue) status badges, addition/deletion stats, and direct `DIFF ➔` jump buttons.
- **Native Mayan Markdown Parser (`ui/markdown.rs`):**
  - Comprehensive Markdown formatting support:
    - Headings (H1–H4) with gold/jade relief lines and hierarchy.
    - Blockquotes (`>`) with solar gold borders and obsidian card backgrounds.
    - Fenced Code Blocks (```` ``` ````) with monospaced jade glow syntax.
    - Inline Code (`` `code` ``) with obsidian badge tags.
    - Ordered and unordered list items with diamond glyphs and numeric bullets.
    - Horizontal rules (`---`) with stone-carved borders.
- **Interactive Yaxché Commit DAG (`El Telar de Yaxché`):**
  - Topological branch lanes with Kukulkan feathered-serpent Bézier curves and glowing solar seals.
  - Smooth vertical mouse-wheel scrolling (`DagMessage::Scrolled`) with viewport bounds clamping.
  - Dynamic zoom in, zoom out, and HEAD focus controls.
- **Sacred Git Workbenches:**
  - **Etz'nab' Stone Diff Workbench (`git diff`):** Surgical intraline diff viewer with hunk-level staging (`Stage Hunk`, `Unstage Hunk`, `Discard Hunk`), file staging, and character-level insertion/deletion highlights.
  - **K'uk'ulkan Rebase Crucible (`git rebase -i`):** Interactive timeline rebase sequencer with step reordering (Move Up / Move Down), action selector (`Pick`, `Reword`, `Edit`, `Squash`, `Drop`), commit message editing, and 3-way conflict resolution (`Ours`, `Theirs`, `Synthesize`).
  - **The Ahau Lineage (`git blame`):** Line-by-line file blame viewer with author names, timestamps, and commit age heatmaps.
  - **The Popol Vuh Chronicle (`git log`):** Full-text search and author filter across repository commit history with instant DAG selection.
  - **The K'uk'ulkan Confluence (`git merge` / `git cherry-pick`):** Branch merge altar with strategy selectors (`Fast-Forward`, `No Fast-Forward`, `Rebase`, `Squash`) and cherry-pick execution altar.
  - **The Xibalba Mirror (`git reflog` / `git reset`):** Visual reflog timeline with three reset dials (`Soft`, `Mixed`, `Hard`).
  - **The Chac Purifier (`git clean` / `git stash`):** Untracked file preview and purge altar, combined with multi-stash cist vault with pop, drop, and branch-sprouting capabilities.
  - **The Overseer Portals (`git remote` / `git push`):** Remote branch synchronization altar with protected `push --force-with-lease`, `pull --rebase`, and `fetch --prune`.
  - **Ixchel Oracle Radar (`DORA Metrics`):** Live telemetry client polling the ZIO HTTP daemon for Lead Time, Deployment Frequency, Change Failure Rate, Time to Restore, and repository health scores.
- **Workflow & Productivity Architecture:**
  - **Sacred Glyph Keyboard Cheat Sheet (`F1` / `?`):** Modal displaying all application keybindings.
  - **Universal Command Palette (`Ctrl+P` / `Ctrl+K`):** Fuzzy-searchable action dispatcher to navigate tabs and trigger Git rituals.
  - **Multi-Repository Switcher & Genesis Altars:** Repository selector dropdown, Clone Chamber modal (`git clone`), Genesis Altar modal (`git init`), and persistent recent repository storage in `config.toml`.
  - **Tooltip Descriptions:** Informative hover tooltips for every UI button, tab, and action.
- **Mythology-Themed Windows Installer (`installer/itzamna.iss`):**
  - Production-grade Inno Setup 6 installer (`ItzamnaSetup-v0.1.0.exe`).
  - Companion live video player window (`video_player.ps1`) playing `background.mp4` on continuous loop throughout the installation.
  - High-DPI custom wizard graphics (`wizard_large.bmp`, `wizard_small.bmp`) and application icon (`icon.ico`).
  - Custom Mayan cosmological text for all wizard pages and step buttons.
  - **Desktop shortcut creation checked by default.**
  - **Launch application on finish checked by default.**
  - **Custom installation path support.**
- **Cross-Platform CI/CD Release Workflows:**
  - Automated CI test matrix on push to `main` and pull requests (`.github/workflows/ci.yml`).
  - Automated Release workflow triggering on Git tag push (`v*`) (`.github/workflows/release.yml`), compiling Windows, Linux, and macOS binaries and publishing `ItzamnaSetup-v0.1.0.exe` directly on GitHub Releases.
