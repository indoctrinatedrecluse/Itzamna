//! The Cosmic Loom (El Telar Cósmico)
//! Procedural GPU/Vector Canvas: Concentric Tzolk'in gears, rotating Mayan calendar
//! rings, drifting solar stardust, and celestial nebula webs.

use iced::mouse;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{Color, Element, Point, Rectangle, Renderer, Size, Theme};
use std::f32::consts::PI;

use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub struct CosmicLoom {
    time: f32,
    stars: Vec<StarMote>,
}

#[derive(Debug, Clone)]
struct StarMote {
    x_ratio: f32,
    y_ratio: f32,
    base_radius: f32,
    phase: f32,
    speed_y: f32,
    color: Color,
}

impl Default for CosmicLoom {
    fn default() -> Self {
        Self::new()
    }
}

impl CosmicLoom {
    pub fn new() -> Self {
        // Deterministic celestial dust motes
        let mut stars = Vec::with_capacity(72);
        for i in 0..72 {
            let fi = i as f32;
            let x_ratio = ((fi * 17.13 + 3.7) % 100.0) / 100.0;
            let y_ratio = ((fi * 29.41 + 11.3) % 100.0) / 100.0;
            let phase = fi * 0.45;
            let speed_y = 0.00015 + ((fi * 7.3) % 5.0) * 0.00005;
            let is_gold = i % 3 == 0;
            let color = if is_gold {
                MayaColors::SOLAR_GOLD
            } else if i % 2 == 0 {
                MayaColors::JADE_GLOW
            } else {
                MayaColors::MAYA_BLUE
            };

            stars.push(StarMote {
                x_ratio,
                y_ratio,
                base_radius: 1.2 + (fi % 3.0) * 0.8,
                phase,
                speed_y,
                color,
            });
        }

        Self { time: 0.0, stars }
    }

    pub fn tick(&mut self, delta_seconds: f32) {
        self.time += delta_seconds;

        // Drift star particles upward with wrap-around
        for star in &mut self.stars {
            star.y_ratio -= star.speed_y * delta_seconds * 60.0;
            if star.y_ratio < 0.0 {
                star.y_ratio += 1.0;
            }
        }
    }

    pub fn view<'a, Message: 'a>(&'a self, width: f32, height: f32) -> Element<'a, Message> {
        Canvas::new(self)
            .width(width)
            .height(height)
            .into()
    }
}

impl<Message> canvas::Program<Message> for CosmicLoom {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = Point::new(bounds.width * 0.5, bounds.height * 0.52);

        // 1. Cosmic Void Backdrop
        frame.fill_rectangle(
            Point::ORIGIN,
            bounds.size(),
            MayaColors::OBSIDIAN_VOID,
        );

        // 2. Drifting Star Motes and Nebula Web
        self.draw_celestial_dust(&mut frame, bounds.size());

        // 3. The Interlocking Tzolk'in & Haab' Sacred Gears
        self.draw_tzolkin_gears(&mut frame, center);

        // 4. Central Sun Stone Disc (Kin Ahau)
        self.draw_sun_stone(&mut frame, center);

        vec![frame.into_geometry()]
    }
}

impl CosmicLoom {
    fn draw_celestial_dust(&self, frame: &mut Frame, size: Size) {
        // Draw constellation triangulation lines between nearby stars
        let star_count = self.stars.len().min(36);
        for i in 0..star_count {
            for j in (i + 1)..star_count {
                let s1 = &self.stars[i];
                let s2 = &self.stars[j];
                let p1 = Point::new(s1.x_ratio * size.width, s1.y_ratio * size.height);
                let p2 = Point::new(s2.x_ratio * size.width, s2.y_ratio * size.height);

                let dx = p1.x - p2.x;
                let dy = p1.y - p2.y;
                let dist_sq = dx * dx + dy * dy;

                if dist_sq < 14000.0 {
                    let alpha = (1.0 - (dist_sq / 14000.0)) * 0.12;
                    let stroke_color = Color {
                        a: alpha,
                        ..MayaColors::MAYA_BLUE
                    };
                    let line = Path::line(p1, p2);
                    frame.stroke(
                        &line,
                        Stroke::default().with_color(stroke_color).with_width(0.7),
                    );
                }
            }
        }

        // Draw individual pulsing star motes
        for star in &self.stars {
            let px = star.x_ratio * size.width;
            let py = star.y_ratio * size.height;
            let pulse = ((self.time * 2.2 + star.phase).sin() * 0.5 + 0.5) * 0.8 + 0.3;
            let radius = star.base_radius * (0.8 + pulse * 0.4);

            let glow_color = Color {
                a: 0.18 * pulse,
                ..star.color
            };
            let core_color = Color {
                a: 0.75 * pulse,
                ..star.color
            };

            // Outer soft aura
            let aura = Path::circle(Point::new(px, py), radius * 2.5);
            frame.fill(&aura, glow_color);

            // Core stardust
            let core = Path::circle(Point::new(px, py), radius);
            frame.fill(&core, core_color);
        }
    }

    fn draw_tzolkin_gears(&self, frame: &mut Frame, center: Point) {
        let t = self.time;

        // Outer Gear: The Haab' Solar Wheel (365 days / 18 Uinals)
        // Rotates slowly clockwise
        let outer_radius = 290.0;
        let outer_rot = t * 0.04;
        self.draw_mayan_ring(
            frame,
            center,
            outer_radius,
            24.0,
            outer_rot,
            36,
            MayaColors::OBSIDIAN_BORDER,
            MayaColors::JADE_SHADOW,
            0.4,
        );

        // Middle Gear: The Tzolk'in Sacred 260-Day Wheel (20 Glyphs x 13 Numbers)
        // Rotates counter-clockwise
        let mid_radius = 210.0;
        let mid_rot = -t * 0.07;
        self.draw_mayan_ring(
            frame,
            center,
            mid_radius,
            18.0,
            mid_rot,
            20,
            MayaColors::JADE_PRIME,
            MayaColors::SOLAR_DARK,
            0.6,
        );

        // Inner Gear: The Lunar / Kin Cycle
        // Rotates clockwise with stepped teeth
        let inner_radius = 140.0;
        let inner_rot = t * 0.12;
        self.draw_stepped_pyramid_ring(
            frame,
            center,
            inner_radius,
            14.0,
            inner_rot,
            13,
            MayaColors::SOLAR_AMBER,
        );

        // Concentric geometric hairline circles
        for r in [90.0, 175.0, 250.0, 320.0] {
            let circle = Path::circle(center, r);
            frame.stroke(
                &circle,
                Stroke::default()
                    .with_color(Color {
                        a: 0.15,
                        ..MayaColors::JADE_GLOW
                    })
                    .with_width(0.75),
            );
        }
    }

    fn draw_mayan_ring(
        &self,
        frame: &mut Frame,
        center: Point,
        radius: f32,
        band_width: f32,
        rotation: f32,
        notches: usize,
        rim_color: Color,
        notch_color: Color,
        alpha: f32,
    ) {
        // Base ring band
        let outer_circle = Path::circle(center, radius);
        let inner_circle = Path::circle(center, radius - band_width);

        frame.stroke(
            &outer_circle,
            Stroke::default()
                .with_color(Color { a: alpha, ..rim_color })
                .with_width(1.5),
        );
        frame.stroke(
            &inner_circle,
            Stroke::default()
                .with_color(Color { a: alpha * 0.8, ..rim_color })
                .with_width(1.0),
        );

        // Radial teeth & Mayan step crenellations
        let angle_step = (2.0 * PI) / notches as f32;
        for i in 0..notches {
            let angle = rotation + i as f32 * angle_step;
            let cos_a = angle.cos();
            let sin_a = angle.sin();

            let p_inner = Point::new(
                center.x + (radius - band_width) * cos_a,
                center.y + (radius - band_width) * sin_a,
            );
            let p_outer = Point::new(
                center.x + (radius + 4.0) * cos_a,
                center.y + (radius + 4.0) * sin_a,
            );

            let spoke = Path::line(p_inner, p_outer);
            frame.stroke(
                &spoke,
                Stroke::default()
                    .with_color(Color { a: alpha * 0.9, ..notch_color })
                    .with_width(2.0),
            );

            // Small glyph dots between spokes
            let mid_angle = angle + angle_step * 0.5;
            let dot_p = Point::new(
                center.x + (radius - band_width * 0.5) * mid_angle.cos(),
                center.y + (radius - band_width * 0.5) * mid_angle.sin(),
            );
            let dot = Path::circle(dot_p, 2.0);
            frame.fill(
                &dot,
                Color { a: alpha * 0.7, ..notch_color },
            );
        }
    }

    fn draw_stepped_pyramid_ring(
        &self,
        frame: &mut Frame,
        center: Point,
        radius: f32,
        height: f32,
        rotation: f32,
        count: usize,
        color: Color,
    ) {
        let step = (2.0 * PI) / count as f32;
        for i in 0..count {
            let angle = rotation + i as f32 * step;
            let next_angle = angle + step * 0.45;

            // Draw a triangular solar flare ray
            let p1 = Point::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            );
            let p2 = Point::new(
                center.x + (radius + height) * ((angle + next_angle) * 0.5).cos(),
                center.y + (radius + height) * ((angle + next_angle) * 0.5).sin(),
            );
            let p3 = Point::new(
                center.x + radius * next_angle.cos(),
                center.y + radius * next_angle.sin(),
            );

            let triangle = Path::new(|builder| {
                builder.move_to(p1);
                builder.line_to(p2);
                builder.line_to(p3);
                builder.close();
            });

            frame.fill(
                &triangle,
                Color {
                    a: 0.35 + (i % 2) as f32 * 0.2,
                    ..color
                },
            );
        }
    }

    fn draw_sun_stone(&self, frame: &mut Frame, center: Point) {
        let t = self.time;
        let sun_radius = 62.0;

        // Radiant pulsing halo
        let pulse = (t * 2.0).sin() * 0.15 + 0.85;
        let halo = Path::circle(center, sun_radius * 1.55 * pulse);
        frame.fill(
            &halo,
            Color {
                a: 0.08 * pulse,
                ..MayaColors::SOLAR_GOLD
            },
        );

        // Outer sun ring
        let outer_disc = Path::circle(center, sun_radius);
        frame.fill(
            &outer_disc,
            Color {
                a: 0.75,
                ..MayaColors::OBSIDIAN_BASE
            },
        );
        frame.stroke(
            &outer_disc,
            Stroke::default()
                .with_color(MayaColors::SOLAR_GOLD)
                .with_width(2.5),
        );

        // Inner glyph core: Ahau (The Sun God) eye motif
        let inner_disc = Path::circle(center, sun_radius * 0.65);
        frame.stroke(
            &inner_disc,
            Stroke::default()
                .with_color(MayaColors::JADE_GLOW)
                .with_width(1.5),
        );

        // 4 Cardinal solar rays (N, S, E, W - The 4 Bacabs / World Bearers)
        for dir in 0..4 {
            let angle = t * 0.08 + dir as f32 * (PI * 0.5);
            let ray_start = Point::new(
                center.x + (sun_radius * 0.65) * angle.cos(),
                center.y + (sun_radius * 0.65) * angle.sin(),
            );
            let ray_end = Point::new(
                center.x + (sun_radius * 1.35) * angle.cos(),
                center.y + (sun_radius * 1.35) * angle.sin(),
            );
            let ray = Path::line(ray_start, ray_end);
            frame.stroke(
                &ray,
                Stroke::default()
                    .with_color(MayaColors::SOLAR_AMBER)
                    .with_width(2.0),
            );
        }
    }
}
