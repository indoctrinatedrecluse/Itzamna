//! The Carved Stone Diff Workbench (El Banco de Trabajo de Diferencias)
//! Intra-line syntax-highlighted diffs with surgical hunk-level staging.

use iced::widget::{button, column, container, row, scrollable, text, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::git::{DiffHunk, DiffLine, DiffLineTag, FileDiff};
use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum DiffMessage {
    StageHunk(usize),
    UnstageHunk(usize),
    DiscardHunk(usize),
    StageEntireFile,
    CloseDiff,
}

pub struct DiffWorkbench {
    pub file_diff: Option<FileDiff>,
    pub file_path: String,
}

impl Default for DiffWorkbench {
    fn default() -> Self {
        Self::new("src/app.rs".to_string())
    }
}

impl DiffWorkbench {
    pub fn new(file_path: String) -> Self {
        let sample_old = r#"fn main() {
    println!("Hello, World!");
    let x = 10;
    println!("Value: {}", x);
}"#;
        let sample_new = r#"fn main() {
    println!("🔱 Awakening Itzamna Studio...");
    let x = 42; // Sacred Kin Number
    let y = 108; // Tzolk'in Harmony
    println!("Solar Coordinates: {}, {}", x, y);
}"#;

        let diff = crate::git::DiffEngine::compute_diff(sample_old, sample_new, &file_path);

        Self {
            file_diff: Some(diff),
            file_path,
        }
    }

    pub fn set_file(&mut self, path: String, old_text: &str, new_text: &str) {
        self.file_path = path.clone();
        self.file_diff = Some(crate::git::DiffEngine::compute_diff(old_text, new_text, &path));
    }

    pub fn view<'a, Message: 'a + From<DiffMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let header = row![
            text("🔪 ETZ'NAB' STONE DIFF:").size(11).color(MayaColors::SOLAR_GOLD),
            text(format!(" {}", self.file_path)).size(11).color(MayaColors::STARLIGHT_WHITE),
            Space::new().width(Length::Fill),
            tooltip(
                button(text("Stage File +").size(10).color(MayaColors::JADE_GLOW))
                    .padding(Padding::from([2, 8]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border {
                            color: MayaColors::JADE_SHADOW,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(DiffMessage::StageEntireFile.into()),
                text("Stage all modifications in this file (git add <file>)").size(11),
                tooltip::Position::Bottom,
            ),
            Space::new().width(6),
            tooltip(
                button(text("✕ Close").size(10).color(MayaColors::PARCHMENT_MUTED))
                    .padding(Padding::from([2, 8]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border {
                            color: MayaColors::OBSIDIAN_BORDER,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(DiffMessage::CloseDiff.into()),
                text("Close diff view and return to commit DAG").size(11),
                tooltip::Position::Bottom,
            ),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        let Some(diff) = &self.file_diff else {
            return container(column![header, text("No diff offerings present.").size(12)])
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        };

        let mut hunk_list = column![].spacing(10);

        for (hunk_idx, hunk) in diff.hunks.iter().enumerate() {
            let hunk_card = self.render_hunk(hunk_idx, hunk);
            hunk_list = hunk_list.push(hunk_card);
        }

        let body = scrollable(hunk_list.padding(Padding::from([8, 12])))
            .height(Length::Fill);

        container(column![header, body])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.60, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_hunk<'a, Message: 'a + From<DiffMessage> + Clone>(
        &'a self,
        hunk_idx: usize,
        hunk: &'a DiffHunk,
    ) -> Element<'a, Message> {
        let hunk_header = row![
            text(&hunk.header).size(10).color(MayaColors::SOLAR_AMBER),
            Space::new().width(Length::Fill),
            tooltip(
                button(text("Stage Hunk +").size(10).color(MayaColors::JADE_GLOW))
                    .padding(Padding::from([1, 6]))
                    .style(|_theme, _status| button::Style {
                        background: Some(Color { a: 0.4, ..MayaColors::JADE_SHADOW }.into()),
                        border: Border {
                            color: MayaColors::JADE_GLOW,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(DiffMessage::StageHunk(hunk_idx).into()),
                text("Surgically stage this modification block into the index (git add -p)").size(11),
                tooltip::Position::Left,
            ),
        ]
        .padding(Padding::from([4, 8]))
        .align_y(Alignment::Center);

        let mut lines_col = column![].spacing(1);

        for line in &hunk.lines {
            let line_row = self.render_line(line);
            lines_col = lines_col.push(line_row);
        }

        container(column![hunk_header, lines_col])
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.85, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_line<'a, Message: 'a>(&'a self, line: &'a DiffLine) -> Element<'a, Message> {
        let (sign, text_color, bg_color) = match line.tag {
            DiffLineTag::Context => (" ", MayaColors::STARLIGHT_WHITE, Color::TRANSPARENT),
            DiffLineTag::Addition => ("+", MayaColors::JADE_GLOW, Color::from_rgba(0.0, 0.95, 0.60, 0.12)),
            DiffLineTag::Deletion => ("-", MayaColors::CINNABAR_BRIGHT, Color::from_rgba(1.0, 0.28, 0.35, 0.14)),
        };

        let old_num_str = line.old_lineno.map(|n| n.to_string()).unwrap_or_default();
        let new_num_str = line.new_lineno.map(|n| n.to_string()).unwrap_or_default();

        let gutter = row![
            text(format!("{:>3}", old_num_str)).size(9).color(MayaColors::GLYPH_DIM),
            text(format!("{:>3}", new_num_str)).size(9).color(MayaColors::GLYPH_DIM),
            text(sign).size(10).color(text_color),
        ]
        .spacing(4)
        .align_y(Alignment::Center);

        let code_line = text(&line.content)
            .size(11)
            .color(text_color);

        let content = row![
            gutter,
            Space::new().width(8),
            code_line,
        ]
        .padding(Padding::from([1, 6]))
        .align_y(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .style(move |_theme| container::Style {
                background: Some(bg_color.into()),
                ..Default::default()
            })
            .into()
    }
}
