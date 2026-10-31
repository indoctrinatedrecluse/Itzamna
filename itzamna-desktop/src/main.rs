pub mod app;
pub mod canvas;
pub mod git;
pub mod theme;
pub mod ui;

use app::ItzamnaApp;

fn main() -> iced::Result {
    tracing_subscriber::fmt::init();

    iced::application(
        ItzamnaApp::new,
        ItzamnaApp::update,
        ItzamnaApp::view,
    )
    .subscription(ItzamnaApp::subscription)
    .title("Itzamna — Mayan Git Studio")
    .run()
}
