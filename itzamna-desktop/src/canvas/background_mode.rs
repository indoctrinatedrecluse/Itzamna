//! Background Render Modes for the Main Canvas
//! Supports procedural vector GPU shader, static Google Flow artwork,
//! video playback loop, and minimal obsidian mode.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundMode {
    ProceduralLoom,
    StaticArtwork,
    VideoLoop,
    StealthObsidian,
}

impl BackgroundMode {
    pub fn next(&self) -> Self {
        match self {
            Self::ProceduralLoom => Self::StaticArtwork,
            Self::StaticArtwork => Self::VideoLoop,
            Self::VideoLoop => Self::StealthObsidian,
            Self::StealthObsidian => Self::ProceduralLoom,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::ProceduralLoom => "✨ Loom: Procedural",
            Self::StaticArtwork => "🖼️ Loom: Static Art",
            Self::VideoLoop => "🎬 Loom: Video Stream",
            Self::StealthObsidian => "🌑 Loom: Stealth Void",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ProceduralLoom => "Live GPU/Vector Tzolk'in gears & stardust motes (Active 60 FPS)",
            Self::StaticArtwork => "Static high-res Google Flow temple backdrop (Low Power / Battery Saver)",
            Self::VideoLoop => "Streaming Google Flow cosmic motion video loop",
            Self::StealthObsidian => "Zero distraction deep obsidian stone relief",
        }
    }
}
