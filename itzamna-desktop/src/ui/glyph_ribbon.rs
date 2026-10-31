//! The Sacred Glyph Ribbon (El Listón de Glifos Sagrados)
//! An animated looping marquee of ancient Maya Tzolk'in hieroglyphs orbiting
//! and framing the main window menu header.

use iced::mouse;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::{Color, Element, Point, Rectangle, Renderer, Size, Theme};

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub struct GlyphCard {
    pub name: &'static str,
    pub translation: &'static str,
    pub git_analogy: &'static str,
    pub symbol_id: usize,
}

pub const SACRED_GLYPHS: [GlyphCard; 20] = [
    GlyphCard { name: "IMIX", translation: "Water Lily", git_analogy: "Init / Root", symbol_id: 0 },
    GlyphCard { name: "IK'", translation: "Wind / Breath", git_analogy: "Diff / Patch", symbol_id: 1 },
    GlyphCard { name: "AK'B'AL", translation: "Night / Void", git_analogy: "Stash Cist", symbol_id: 2 },
    GlyphCard { name: "K'AN", translation: "Seed / Creation", git_analogy: "Commit", symbol_id: 3 },
    GlyphCard { name: "CHICCHAN", translation: "Serpent", git_analogy: "Merge / Weave", symbol_id: 4 },
    GlyphCard { name: "KIMI", translation: "Death / Rebirth", git_analogy: "Prune / Reset", symbol_id: 5 },
    GlyphCard { name: "MANIK'", translation: "Grasp", git_analogy: "Cherry-Pick", symbol_id: 6 },
    GlyphCard { name: "LAMAT", translation: "Venus Star", git_analogy: "Tag Signet", symbol_id: 7 },
    GlyphCard { name: "MULUK", translation: "Sacred Water", git_analogy: "Push / Stream", symbol_id: 8 },
    GlyphCard { name: "OK", translation: "Guide Dog", git_analogy: "Reflog Trace", symbol_id: 9 },
    GlyphCard { name: "CHUWEN", translation: "Weaver / Artisan", git_analogy: "DAG Loom", symbol_id: 10 },
    GlyphCard { name: "EB'", translation: "Sacred Stair", git_analogy: "Rebase Step", symbol_id: 11 },
    GlyphCard { name: "B'EN", translation: "Pillar / Stela", git_analogy: "Worktree", symbol_id: 12 },
    GlyphCard { name: "IX", translation: "Jaguar", git_analogy: "Detached Head", symbol_id: 13 },
    GlyphCard { name: "MEN", translation: "Eagle Vision", git_analogy: "Oracle Metric", symbol_id: 14 },
    GlyphCard { name: "K'IB'", translation: "Cleansing Owl", git_analogy: "Clean Workspace", symbol_id: 15 },
    GlyphCard { name: "KAB'AN", translation: "Earth Motion", git_analogy: "Remote Fetch", symbol_id: 16 },
    GlyphCard { name: "ETZ'NAB'", translation: "Flint Blade", git_analogy: "Hunk Stage", symbol_id: 17 },
    GlyphCard { name: "KAWAK", translation: "Thundercloud", git_analogy: "Conflict Altar", symbol_id: 18 },
    GlyphCard { name: "AHAU", translation: "Lord / The Sun", git_analogy: "HEAD Sovereign", symbol_id: 19 },
];

#[derive(Debug, Clone)]
pub struct GlyphRibbon {
    pub scroll_offset: f32,
    pub speed: f32,
    pub card_width: f32,
    pub card_height: f32,
    pub card_spacing: f32,
    pub hovered_index: Option<usize>,
}

impl Default for GlyphRibbon {
    fn default() -> Self {
        Self::new()
    }
}

impl GlyphRibbon {
    pub fn new() -> Self {
        Self {
            scroll_offset: 0.0,
            speed: 28.0, // Pixels per second
            card_width: 130.0,
            card_height: 38.0,
            card_spacing: 12.0,
            hovered_index: None,
        }
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        let stride = self.card_width + self.card_spacing;
        let total_loop_width = stride * SACRED_GLYPHS.len() as f32;

        self.scroll_offset += self.speed * delta_seconds;
        if self.scroll_offset >= total_loop_width {
            self.scroll_offset -= total_loop_width;
        }
    }

    pub fn view<'a, Message: 'a>(&'a self, width: f32, height: f32) -> Element<'a, Message> {
        Canvas::new(self)
            .width(width)
            .height(height)
            .into()
    }
}

impl<Message> canvas::Program<Message> for GlyphRibbon {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let stride = self.card_width + self.card_spacing;
        let total_loop_width = stride * SACRED_GLYPHS.len() as f32;

        // Base background bar for the ribbon
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            Color { a: 0.88, ..MayaColors::OBSIDIAN_BASE },
        );

        // Top and bottom stone relief separator line
        let top_border = Path::line(Point::ORIGIN, Point::new(bounds.width, 0.0));
        let bot_border = Path::line(
            Point::new(0.0, bounds.height),
            Point::new(bounds.width, bounds.height),
        );
        let border_stroke = Stroke::default()
            .with_color(MayaColors::OBSIDIAN_BORDER)
            .with_width(1.0);
        frame.stroke(&top_border, border_stroke);
        frame.stroke(&bot_border, border_stroke);

        // Calculate visible items
        let offset = self.scroll_offset;
        let mut x = -offset;
        while x < bounds.width {
            // Repeat glyph array seamlessly
            for (idx, glyph) in SACRED_GLYPHS.iter().enumerate() {
                let card_x = x;
                let card_y = (bounds.height - self.card_height) * 0.5;

                if card_x + self.card_width > 0.0 && card_x < bounds.width {
                    let card_rect = Rectangle::new(
                        Point::new(card_x, card_y),
                        Size::new(self.card_width, self.card_height),
                    );

                    let is_hovered = cursor
                        .position_in(bounds)
                        .map(|pos| card_rect.contains(pos))
                        .unwrap_or(false);

                    self.draw_cartouche(&mut frame, card_rect, glyph, is_hovered, idx);
                }

                x += stride;
                if x >= bounds.width + total_loop_width {
                    break;
                }
            }
        }

        vec![frame.into_geometry()]
    }
}

impl GlyphRibbon {
    fn draw_cartouche(
        &self,
        frame: &mut Frame,
        rect: Rectangle,
        glyph: &GlyphCard,
        is_hovered: bool,
        index: usize,
    ) {
        // 1. Stone Cartouche Card Body
        let bg_color = if is_hovered {
            Color { a: 0.95, ..MayaColors::OBSIDIAN_SURFACE }
        } else {
            Color { a: 0.65, ..MayaColors::OBSIDIAN_BASE }
        };

        let border_color = if is_hovered {
            MayaColors::JADE_GLOW
        } else if index == 19 {
            // Ahau / Sun sovereign
            MayaColors::SOLAR_GOLD
        } else {
            MayaColors::OBSIDIAN_BORDER
        };

        // Draw pill-shaped stone cartouche
        let cartouche = Path::rounded_rectangle(rect.position(), rect.size(), 5.0.into());
        frame.fill(&cartouche, bg_color);
        frame.stroke(
            &cartouche,
            Stroke::default()
                .with_color(border_color)
                .with_width(if is_hovered { 1.5 } else { 1.0 }),
        );

        // 2. Mayan Hieroglyph Symbol Icon (Procedural Geometric Seal)
        let icon_box_size = 24.0;
        let icon_center = Point::new(
            rect.x + 6.0 + icon_box_size * 0.5,
            rect.y + rect.height * 0.5,
        );
        self.draw_glyph_symbol(frame, icon_center, glyph.symbol_id, is_hovered);

        // 3. Glyph Sacred Name
        let name_color = if is_hovered {
            MayaColors::SOLAR_GOLD
        } else if index == 19 {
            MayaColors::SOLAR_GOLD
        } else {
            MayaColors::STARLIGHT_WHITE
        };

        frame.fill_text(Text {
            content: glyph.name.to_string(),
            position: Point::new(rect.x + 36.0, rect.y + 4.0),
            color: name_color,
            size: 11.0.into(),
            line_height: iced::widget::text::LineHeight::Relative(1.0),
            shaping: iced::widget::text::Shaping::Basic,
            ..Default::default()
        });

        // 4. Git Analogy / Function Subtitle
        let sub_color = if is_hovered {
            MayaColors::JADE_GLOW
        } else {
            MayaColors::GLYPH_DIM
        };

        frame.fill_text(Text {
            content: glyph.git_analogy.to_string(),
            position: Point::new(rect.x + 36.0, rect.y + 20.0),
            color: sub_color,
            size: 9.0.into(),
            line_height: iced::widget::text::LineHeight::Relative(1.0),
            shaping: iced::widget::text::Shaping::Basic,
            ..Default::default()
        });

        // 5. Tiny Jade Corner Bead
        if is_hovered {
            let bead = Path::circle(Point::new(rect.x + rect.width - 6.0, rect.y + 6.0), 2.5);
            frame.fill(&bead, MayaColors::JADE_GLOW);
        }
    }

    fn draw_glyph_symbol(&self, frame: &mut Frame, center: Point, symbol_id: usize, is_hovered: bool) {
        let color = if is_hovered {
            MayaColors::JADE_GLOW
        } else {
            MayaColors::SOLAR_AMBER
        };

        // Draw characteristic geometric Maya logograms
        match symbol_id % 5 {
            0 => {
                // Sacred Sun Eye (Ahau / Imix)
                let eye_outer = Path::circle(center, 9.0);
                frame.stroke(&eye_outer, Stroke::default().with_color(color).with_width(1.2));
                let pupil = Path::circle(center, 3.5);
                frame.fill(&pupil, color);
            }
            1 => {
                // The T-Shaped Wind Tau (Ik')
                let tau = Path::new(|b| {
                    b.move_to(Point::new(center.x - 7.0, center.y - 6.0));
                    b.line_to(Point::new(center.x + 7.0, center.y - 6.0));
                    b.move_to(Point::new(center.x, center.y - 6.0));
                    b.line_to(Point::new(center.x, center.y + 7.0));
                });
                frame.stroke(&tau, Stroke::default().with_color(color).with_width(2.0));
            }
            2 => {
                // Feathered Serpent Coil (Chicchan)
                let snake = Path::new(|b| {
                    b.move_to(Point::new(center.x - 7.0, center.y + 5.0));
                    b.bezier_curve_to(
                        Point::new(center.x - 4.0, center.y - 8.0),
                        Point::new(center.x + 4.0, center.y + 8.0),
                        Point::new(center.x + 7.0, center.y - 5.0),
                    );
                });
                frame.stroke(&snake, Stroke::default().with_color(color).with_width(1.8));
            }
            3 => {
                // Kan Solar Seed Cross
                let ring = Path::circle(center, 8.5);
                frame.stroke(&ring, Stroke::default().with_color(color).with_width(1.0));
                let h_cross = Path::line(
                    Point::new(center.x - 5.5, center.y),
                    Point::new(center.x + 5.5, center.y),
                );
                let v_cross = Path::line(
                    Point::new(center.x, center.y - 5.5),
                    Point::new(center.x, center.y + 5.5),
                );
                frame.stroke(&h_cross, Stroke::default().with_color(color).with_width(1.5));
                frame.stroke(&v_cross, Stroke::default().with_color(color).with_width(1.5));
            }
            _ => {
                // Stepped Pyramid Chevron
                let chevron = Path::new(|b| {
                    b.move_to(Point::new(center.x - 7.0, center.y + 5.0));
                    b.line_to(Point::new(center.x, center.y - 6.0));
                    b.line_to(Point::new(center.x + 7.0, center.y + 5.0));
                });
                frame.stroke(&chevron, Stroke::default().with_color(color).with_width(1.8));
            }
        }
    }
}
