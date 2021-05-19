use eframe::NativeOptions;

fn main() {
    eframe::run_native(Box::new(epick::Epick::default()), NativeOptions::default())
}
