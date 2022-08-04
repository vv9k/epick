use eframe::{IconData, NativeOptions};
use image::{io::Reader as ImageReader, ImageFormat};
use std::io::Cursor;

const APP_ICON_DATA: &[u8] = include_bytes!("../assets/icon.png");
const APP_ICON_WIDTH: u32 = 48;
const APP_ICON_HEIGHT: u32 = 48;

fn main() {
    let mut opts = NativeOptions::default();

    //pretty_env_logger::init();
    //let subscriber = tracing_subscriber::fmt()
    //.with_file(true)
    //.with_target(true)
    //.with_line_number(true)
    //.pretty()
    //.with_max_level(tracing::Level::TRACE)
    //.finish();
    //tracing::subscriber::set_global_default(subscriber).unwrap();

    let mut img = ImageReader::new(Cursor::new(APP_ICON_DATA));
    img.set_format(ImageFormat::Png);
    match img
        .decode()
        .map(|img| img.as_rgba8().map(|img| img.to_vec()))
    {
        Ok(Some(rgba)) => {
            opts.icon_data = Some(IconData {
                rgba,
                width: APP_ICON_WIDTH,
                height: APP_ICON_HEIGHT,
            })
        }
        Err(e) => {
            eprintln!("failed to load app icon data - {}", e);
        }
        _ => {}
    }

    eframe::run_native(
        "epick - Color Picker",
        opts,
        Box::new(|ctx| epick::Epick::init(ctx)),
    )
}
