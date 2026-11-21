# Itzamna Git Studio

Itzamna is a native Rust desktop application for inspecting and working with local Git repositories. It uses Iced for the UI and `git2` for repository operations.

## Features

- Topological commit graph with multi-parent merge routing, branch heads, and tags.
- Working tree and index views, including individual hunk stage, unstage, and discard operations.
- Interactive rebase planning and conflict resolution.
- Branch, stash, reflog, blame, and remote workbenches.
- Asynchronous push, protected force push, pull/rebase, and fetch/prune operations.
- Local repository telemetry calculated from commits, diffs, release tags, and author history. Lead time and change failure rate use repository heuristics; recovery time is reported as unavailable because Git history alone does not provide incident recovery data.
- CommonMark rendering for commit descriptions and documentation, including tables, task lists, emphasis, code, and clickable HTTP(S) links.
- Hierarchical file explorer and a text editor with Ctrl+S / Command+S support.

## Build and run

Install the stable Rust toolchain. On Linux, install the native graphics and audio development packages used by Iced (X11/Wayland, XKB, ALSA, and pkg-config).

```powershell
cargo run --manifest-path itzamna-desktop/Cargo.toml
cargo test --manifest-path itzamna-desktop/Cargo.toml
```

On Linux and macOS, use the same Cargo commands from a shell with the platform’s native development libraries installed.

## Assets

Desktop assets are maintained in `itzamna-desktop/assets`. Video frames are loaded on a background worker after the window starts, and the application samples a bounded set of frames to limit startup work and memory use.

## Packaging

The release workflow builds and tests Windows, Linux, and macOS targets. The Inno Setup installer and `scripts/package_release.ps1` create the Windows distribution.

## License

MIT. See [LICENSE](LICENSE).
