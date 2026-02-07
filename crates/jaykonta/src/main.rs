//! Point d'entree binaire JayKonta.

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "JayKonta",
        options,
        Box::new(|cc| Ok(Box::new(jaykonta::app::JayKontaApp::new(cc)))),
    )
}
