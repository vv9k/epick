use eframe::NativeOptions;

fn main() {
    eframe::run_native(
        Box::new(epick::ColorPicker::default()),
        NativeOptions::default(),
    )
}
