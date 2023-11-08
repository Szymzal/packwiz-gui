use packwiz_gui::app::App;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
    let options = eframe::NativeOptions {
        initial_window_pos: Some(egui::pos2(320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Packwiz GUI",
        options,
        Box::new(|_cc| Box::<App>::default()),
    )
}
