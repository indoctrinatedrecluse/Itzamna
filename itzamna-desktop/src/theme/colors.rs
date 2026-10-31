//! Obsidian & Jade Palette — Mayan Cosmic Color Harmonies

use iced::Color;

pub struct MayaColors;

impl MayaColors {
    // Obsidian & Stone Foundations
    pub const OBSIDIAN_VOID: Color = Color::from_rgb(0.035, 0.039, 0.055);      // #090A0E Deep cosmic background
    pub const OBSIDIAN_BASE: Color = Color::from_rgb(0.063, 0.071, 0.098);      // #101219 Panel background
    pub const OBSIDIAN_SURFACE: Color = Color::from_rgb(0.098, 0.110, 0.149);   // #191C26 Card surface
    pub const OBSIDIAN_BORDER: Color = Color::from_rgb(0.165, 0.184, 0.239);    // #2A2F3D Stone carved border
    pub const STELE_GREY: Color = Color::from_rgb(0.247, 0.275, 0.345);         // #3F4658 Chiseled stone accent

    // Sacred Jade (Life, Growth, Branches, Clean Trees)
    pub const JADE_GLOW: Color = Color::from_rgb(0.0, 0.95, 0.60);              // #00F299 Luminous active jade
    pub const JADE_PRIME: Color = Color::from_rgb(0.0, 0.72, 0.44);             // #00B870 Deep polished jade
    pub const JADE_SHADOW: Color = Color::from_rgb(0.0, 0.35, 0.22);            // #005938 Jade shadow

    // Sun Stone Gold (Kin, HEAD, Sovereign Commits, Solar Energy)
    pub const SOLAR_GOLD: Color = Color::from_rgb(1.0, 0.84, 0.0);              // #FFD700 Pure sovereign gold
    pub const SOLAR_AMBER: Color = Color::from_rgb(1.0, 0.70, 0.0);             // #FFB300 Warm amber halo
    pub const SOLAR_DARK: Color = Color::from_rgb(0.60, 0.42, 0.0);              // #996B00 Aged bronze/gold

    // Cinnabar & Bloodstone (Xibalba, Hot Conflicts, Deletions)
    pub const CINNABAR_BRIGHT: Color = Color::from_rgb(1.0, 0.28, 0.35);        // #FF4759 Hot warning
    pub const CINNABAR_DARK: Color = Color::from_rgb(0.65, 0.10, 0.18);          // #A61A2E Deep ritual red

    // Maya Blue & Lapis (Celestials, Remote Tracking, Waves)
    pub const MAYA_BLUE: Color = Color::from_rgb(0.28, 0.79, 0.95);             // #48CAE4 Celestial sky blue
    pub const MAYA_DEEP: Color = Color::from_rgb(0.0, 0.47, 0.71);              // #0077B5 Deep cenote blue

    // Ancient Parchment & Starlight
    pub const STARLIGHT_WHITE: Color = Color::from_rgb(0.92, 0.94, 0.98);       // #EBF0FA Starlight text
    pub const PARCHMENT_MUTED: Color = Color::from_rgb(0.65, 0.68, 0.75);       // #A6ADC0 Inactive glyphs
    pub const GLYPH_DIM: Color = Color::from_rgb(0.40, 0.43, 0.50);             // #666E80 Carved relief text
}
