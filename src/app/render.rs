// Mostly taken from egui's demos
use crate::color::{contrast_color, Gradient};
use egui::color::Color32;
use egui::{
    lerp, pos2, remap_clamp, vec2, ImageButton, Painter, Rect, Response, Sense, Shape, Stroke,
    TextureId, Ui, Vec2,
};
use epaint::Mesh;
use std::{collections::HashMap, ops::RangeInclusive};

/// Number of vertices per dimension in the color sliders.
/// We need at least 6 for hues, and more for smooth 2D areas.
/// Should always be a multiple of 6 to hit the peak hues in HSV/HSL (every 60°).
const N: u32 = 6 * 6;

pub fn tex_color(
    ui: &mut Ui,
    tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    tex_mngr: &mut TextureManager,
    color: Color32,
    size: Vec2,
    on_hover: Option<&str>,
) -> Option<Response> {
    let gradient = Gradient::one_color(color);
    tex_gradient(ui, tex_allocator, tex_mngr, &gradient, size, on_hover)
}

pub fn tex_gradient(
    ui: &mut Ui,
    tex_allocator: &mut Option<&mut dyn epi::TextureAllocator>,
    tex_mngr: &mut TextureManager,
    gradient: &Gradient,
    size: Vec2,
    on_hover: Option<&str>,
) -> Option<Response> {
    if let Some(tex_allocator) = tex_allocator {
        let resp = ui.horizontal(|ui| {
            let tex = tex_mngr.get(*tex_allocator, &gradient);
            let texel_offset = 0.5 / (gradient.0.len() as f32);
            let uv = Rect::from_min_max(pos2(texel_offset, 0.0), pos2(1.0 - texel_offset, 1.0));
            let image = ImageButton::new(tex, size).uv(uv);
            let mut resp = ui.add(image);

            if let Some(on_hover) = on_hover {
                resp = resp.on_hover_text(on_hover);
            }

            resp
        });
        return Some(resp.inner);
    }
    None
}

fn background_checkers(painter: &Painter, rect: Rect) {
    let rect = rect.shrink(0.5); // Small hack to avoid the checkers from peeking through the sides
    if !rect.is_positive() {
        return;
    }

    let mut top_color = Color32::from_gray(128);
    let mut bottom_color = Color32::from_gray(32);
    let checker_size = Vec2::splat(rect.height() / 2.0);
    let n = (rect.width() / checker_size.x).round() as u32;

    let mut mesh = Mesh::default();
    for i in 0..n {
        let x = lerp(rect.left()..=rect.right(), i as f32 / (n as f32));
        mesh.add_colored_rect(
            Rect::from_min_size(pos2(x, rect.top()), checker_size),
            top_color,
        );
        mesh.add_colored_rect(
            Rect::from_min_size(pos2(x, rect.center().y), checker_size),
            bottom_color,
        );
        std::mem::swap(&mut top_color, &mut bottom_color);
    }
    painter.add(Shape::mesh(mesh));
}

pub fn color_slider_1d(
    ui: &mut Ui,
    value: &mut f32,
    range: RangeInclusive<f32>,
    color_at: impl Fn(f32) -> Color32,
) -> Response {
    #![allow(clippy::identity_op)]

    let desired_size = vec2(
        ui.spacing().slider_width * 2.5,
        ui.spacing().interact_size.y * 2.,
    );
    let (rect, response) = ui.allocate_at_least(desired_size, Sense::click_and_drag());

    if let Some(mpos) = response.interact_pointer_pos() {
        *value = remap_clamp(mpos.x, rect.left()..=rect.right(), range);
    }

    let visuals = ui.style().interact(&response);

    background_checkers(ui.painter(), rect); // for alpha:

    {
        // fill color:
        let mut mesh = Mesh::default();
        for i in 0..=N {
            let t = i as f32 / (N as f32);
            let color = color_at(t);
            let x = lerp(rect.left()..=rect.right(), t);
            mesh.colored_vertex(pos2(x, rect.top()), color);
            mesh.colored_vertex(pos2(x, rect.bottom()), color);
            if i < N {
                mesh.add_triangle(2 * i + 0, 2 * i + 1, 2 * i + 2);
                mesh.add_triangle(2 * i + 1, 2 * i + 2, 2 * i + 3);
            }
        }
        ui.painter().add(Shape::mesh(mesh));
    }

    ui.painter().rect_stroke(rect, 0.0, visuals.bg_stroke); // outline

    {
        // Show where the slider is at:
        let x = if *value >= 0. && *value <= 1.0 {
            lerp(rect.left()..=rect.right(), *value)
        } else {
            rect.left() + *value
        };
        let r = rect.height() / 4.0;
        let picked_color = color_at(*value);
        ui.painter().add(Shape::polygon(
            vec![
                pos2(x - r, rect.bottom()),
                pos2(x + r, rect.bottom()),
                pos2(x, rect.center().y),
            ],
            picked_color,
            Stroke::new(visuals.fg_stroke.width, contrast_color(picked_color)),
        ));
    }

    response
}

#[derive(Default, Debug)]
pub struct TextureManager(HashMap<Gradient, TextureId>);

impl TextureManager {
    fn get(
        &mut self,
        tex_allocator: &mut dyn epi::TextureAllocator,
        gradient: &Gradient,
    ) -> TextureId {
        *self.0.entry(gradient.clone()).or_insert_with(|| {
            let pixels = gradient.to_pixel_row();
            let width = pixels.len();
            let height = 1;
            tex_allocator.alloc_srgba_premultiplied((width, height), &pixels)
        })
    }
}
