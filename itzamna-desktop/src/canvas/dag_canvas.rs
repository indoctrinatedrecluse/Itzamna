//! The Yaxche Commit DAG Canvas
//! GPU-accelerated interactive commit graph rendering topological branch lanes,
//! Kukulkan feathered-serpent bezier curves, and glowing solar seals.

use iced::mouse;
use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke, Text};
use iced::{Color, Element, Length, Point, Rectangle, Renderer, Size, Theme};

use crate::git::GitCommitNode;
use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum DagMessage {
    CommitSelected(String),
    Scrolled(f32),
}

#[derive(Debug, Clone)]
pub struct DagCanvas {
    pub commits: Vec<GitCommitNode>,
    pub selected_sha: Option<String>,
    pub hovered_sha: Option<String>,
    pub pan_y: f32,
    pub zoom: f32,
    pub lane_width: f32,
    pub row_height: f32,
}

impl Default for DagCanvas {
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

fn get_lane_color(lane: usize) -> Color {
    match lane % 6 {
        0 => MayaColors::JADE_GLOW,
        1 => MayaColors::MAYA_BLUE,
        2 => MayaColors::SOLAR_GOLD,
        3 => MayaColors::CINNABAR_BRIGHT,
        4 => Color::from_rgb(0.18, 0.85, 0.72), // Quetzal Teal
        5 => Color::from_rgb(0.72, 0.42, 0.92), // Cosmic Amethyst
        _ => MayaColors::JADE_PRIME,
    }
}

impl DagCanvas {
    pub fn new(commits: Vec<GitCommitNode>) -> Self {
        let selected_sha = commits.first().map(|c| c.short_sha.clone());
        Self {
            commits,
            selected_sha,
            hovered_sha: None,
            pan_y: 20.0,
            zoom: 1.0,
            lane_width: 32.0,
            row_height: 52.0,
        }
    }

    pub fn set_commits(&mut self, commits: Vec<GitCommitNode>) {
        if self.selected_sha.is_none() {
            self.selected_sha = commits.first().map(|c| c.short_sha.clone());
        }
        self.commits = commits;
    }

    pub fn zoom_in(&mut self) {
        self.zoom = (self.zoom + 0.15).min(2.0);
    }

    pub fn zoom_out(&mut self) {
        self.zoom = (self.zoom - 0.15).max(0.65);
    }

    pub fn reset_view(&mut self) {
        self.zoom = 1.0;
        self.pan_y = 20.0;
    }

    pub fn pan(&mut self, dy: f32) {
        let max_pan = 40.0;
        let min_pan = -((self.commits.len() as f32 * self.row_height * self.zoom) - 200.0).max(0.0);
        self.pan_y = (self.pan_y + dy).clamp(min_pan, max_pan);
    }

    pub fn view<'a, Message: 'a + From<DagMessage>>(
        &'a self,
        width: impl Into<Length>,
        height: impl Into<Length>,
    ) -> Element<'a, Message> {
        Canvas::new(self).width(width).height(height).into()
    }
}

impl<Message: From<DagMessage>> canvas::Program<Message> for DagCanvas {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        event: &iced::Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<iced::widget::Action<Message>> {
        match event {
            iced::Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                if cursor.position_in(bounds).is_some() {
                    let dy = match delta {
                        mouse::ScrollDelta::Lines { y, .. } => *y * 36.0,
                        mouse::ScrollDelta::Pixels { y, .. } => *y,
                    };
                    return Some(iced::widget::Action::publish(
                        DagMessage::Scrolled(dy).into(),
                    ));
                }
            }
            iced::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                if let Some(cursor_pos) = cursor.position_in(bounds) {
                    let start_x = 40.0;
                    let start_y = self.pan_y;

                    for (idx, commit) in self.commits.iter().enumerate() {
                        let cy = start_y + (idx as f32 * self.row_height * self.zoom);
                        let row_rect = Rectangle::new(
                            Point::new(start_x, cy - 18.0 * self.zoom),
                            Size::new(bounds.width - start_x - 20.0, self.row_height * self.zoom),
                        );

                        if row_rect.contains(cursor_pos) {
                            return Some(iced::widget::Action::publish(
                                DagMessage::CommitSelected(commit.short_sha.clone()).into(),
                            ));
                        }
                    }
                }
            }
            _ => {}
        }

        None
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        if self.commits.is_empty() {
            frame.fill_text(Text {
                content: "No commits awakened in this repository yet.".into(),
                position: Point::new(40.0, 40.0),
                color: MayaColors::PARCHMENT_MUTED,
                size: 13.0.into(),
                ..Default::default()
            });
            return vec![frame.into_geometry()];
        }

        let start_x = 40.0;
        let start_y = self.pan_y;
        let cursor_pos = cursor.position_in(bounds);

        // Calculate maximum lane to provide adequate spacing
        let max_lane = self
            .commits
            .iter()
            .map(|c| c.branch_lane)
            .max()
            .unwrap_or(0);
        let total_lanes = (max_lane + 1).max(3);

        // 1. Draw Lane Guidelines (The Sacred Weft)
        for lane in 0..total_lanes {
            let lx = start_x + (lane as f32 * self.lane_width * self.zoom);
            let guide = Path::line(Point::new(lx, 0.0), Point::new(lx, bounds.height));
            let lane_c = get_lane_color(lane);
            frame.stroke(
                &guide,
                Stroke::default()
                    .with_color(Color { a: 0.08, ..lane_c })
                    .with_width(1.0),
            );
        }

        // Fast lookup table: sha and short_sha -> commit index
        let mut sha_to_idx: std::collections::HashMap<&str, usize> =
            std::collections::HashMap::new();
        for (i, c) in self.commits.iter().enumerate() {
            sha_to_idx.insert(c.sha.as_str(), i);
            sha_to_idx.insert(c.short_sha.as_str(), i);
        }

        // 2. Draw Kukulkan Bezier Connection Ribbons (Parent -> Child for all parents)
        for (idx, commit) in self.commits.iter().enumerate() {
            let child_y = start_y + (idx as f32 * self.row_height * self.zoom);
            let child_x = start_x + (commit.branch_lane as f32 * self.lane_width * self.zoom);
            let child_pt = Point::new(child_x, child_y);
            let child_color = get_lane_color(commit.branch_lane);

            for parent_sha in &commit.parents {
                if let Some(&parent_idx) = sha_to_idx.get(parent_sha.as_str()) {
                    let parent_commit = &self.commits[parent_idx];
                    let parent_y = start_y + (parent_idx as f32 * self.row_height * self.zoom);
                    let parent_x =
                        start_x + (parent_commit.branch_lane as f32 * self.lane_width * self.zoom);
                    let parent_pt = Point::new(parent_x, parent_y);
                    let parent_color = get_lane_color(parent_commit.branch_lane);

                    let ribbon_color = if commit.branch_lane == parent_commit.branch_lane {
                        child_color
                    } else {
                        parent_color
                    };

                    if commit.branch_lane == parent_commit.branch_lane {
                        // Straight branch line
                        let line = Path::line(child_pt, parent_pt);
                        frame.stroke(
                            &line,
                            Stroke::default()
                                .with_color(Color {
                                    a: 0.70,
                                    ..ribbon_color
                                })
                                .with_width(2.5 * self.zoom),
                        );
                    } else {
                        // Sinuous Feathered Serpent (Kukulkan) Cubic Bezier Curve
                        let delta_y = parent_y - child_y;
                        let cp1 = Point::new(child_x, child_y + delta_y * 0.45);
                        let cp2 = Point::new(parent_x, parent_y - delta_y * 0.45);

                        let bezier = Path::new(|b| {
                            b.move_to(child_pt);
                            b.bezier_curve_to(cp1, cp2, parent_pt);
                        });

                        // Glowing outer aura
                        frame.stroke(
                            &bezier,
                            Stroke::default()
                                .with_color(Color {
                                    a: 0.22,
                                    ..MayaColors::SOLAR_GOLD
                                })
                                .with_width(4.5 * self.zoom),
                        );
                        // Core serpent ribbon
                        frame.stroke(
                            &bezier,
                            Stroke::default()
                                .with_color(Color {
                                    a: 0.85,
                                    ..ribbon_color
                                })
                                .with_width(2.5 * self.zoom),
                        );
                    }
                } else {
                    // Parent is beyond the loaded window: draw a clean downward stub
                    let stub_end = Point::new(child_x, child_y + 24.0 * self.zoom);
                    let line = Path::line(child_pt, stub_end);
                    frame.stroke(
                        &line,
                        Stroke::default()
                            .with_color(Color {
                                a: 0.35,
                                ..child_color
                            })
                            .with_width(2.0 * self.zoom),
                    );
                }
            }
        }

        // 3. Draw Commit Nodes & Descriptive Cartouches
        for (idx, commit) in self.commits.iter().enumerate() {
            let cy = start_y + (idx as f32 * self.row_height * self.zoom);
            let cx = start_x + (commit.branch_lane as f32 * self.lane_width * self.zoom);
            let center = Point::new(cx, cy);

            let is_selected = self.selected_sha.as_deref() == Some(&commit.short_sha);
            let is_hovered = cursor_pos
                .map(|p| {
                    let dx = p.x - cx;
                    let dy = p.y - cy;
                    dx * dx + dy * dy < (18.0 * self.zoom) * (18.0 * self.zoom)
                })
                .unwrap_or(false);

            // A. Draw Solar Kin Seal (The Commit Node)
            let radius = if commit.is_head {
                11.0 * self.zoom
            } else {
                8.0 * self.zoom
            };

            let lane_c = get_lane_color(commit.branch_lane);
            let (core_color, aura_color) = if commit.is_head {
                (MayaColors::SOLAR_GOLD, MayaColors::SOLAR_AMBER)
            } else if is_selected || is_hovered {
                (MayaColors::JADE_GLOW, lane_c)
            } else {
                (lane_c, MayaColors::OBSIDIAN_BORDER)
            };

            // Halo glow on hover / select / HEAD
            if commit.is_head || is_selected || is_hovered {
                let halo = Path::circle(center, radius * 2.2);
                frame.fill(
                    &halo,
                    Color {
                        a: if is_selected { 0.25 } else { 0.12 },
                        ..aura_color
                    },
                );
            }

            // Outer stone ring
            let outer_disc = Path::circle(center, radius);
            frame.fill(
                &outer_disc,
                Color {
                    a: 0.95,
                    ..MayaColors::OBSIDIAN_BASE
                },
            );
            frame.stroke(
                &outer_disc,
                Stroke::default()
                    .with_color(core_color)
                    .with_width(if commit.is_head { 2.5 } else { 1.5 }),
            );

            // Inner solar core dot
            let inner_dot = Path::circle(center, radius * 0.45);
            frame.fill(&inner_dot, core_color);

            // B. Draw Inscription Text & Metadata (To the right of the graph lanes)
            let text_x = start_x + (total_lanes as f32 * self.lane_width * self.zoom) + 16.0;

            // Commit SHA badge
            let sha_text = format!("kin:{}", commit.short_sha);
            frame.fill_text(Text {
                content: sha_text,
                position: Point::new(text_x, cy - 8.0 * self.zoom),
                color: if is_selected {
                    MayaColors::SOLAR_GOLD
                } else {
                    MayaColors::SOLAR_AMBER
                },
                size: (11.0 * self.zoom).into(),
                ..Default::default()
            });

            // Dynamic badge placement: branch heads and tags
            let mut badge_cursor_x = text_x + 76.0 * self.zoom;

            // Render Branch Badges
            for branch in &commit.branches {
                let is_head_branch = branch.starts_with("HEAD");
                let is_remote = branch.contains('/');
                let (badge_bg, _badge_border, badge_text_color) = if is_head_branch {
                    (
                        MayaColors::SOLAR_GOLD,
                        MayaColors::SOLAR_AMBER,
                        MayaColors::OBSIDIAN_VOID,
                    )
                } else if is_remote {
                    (
                        MayaColors::MAYA_BLUE,
                        MayaColors::MAYA_DEEP,
                        MayaColors::OBSIDIAN_VOID,
                    )
                } else {
                    (
                        MayaColors::JADE_PRIME,
                        MayaColors::JADE_GLOW,
                        MayaColors::OBSIDIAN_VOID,
                    )
                };

                let badge_width = (branch.len() as f32 * 6.2 + 10.0) * self.zoom;
                let badge_rect = Rectangle::new(
                    Point::new(badge_cursor_x, cy - 11.0 * self.zoom),
                    Size::new(badge_width, 14.0 * self.zoom),
                );

                frame.fill_rectangle(badge_rect.position(), badge_rect.size(), badge_bg);

                frame.fill_text(Text {
                    content: branch.clone(),
                    position: Point::new(badge_cursor_x + 5.0 * self.zoom, cy - 9.0 * self.zoom),
                    color: badge_text_color,
                    size: (8.5 * self.zoom).into(),
                    ..Default::default()
                });

                badge_cursor_x += badge_width + 6.0 * self.zoom;
            }

            // Render Tag Badges
            for tag in &commit.tags {
                let tag_display = format!("🏷️ {}", tag);
                let badge_width = (tag_display.len() as f32 * 5.8 + 10.0) * self.zoom;
                let badge_rect = Rectangle::new(
                    Point::new(badge_cursor_x, cy - 11.0 * self.zoom),
                    Size::new(badge_width, 14.0 * self.zoom),
                );

                frame.fill_rectangle(
                    badge_rect.position(),
                    badge_rect.size(),
                    MayaColors::SOLAR_AMBER,
                );

                frame.fill_text(Text {
                    content: tag_display,
                    position: Point::new(badge_cursor_x + 5.0 * self.zoom, cy - 9.0 * self.zoom),
                    color: MayaColors::OBSIDIAN_VOID,
                    size: (8.5 * self.zoom).into(),
                    ..Default::default()
                });

                badge_cursor_x += badge_width + 6.0 * self.zoom;
            }

            // Commit Summary
            let summary_x = badge_cursor_x + 4.0;
            let summary_color = if is_selected {
                MayaColors::STARLIGHT_WHITE
            } else if is_hovered {
                MayaColors::JADE_GLOW
            } else {
                MayaColors::PARCHMENT_MUTED
            };

            frame.fill_text(Text {
                content: commit.summary.clone(),
                position: Point::new(summary_x, cy - 8.0 * self.zoom),
                color: summary_color,
                size: (12.0 * self.zoom).into(),
                ..Default::default()
            });

            // Author & Relative time
            let author_meta = format!("{} · {}", commit.author_name, commit.relative_time);
            frame.fill_text(Text {
                content: author_meta,
                position: Point::new(text_x, cy + 9.0 * self.zoom),
                color: MayaColors::GLYPH_DIM,
                size: (9.5 * self.zoom).into(),
                ..Default::default()
            });
        }

        vec![frame.into_geometry()]
    }
}
