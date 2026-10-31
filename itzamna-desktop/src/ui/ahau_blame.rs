//! The Ahau Lineage (El Linaje de Ahau — Interactive Git Blame & Heatmap)
//! Line-by-line author attribution with color-coded age gradients and commit inspector links.

use iced::widget::{button, column, container, row, scrollable, text, text_input, tooltip, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::git::BlameLineInfo;
use crate::theme::MayaColors;

#[derive(Debug, Clone)]
pub enum BlameMessage {
    FilePathChanged(String),
    ExecuteBlame,
    SelectCommitSha(String),
}

pub struct AhauBlameWorkbench {
    pub file_path_input: String,
    pub current_file: String,
    pub lines: Vec<BlameLineInfo>,
    pub selected_sha: Option<String>,
    pub error_message: Option<String>,
}

impl Default for AhauBlameWorkbench {
    fn default() -> Self {
        Self::new("Cargo.toml".to_string())
    }
}

impl AhauBlameWorkbench {
    pub fn new(initial_file: String) -> Self {
        Self {
            file_path_input: initial_file.clone(),
            current_file: initial_file,
            lines: Vec::new(),
            selected_sha: None,
            error_message: None,
        }
    }

    pub fn set_blame_data(&mut self, file: String, lines: Vec<BlameLineInfo>) {
        self.current_file = file.clone();
        self.file_path_input = file;
        self.lines = lines;
        self.error_message = None;
    }

    pub fn set_error(&mut self, err: String) {
        self.error_message = Some(err);
    }

    pub fn view<'a, Message: 'a + From<BlameMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let title_bar = row![
            text("👑 EL LINAJE DE AHAU (GIT BLAME)")
                .size(11)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(12),
            text(format!("{} Inscribed Lines", self.lines.len()))
                .size(10)
                .color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            text_input("File path (e.g. Cargo.toml)...", &self.file_path_input)
                .size(10)
                .padding(Padding::from([3, 7]))
                .width(220)
                .style(|_theme, _status| text_input::Style {
                    background: MayaColors::OBSIDIAN_SURFACE.into(),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 3.0.into(),
                    },
                    icon: MayaColors::PARCHMENT_MUTED,
                    placeholder: MayaColors::GLYPH_DIM,
                    value: MayaColors::STARLIGHT_WHITE,
                    selection: MayaColors::JADE_SHADOW,
                })
                .on_input(|val| BlameMessage::FilePathChanged(val).into()),
            Space::new().width(6),
            tooltip(
                button(text("🔍 Blame File").size(10).color(MayaColors::OBSIDIAN_VOID))
                    .padding(Padding::from([3, 10]))
                    .style(|_theme, _status| button::Style {
                        background: Some(MayaColors::SOLAR_GOLD.into()),
                        border: Border {
                            color: MayaColors::SOLAR_AMBER,
                            width: 1.0,
                            radius: 3.0.into(),
                        },
                        ..Default::default()
                    })
                    .on_press(BlameMessage::ExecuteBlame.into()),
                text("Run line-by-line author attribution on file (git blame <path>)").size(11),
                tooltip::Position::Bottom,
            ),
        ]
        .padding(Padding::from([6, 12]))
        .align_y(Alignment::Center);

        if let Some(ref err) = self.error_message {
            let err_banner = container(
                row![
                    text("⚠️ ").size(12),
                    text(format!("Blame failed: {}", err)).size(11).color(MayaColors::CINNABAR_BRIGHT),
                ]
                .align_y(Alignment::Center)
            )
            .padding(Padding::from([6, 12]))
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.35, ..MayaColors::CINNABAR_DARK }.into()),
                ..Default::default()
            });

            return container(column![title_bar, err_banner])
                .width(Length::Fill)
                .height(Length::Fill)
                .into();
        }

        let mut lines_column = column![].spacing(1);

        for line in &self.lines {
            lines_column = lines_column.push(self.render_blame_row(line));
        }

        let scroll_body = scrollable(lines_column.padding(Padding::from([6, 10])))
            .height(Length::Fill);

        container(column![title_bar, scroll_body])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.65, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border {
                    color: MayaColors::OBSIDIAN_BORDER,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_blame_row<'a, Message: 'a + From<BlameMessage> + Clone>(
        &'a self,
        line: &'a BlameLineInfo,
    ) -> Element<'a, Message> {
        // Age Heat color gradient:
        // Recent lines (<7 days) glow radiant Jade
        // Medium age lines glow Solar Amber
        // Ancient lines fade to Obsidian Slate
        let heat_color = if line.age_heat > 0.8 {
            MayaColors::JADE_GLOW
        } else if line.age_heat > 0.4 {
            MayaColors::SOLAR_AMBER
        } else {
            MayaColors::PARCHMENT_MUTED
        };

        let sha_btn = tooltip(
            button(text(&line.short_sha).size(9).color(heat_color))
                .padding(Padding::from([1, 4]))
                .style(|_theme, _status| button::Style {
                    background: Some(MayaColors::OBSIDIAN_BASE.into()),
                    border: Border {
                        color: MayaColors::OBSIDIAN_BORDER,
                        width: 1.0,
                        radius: 2.0.into(),
                    },
                    ..Default::default()
                })
                .on_press(BlameMessage::SelectCommitSha(line.commit_sha.clone()).into()),
            text(format!("Inspect commit [{}] by {} ({})", line.short_sha, line.author_name, line.relative_time)).size(11),
            tooltip::Position::Right,
        );

        let gutter = row![
            text(format!("{:>4}", line.line_no)).size(9).color(MayaColors::GLYPH_DIM),
            Space::new().width(6),
            sha_btn,
            Space::new().width(6),
            text(format!("{:<14}", line.author_name)).size(9).color(MayaColors::STARLIGHT_WHITE),
            Space::new().width(6),
            text(format!("{:>7}", line.relative_time)).size(9).color(heat_color),
            Space::new().width(8),
            // Heat bar
            container(Space::new())
                .width(2)
                .height(14)
                .style(move |_theme| container::Style {
                    background: Some(heat_color.into()),
                    ..Default::default()
                }),
        ]
        .align_y(Alignment::Center);

        let code_text = text(&line.code_content)
            .size(10)
            .color(MayaColors::STARLIGHT_WHITE);

        let full_row = row![
            gutter,
            Space::new().width(10),
            code_text,
        ]
        .align_y(Alignment::Center)
        .padding(Padding::from([1, 4]));

        container(full_row)
            .width(Length::Fill)
            .style(|_theme| container::Style {
                background: Some(Color { a: 0.15, ..MayaColors::OBSIDIAN_SURFACE }.into()),
                ..Default::default()
            })
            .into()
    }
}
