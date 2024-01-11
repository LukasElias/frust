extern crate frust_core;

mod editor;

fn main() {
    let native_options = eframe::NativeOptions {
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };

    eframe::run_native("frust", native_options, Box::new(
        |cc| {
            Box::new(editor::App::new(cc))
        }
    )).unwrap();
}