use eframe::NativeOptions;

fn main() {
    eframe::run_native(
        "epick",
        NativeOptions::default(),
        Box::new(|_| epick::Epick::init()),
    )
}
