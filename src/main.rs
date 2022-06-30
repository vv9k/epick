use eframe::NativeOptions;

fn main() {
    let opts = NativeOptions {
        always_on_top: true,
        ..Default::default()
    };
    eframe::run_native("epick", opts, Box::new(|ctx| epick::Epick::init(ctx)))
}
