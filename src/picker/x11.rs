#![cfg(unix)]
use crate::color::Color;
use crate::picker::DisplayPicker;
use anyhow::{Context, Result};
use egui::Color32;
use x11rb::connection::Connection;
use x11rb::image::Image;
use x11rb::protocol::xproto::{ConnectionExt, Screen, Window};
use x11rb::rust_connection::RustConnection;

fn get_cursor_xy<C: Connection>(conn: &C, window: Window) -> Result<(i16, i16)> {
    conn.query_pointer(window)
        .context("connection failed")?
        .reply()
        .context("failed to query pointer")
        .map(|reply| (reply.root_x, reply.root_y))
}

fn get_color<C: Connection>(conn: &C, window: Window, x: i16, y: i16) -> Result<(u8, u8, u8)> {
    let img = Image::get(conn, window, x, y, 1, 1).context("failed to get image")?;
    let pixel = img.get_pixel(0, 0);

    let red = (pixel >> 8) & 0xff;
    let green = (pixel >> 16) & 0xff;
    let blue = (pixel >> 24) & 0xff;

    Ok((red as u8, green as u8, blue as u8))
}

fn get_color_for_screen<C: Connection>(conn: &C, screen: &Screen) -> Result<(u8, u8, u8)> {
    let (x, y) = get_cursor_xy(conn, screen.root)?;
    get_color(conn, screen.root, x, y)
}

fn get_color_for_conn<C: Connection>(conn: &C, screen_num: usize) -> Result<(u8, u8, u8)> {
    let screen = &conn.setup().roots[screen_num];
    get_color_for_screen(conn, screen)
}

#[derive(Debug)]
pub struct X11Conn {
    conn: RustConnection,
    screen_num: usize,
}

impl X11Conn {
    pub fn new() -> Result<Self> {
        let (conn, screen_num) = x11rb::connect(None).context("failed to connect to x11 server")?;

        Ok(Self { conn, screen_num })
    }
}

impl DisplayPicker for X11Conn {
    fn get_color_under_cursor(&self) -> Result<Color> {
        get_color_for_conn(&self.conn, self.screen_num)
            .map(|color| Color32::from_rgb(color.0, color.1, color.2).into())
    }
}
