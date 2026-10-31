pub mod app;
pub mod canvas;
pub mod git;
pub mod theme;
pub mod ui;

use app::ItzamnaApp;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    let icon_bytes = include_bytes!("../assets/icon.png");
    let icon = image::load_from_memory(icon_bytes)
        .ok()
        .and_then(|img| {
            let rgba = img.to_rgba8();
            let (width, height) = rgba.dimensions();
            iced::window::icon::from_rgba(rgba.into_raw(), width, height).ok()
        });

    iced::application(
        ItzamnaApp::new,
        ItzamnaApp::update,
        ItzamnaApp::view,
    )
    .subscription(ItzamnaApp::subscription)
    .title("Itzamna — Mayan Git Studio")
    .window(iced::window::Settings {
        icon,
        ..Default::default()
    })
    .run()
}
