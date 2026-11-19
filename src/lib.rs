pub mod app;
pub mod i18n;
pub mod model;
pub mod storage;
pub mod ui;
pub mod utils;

pub use app::PapelApp;

pub const APP_ID: &str = "io.github.papel.editor";

pub fn run() {
    // Desactivar portal externo problemático de GNOME para que el diálogo corra directo en el proceso
    std::env::set_var("GTK_USE_PORTAL", "0");

    libadwaita::init().expect("Error al inicializar Libadwaita");

    let app = libadwaita::Application::builder()
        .application_id(APP_ID)
        .build();

    use libadwaita::prelude::*;
    app.connect_activate(|app| {
        let editor = PapelApp::build(app);
        editor.present();
    });

    app.run();
}
