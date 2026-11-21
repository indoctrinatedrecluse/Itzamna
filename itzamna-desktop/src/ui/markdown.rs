//! Mayan Markdown Renderer (El Glifo de Texto)
//! Converts Markdown text into rich, styled Iced widgets with Mesoamerican aesthetic flourishes.

use iced::widget::{column, container, Space};
use iced::{Element, Length};

use crate::theme::MayaColors;

pub struct MayanMarkdown;

impl MayanMarkdown {
    /// Parse any markdown string into an Iced Element
    pub fn render<'a, Message: 'a + From<crate::ui::CenterLoomMessage>>(
        content: &str,
    ) -> Element<'a, Message> {
        use iced::widget::text::Span;
        use pulldown_cmark::{Event, Options, Parser, Tag, TagEnd};
        #[derive(Clone, Copy)]
        enum Block {
            Paragraph,
            Heading(usize),
            Code,
            Quote,
            List,
            Table,
        }
        let parser = Parser::new_ext(
            content,
            Options::ENABLE_TABLES
                | Options::ENABLE_TASKLISTS
                | Options::ENABLE_STRIKETHROUGH
                | Options::ENABLE_FOOTNOTES,
        );
        let mut root = column![].spacing(6).width(Length::Fill);
        let mut spans: Vec<Span<'static, String>> = Vec::new();
        let mut block = Block::Paragraph;
        let mut current_link: Option<String> = None;
        let mut strong = false;
        let mut emphasis = false;
        let mut strike = false;
        let mut list_counters: Vec<Option<u64>> = Vec::new();
        fn flush<'a, Message: 'a + From<crate::ui::CenterLoomMessage>>(
            root: iced::widget::Column<'a, Message>,
            spans: &mut Vec<Span<'static, String>>,
            block: Block,
        ) -> iced::widget::Column<'a, Message> {
            if spans.is_empty() {
                return root;
            }
            let size = match block {
                Block::Heading(0) => 15,
                Block::Heading(1) => 13,
                Block::Heading(2) => 12,
                Block::Heading(_) => 11,
                Block::Code => 10,
                _ => 10,
            };
            let spans = std::mem::take(spans);
            root.push(
                iced::widget::rich_text(spans)
                    .size(size)
                    .width(Length::Fill)
                    .on_link_click(|url: String| crate::ui::CenterLoomMessage::OpenUrl(url).into()),
            )
        }
        for event in parser {
            match event {
                Event::Start(Tag::Heading { level, .. }) => {
                    root = flush(root, &mut spans, block);
                    block = Block::Heading(match level {
                        pulldown_cmark::HeadingLevel::H1 => 0,
                        pulldown_cmark::HeadingLevel::H2 => 1,
                        pulldown_cmark::HeadingLevel::H3 => 2,
                        _ => 3,
                    });
                }
                Event::End(TagEnd::Heading(_))
                | Event::End(TagEnd::Paragraph)
                | Event::End(TagEnd::CodeBlock) => {
                    root = flush(root, &mut spans, block);
                    block = Block::Paragraph;
                }
                Event::Start(Tag::Paragraph) => block = Block::Paragraph,
                Event::Start(Tag::CodeBlock(_)) => block = Block::Code,
                Event::Start(Tag::BlockQuote(_)) => {
                    block = Block::Quote;
                    spans.push(Span::new("❯ ".to_string()).color(MayaColors::SOLAR_GOLD));
                }
                Event::End(TagEnd::BlockQuote(_)) => {
                    root = flush(root, &mut spans, block);
                    block = Block::Paragraph;
                }
                Event::Start(Tag::List(start)) => {
                    list_counters.push(start);
                    block = Block::List;
                }
                Event::End(TagEnd::List(_)) => {
                    list_counters.pop();
                    root = flush(root, &mut spans, block);
                    block = Block::Paragraph;
                }
                Event::Start(Tag::Item) => {
                    let depth = list_counters.len().saturating_sub(1);
                    let marker = match list_counters.last_mut() {
                        Some(Some(next)) => {
                            let marker = format!("{}. ", next);
                            *next += 1;
                            marker
                        }
                        _ => "• ".to_string(),
                    };
                    spans.push(
                        Span::new(format!("{}{}", "  ".repeat(depth), marker))
                            .color(MayaColors::SOLAR_GOLD),
                    );
                }
                Event::End(TagEnd::Item) => spans.push(Span::new("\n".to_string())),
                Event::Start(Tag::Table(_)) => block = Block::Table,
                Event::End(TagEnd::Table) => {
                    root = flush(root, &mut spans, block);
                    block = Block::Paragraph;
                }
                Event::End(TagEnd::TableCell) => {
                    spans.push(Span::new("  |  ".to_string()).color(MayaColors::MAYA_BLUE))
                }
                Event::Start(Tag::Link { dest_url, .. }) => {
                    current_link = Some(dest_url.to_string())
                }
                Event::End(TagEnd::Link) => current_link = None,
                Event::Start(Tag::Strong) => strong = true,
                Event::End(TagEnd::Strong) => strong = false,
                Event::Start(Tag::Emphasis) => emphasis = true,
                Event::End(TagEnd::Emphasis) => emphasis = false,
                Event::Start(Tag::Strikethrough) => strike = true,
                Event::End(TagEnd::Strikethrough) => strike = false,
                Event::Text(value) | Event::Code(value) => {
                    let block_color = match block {
                        Block::Heading(0) => MayaColors::SOLAR_GOLD,
                        Block::Heading(1) => MayaColors::JADE_GLOW,
                        Block::Heading(2) => MayaColors::MAYA_BLUE,
                        Block::Heading(_) => MayaColors::SOLAR_AMBER,
                        Block::Code => MayaColors::JADE_GLOW,
                        _ => MayaColors::STARLIGHT_WHITE,
                    };
                    let mut span = Span::new(value.to_string()).color(if current_link.is_some() {
                        MayaColors::MAYA_BLUE
                    } else if strong {
                        MayaColors::SOLAR_GOLD
                    } else if emphasis {
                        MayaColors::JADE_GLOW
                    } else {
                        block_color
                    });
                    if let Some(url) = current_link.clone() {
                        span = span.link(url).underline(true);
                    }
                    if strike {
                        span.strikethrough = true;
                    }
                    if strong {
                        span = span.font(iced::Font {
                            weight: iced::font::Weight::Bold,
                            ..iced::Font::DEFAULT
                        });
                    }
                    if emphasis {
                        span = span.font(iced::Font {
                            style: iced::font::Style::Italic,
                            ..iced::Font::DEFAULT
                        });
                    }
                    spans.push(span);
                }
                Event::TaskListMarker(checked) => spans.push(
                    Span::new(if checked { "☑ " } else { "☐ " }.to_string())
                        .color(MayaColors::JADE_GLOW),
                ),
                Event::SoftBreak => spans.push(Span::new(" ".to_string())),
                Event::HardBreak => spans.push(Span::new("\n".to_string())),
                Event::Rule => {
                    root = flush(root, &mut spans, block);
                    root = root.push(Self::render_hr());
                }
                Event::Html(value) | Event::InlineHtml(value) => {
                    spans.push(Span::new(value.to_string()))
                }
                Event::FootnoteReference(label) => spans.push(Span::new(format!("[{}]", label))),
                _ => {}
            }
        }
        root = flush(root, &mut spans, block);
        root.into()
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
}
