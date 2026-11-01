//! Sacred Opaque Tooltip System
//! Renders 100% solid, high-contrast Mesoamerican tooltip frames with obsidian voids and solar gold borders.

use iced::widget::{container, text, tooltip};
pub use iced::widget::tooltip::Position;
use iced::{Border, Element, Padding};

use crate::theme::MayaColors;

pub fn mayan_tooltip<'a, Message: 'a>(
    content: impl Into<Element<'a, Message>>,
    description: impl Into<String>,
    position: Position,
) -> Element<'a, Message> {
    let desc_str = description.into();
    let tooltip_box = container(
        text(desc_str)
            .size(11)
            .color(MayaColors::STARLIGHT_WHITE)
    )
    .padding(Padding::from([5, 9]))
    .style(|_| container::Style {
        background: Some(MayaColors::OBSIDIAN_VOID.into()), // 100% solid, completely opaque!
        border: Border {
            color: MayaColors::SOLAR_GOLD,
            width: 1.0,
            radius: 4.0.into(),
        },
        ..Default::default()
    });

    tooltip(content, tooltip_box, position).into()
}
