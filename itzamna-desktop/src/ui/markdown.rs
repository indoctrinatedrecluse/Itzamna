//! Mayan Markdown Renderer (El Glifo de Texto)
//! Converts Markdown text into rich, styled Iced widgets with Mesoamerican aesthetic flourishes.

use iced::widget::{column, container, row, text, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;

pub struct MayanMarkdown;

impl MayanMarkdown {
    /// Parse any markdown string into an Iced Element
    pub fn render<'a, Message: 'a>(content: &str) -> Element<'a, Message> {
        let mut root_col = column![].spacing(6).width(Length::Fill);

        let mut in_code_block = false;
        let mut code_block_lines: Vec<String> = Vec::new();
        let mut code_lang = String::new();

        for line in content.lines() {
            let trimmed = line.trim();

            // 1. Fenced Code Block delimiter
            if trimmed.starts_with("```") {
                if in_code_block {
                    // Close code block
                    let code_text = code_block_lines.join("\n");
                    root_col = root_col.push(Self::render_code_block(&code_text, &code_lang));
                    code_block_lines.clear();
                    code_lang.clear();
                    in_code_block = false;
                } else {
                    // Open code block
                    in_code_block = true;
                    code_lang = trimmed.trim_start_matches('`').trim().to_string();
                }
                continue;
            }

            if in_code_block {
                code_block_lines.push(line.to_string());
                continue;
            }

            // Empty line
            if trimmed.is_empty() {
                root_col = root_col.push(Space::new().height(4));
                continue;
            }

            // 2. Headings
            if let Some(h1) = trimmed.strip_prefix("# ") {
                root_col = root_col.push(Self::render_h1(h1));
            } else if let Some(h2) = trimmed.strip_prefix("## ") {
                root_col = root_col.push(Self::render_h2(h2));
            } else if let Some(h3) = trimmed.strip_prefix("### ") {
                root_col = root_col.push(Self::render_h3(h3));
            } else if let Some(h4) = trimmed.strip_prefix("#### ") {
                root_col = root_col.push(Self::render_h4(h4));
            }
            // 3. Blockquotes
            else if let Some(quote) = trimmed.strip_prefix("> ") {
                root_col = root_col.push(Self::render_quote(quote));
            }
            // 4. Horizontal Rule
            else if trimmed == "---" || trimmed == "***" || trimmed == "___" {
                root_col = root_col.push(Self::render_hr());
            }
            // 5. Unordered List Items
            else if let Some(item) = trimmed.strip_prefix("* ").or_else(|| trimmed.strip_prefix("- ")) {
                root_col = root_col.push(Self::render_list_item(item, "▸"));
            }
            // 6. Ordered List Items
            else if trimmed.chars().next().map_or(false, |c| c.is_ascii_digit()) && trimmed.contains(". ") {
                let parts: Vec<&str> = trimmed.splitn(2, ". ").collect();
                if parts.len() == 2 {
                    root_col = root_col.push(Self::render_list_item(parts[1], &format!("{}.", parts[0])));
                } else {
                    root_col = root_col.push(Self::render_paragraph(trimmed));
                }
            }
            // 7. Standard Paragraph
            else {
                root_col = root_col.push(Self::render_paragraph(trimmed));
            }
        }

        // Flush unclosed code block if file ends inside one
        if in_code_block && !code_block_lines.is_empty() {
            let code_text = code_block_lines.join("\n");
            root_col = root_col.push(Self::render_code_block(&code_text, &code_lang));
        }

        root_col.into()
    }

    fn render_h1<'a, Message: 'a>(h1: &str) -> Element<'a, Message> {
        column![
            text(h1.to_string())
                .size(15)
                .color(MayaColors::SOLAR_GOLD)
                .wrapping(text::Wrapping::Word),
            container(Space::new())
                .height(1)
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(MayaColors::SOLAR_GOLD.into()),
                    ..Default::default()
                }),
        ]
        .spacing(3)
        .into()
    }

    fn render_h2<'a, Message: 'a>(h2: &str) -> Element<'a, Message> {
        column![
            text(h2.to_string())
                .size(13)
                .color(MayaColors::JADE_GLOW)
                .wrapping(text::Wrapping::Word),
            container(Space::new())
                .height(1)
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Color { a: 0.3, ..MayaColors::JADE_GLOW }.into()),
                    ..Default::default()
                }),
        ]
        .spacing(2)
        .into()
    }

    fn render_h3<'a, Message: 'a>(h3: &str) -> Element<'a, Message> {
        text(h3.to_string())
            .size(12)
            .color(MayaColors::MAYA_BLUE)
            .wrapping(text::Wrapping::Word)
            .into()
    }

    fn render_h4<'a, Message: 'a>(h4: &str) -> Element<'a, Message> {
        text(h4.to_string())
            .size(11)
            .color(MayaColors::SOLAR_AMBER)
            .wrapping(text::Wrapping::Word)
            .into()
    }

    fn render_quote<'a, Message: 'a>(quote: &str) -> Element<'a, Message> {
        container(
            row![
                container(Space::new())
                    .width(3)
                    .height(Length::Fill)
                    .style(|_| container::Style {
                        background: Some(MayaColors::SOLAR_GOLD.into()),
                        ..Default::default()
                    }),
                Space::new().width(8),
                text(quote.to_string())
                    .size(10)
                    .color(MayaColors::STARLIGHT_WHITE)
                    .wrapping(text::Wrapping::Word),
            ]
            .align_y(Alignment::Center)
        )
        .padding(Padding::from([4, 6]))
        .style(|_| container::Style {
            background: Some(Color { a: 0.25, ..MayaColors::OBSIDIAN_SURFACE }.into()),
            border: Border { radius: 2.0.into(), ..Default::default() },
            ..Default::default()
        })
        .into()
    }

    fn render_hr<'a, Message: 'a>() -> Element<'a, Message> {
        container(Space::new())
            .height(1)
            .width(Length::Fill)
            .style(|_| container::Style {
                background: Some(MayaColors::OBSIDIAN_BORDER.into()),
                ..Default::default()
            })
            .into()
    }

    fn render_list_item<'a, Message: 'a>(item: &str, bullet: &str) -> Element<'a, Message> {
        row![
            text(bullet.to_string())
                .size(10)
                .color(MayaColors::SOLAR_GOLD),
            Space::new().width(6),
            text(item.to_string())
                .size(10)
                .color(MayaColors::STARLIGHT_WHITE)
                .wrapping(text::Wrapping::Word),
        ]
        .align_y(Alignment::Start)
        .into()
    }

    fn render_code_block<'a, Message: 'a>(code: &str, lang: &str) -> Element<'a, Message> {
        let header = if !lang.is_empty() {
            row![
                text(lang.to_string()).size(8).color(MayaColors::GLYPH_DIM),
                Space::new().width(Length::Fill),
            ]
        } else {
            row![]
        };

        container(
            column![
                header,
                text(code.to_string())
                    .size(10)
                    .color(MayaColors::JADE_GLOW),
            ]
            .spacing(2)
        )
        .padding(Padding::from([6, 8]))
        .width(Length::Fill)
        .style(|_| container::Style {
            background: Some(Color { a: 0.85, ..MayaColors::OBSIDIAN_VOID }.into()),
            border: Border {
                color: MayaColors::OBSIDIAN_BORDER,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        })
        .into()
    }

    fn render_paragraph<'a, Message: 'a>(paragraph: &str) -> Element<'a, Message> {
        // If inline code tags `...` exist, highlight them
        if paragraph.contains('`') {
            let mut row_content = row![].align_y(Alignment::Center).spacing(2);
            let parts: Vec<&str> = paragraph.split('`').collect();
            for (idx, part) in parts.iter().enumerate() {
                if idx % 2 == 1 {
                    // Inline code
                    row_content = row_content.push(
                        container(text(part.to_string()).size(9).color(MayaColors::JADE_GLOW))
                            .padding(Padding::from([1, 4]))
                            .style(|_| container::Style {
                                background: Some(MayaColors::OBSIDIAN_VOID.into()),
                                border: Border { radius: 2.0.into(), ..Default::default() },
                                ..Default::default()
                            })
                    );
                } else if !part.is_empty() {
                    row_content = row_content.push(
                        text(part.to_string())
                            .size(10)
                            .color(MayaColors::STARLIGHT_WHITE)
                            .wrapping(text::Wrapping::Word)
                    );
                }
            }
            row_content.into()
        } else {
            text(paragraph.to_string())
                .size(10)
                .color(MayaColors::STARLIGHT_WHITE)
                .wrapping(text::Wrapping::Word)
                .into()
        }
    }
}
