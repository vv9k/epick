use eframe::NativeOptions;

fn main() {
    eframe::run_native(
        Box::new(libepick::Epick::default()),
        NativeOptions::default(),
    )
}
