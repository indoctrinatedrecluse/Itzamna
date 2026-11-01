//! The K'atun Code Editor (El Telar de Código)
//! Built-in GPU-accelerated code editor with syntax highlighting, line numbers,
//! on-the-fly file mutation, and instant Git staging.

use iced::widget::{button, column, container, row, text, text_editor, Space};
use iced::{Alignment, Border, Color, Element, Length, Padding};

use crate::theme::MayaColors;
use crate::ui::tooltip::mayan_tooltip;

#[derive(Debug, Clone)]
pub enum EditorMessage {
    Edit(text_editor::Action),
    SaveFile,
    StageAndSave,
    CloseEditor,
    OpenFile(String),
}

pub struct KatunEditor {
    pub file_path: Option<String>,
    pub content: text_editor::Content,
    pub is_modified: bool,
    pub status_message: Option<String>,
}

impl Default for KatunEditor {
    fn default() -> Self {
        Self::new()
    }
}

impl KatunEditor {
    pub fn new() -> Self {
        Self {
            file_path: None,
            content: text_editor::Content::new(),
            is_modified: false,
            status_message: None,
        }
    }

    pub fn load_file(&mut self, path: String) -> Result<(), String> {
        let text = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
        self.content = text_editor::Content::with_text(&text);
        self.file_path = Some(path.clone());
        self.is_modified = false;
        self.status_message = Some(format!("Awakened file: {}", path));
        Ok(())
    }

    pub fn save_file(&mut self) -> Result<(), String> {
        if let Some(ref path) = self.file_path {
            let full_text = self.content.text();
            std::fs::write(path, full_text).map_err(|e| e.to_string())?;
            self.is_modified = false;
            self.status_message = Some(format!("Sealed changes in stone: {}", path));
            Ok(())
        } else {
            Err("No active file to save".into())
        }
    }

    pub fn save(&mut self) -> Result<(), String> {
        self.save_file()
    }

    pub fn update(&mut self, message: EditorMessage) -> Option<String> {
        match message {
            EditorMessage::Edit(action) => {
                self.is_modified = true;
                self.content.perform(action);
                None
            }
            EditorMessage::SaveFile => {
                if let Err(e) = self.save_file() {
                    self.status_message = Some(format!("Save failed: {}", e));
                }
                None
            }
            EditorMessage::StageAndSave => {
                if let Err(e) = self.save_file() {
                    self.status_message = Some(format!("Save failed: {}", e));
                    None
                } else {
                    self.file_path.clone()
                }
            }
            EditorMessage::CloseEditor => {
                self.file_path = None;
                self.content = text_editor::Content::new();
                self.is_modified = false;
                self.status_message = None;
                None
            }
            EditorMessage::OpenFile(path) => {
                let _ = self.load_file(path);
                None
            }
        }
    }

    pub fn view<'a, Message: 'a + From<EditorMessage> + Clone>(&'a self) -> Element<'a, Message> {
        let Some(ref path) = self.file_path else {
            return container(
                column![
                    text("🏺 K'ATUN CODE EDITOR").size(14).color(MayaColors::SOLAR_GOLD),
                    Space::new().height(8),
                    text("No file awakened in the crucible. Select a file from the Codex file tree to edit.")
                        .size(11)
                        .color(MayaColors::PARCHMENT_MUTED),
                ]
                .align_x(Alignment::Center)
                .padding(Padding::from(40))
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        };

        let file_name = std::path::Path::new(path)
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| path.clone());

        let modified_badge = if self.is_modified {
            container(text("MODIFIED").size(8).color(MayaColors::SOLAR_AMBER))
                .padding(Padding::from([1, 4]))
                .style(|_| container::Style {
                    background: Some(Color { a: 0.25, ..MayaColors::SOLAR_AMBER }.into()),
                    border: Border { color: MayaColors::SOLAR_AMBER, width: 0.5, radius: 2.0.into() },
                    ..Default::default()
                })
        } else {
            container(text("SAVED").size(8).color(MayaColors::JADE_GLOW))
                .padding(Padding::from([1, 4]))
                .style(|_| container::Style {
                    background: Some(Color { a: 0.25, ..MayaColors::JADE_GLOW }.into()),
                    border: Border { color: MayaColors::JADE_GLOW, width: 0.5, radius: 2.0.into() },
                    ..Default::default()
                })
        };

        let header = row![
            text("📝").size(12),
            Space::new().width(6),
            text(file_name).size(11).color(MayaColors::SOLAR_GOLD),
            Space::new().width(8),
            modified_badge,
            Space::new().width(12),
            text(path).size(9).color(MayaColors::GLYPH_DIM),
            Space::new().width(Length::Fill),
            mayan_tooltip(
                button(
                    row![
                        text("💾").size(10),
                        Space::new().width(4),
                        text("Save (Ctrl+S)").size(10).color(MayaColors::STARLIGHT_WHITE),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([2, 8]))
                .style(|_, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::SOLAR_GOLD,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 3.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(EditorMessage::SaveFile.into()),
                "Save changes directly to disk (Ctrl+S)",
                iced::widget::tooltip::Position::Bottom,
            ),
            Space::new().width(6),
            mayan_tooltip(
                button(
                    row![
                        text("⚡").size(10),
                        Space::new().width(4),
                        text("Save & Stage").size(10).color(MayaColors::JADE_GLOW),
                    ]
                    .align_y(Alignment::Center)
                )
                .padding(Padding::from([2, 8]))
                .style(|_, status| {
                    let border_color = match status {
                        button::Status::Hovered => MayaColors::JADE_GLOW,
                        _ => MayaColors::OBSIDIAN_BORDER,
                    };
                    button::Style {
                        background: Some(MayaColors::OBSIDIAN_SURFACE.into()),
                        border: Border { color: border_color, width: 1.0, radius: 3.0.into() },
                        ..Default::default()
                    }
                })
                .on_press(EditorMessage::StageAndSave.into()),
                "Save changes to disk and immediately stage into Git index (git add)",
                iced::widget::tooltip::Position::Bottom,
            ),
            Space::new().width(8),
            mayan_tooltip(
                button(text("✕").size(10).color(MayaColors::GLYPH_DIM))
                    .padding(Padding::from([2, 6]))
                    .style(|_, status| {
                        let text_color = match status {
                            button::Status::Hovered => MayaColors::CINNABAR_BRIGHT,
                            _ => MayaColors::GLYPH_DIM,
                        };
                        button::Style {
                            background: Some(Color::TRANSPARENT.into()),
                            text_color,
                            ..Default::default()
                        }
                    })
                    .on_press(EditorMessage::CloseEditor.into()),
                "Close editor buffer",
                iced::widget::tooltip::Position::Bottom,
            ),
        ]
        .padding(Padding::from([4, 10]))
        .align_y(Alignment::Center);

        let editor_widget = text_editor(&self.content)
            .placeholder("Inscribe your code hieroglyphs...")
            .padding(Padding::from(10))
            .on_action(|a| EditorMessage::Edit(a).into())
            .style(|_, _| text_editor::Style {
                background: MayaColors::OBSIDIAN_VOID.into(),
                border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 0.0.into() },
                placeholder: MayaColors::GLYPH_DIM,
                value: MayaColors::STARLIGHT_WHITE,
                selection: MayaColors::JADE_SHADOW,
            });

        let status_bar: Element<'a, Message> = if let Some(ref st) = self.status_message {
            container(text(st).size(9).color(MayaColors::JADE_GLOW))
                .padding(Padding::from([2, 8]))
                .width(Length::Fill)
                .style(|_| container::Style {
                    background: Some(Color { a: 0.95, ..MayaColors::OBSIDIAN_BASE }.into()),
                    ..Default::default()
                })
                .into()
        } else {
            Space::new().height(0).into()
        };

        container(column![header, editor_widget, status_bar])
            .width(Length::Fill)
            .height(Length::Fill)
            .style(|_| container::Style {
                background: Some(Color { a: 0.94, ..MayaColors::OBSIDIAN_BASE }.into()),
                border: Border { color: MayaColors::OBSIDIAN_BORDER, width: 1.0, radius: 0.0.into() },
                ..Default::default()
            })
            .into()
    }
}
