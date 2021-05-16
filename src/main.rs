use eframe::NativeOptions;

fn main() {
    eframe::run_native(
        Box::new(testgui::ColorPicker::default()),
        NativeOptions::default(),
    )
}
