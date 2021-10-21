pub mod slider_1d;
pub mod slider_2d;
pub mod windows;

use crate::color::{Color, DisplayFormat};

use egui::{
    color,
    color::Color32,
    style::{Selection, Widgets},
    CursorIcon, Id, InnerResponse, LayerId, Order, Rect, Sense, Shape, Stroke, Ui, Vec2, Visuals,
};

pub mod colors {
    use egui::Color32;
    use lazy_static::lazy_static;
    lazy_static! {
        pub static ref D_BG_00: Color32 = Color32::from_rgb(0x11, 0x16, 0x1b);
        pub static ref D_BG_0: Color32 = Color32::from_rgb(0x16, 0x1c, 0x23);
        pub static ref D_BG_1: Color32 = Color32::from_rgb(0x23, 0x2d, 0x38);
        pub static ref D_BG_2: Color32 = Color32::from_rgb(0x31, 0x3f, 0x4e);
        pub static ref D_BG_3: Color32 = Color32::from_rgb(0x41, 0x53, 0x67);
        pub static ref D_FG_0: Color32 = Color32::from_rgb(0xe5, 0xde, 0xd6);
        pub static ref L_BG_0: Color32 = Color32::from_rgb(0xbf, 0xbf, 0xbf);
        pub static ref L_BG_1: Color32 = Color32::from_rgb(0xd4, 0xd3, 0xd4);
        pub static ref L_BG_2: Color32 = Color32::from_rgb(0xd9, 0xd9, 0xd9);
        pub static ref L_BG_3: Color32 = Color32::from_rgb(0xea, 0xea, 0xea);
        pub static ref L_BG_4: Color32 = Color32::from_rgb(0xf9, 0xf9, 0xf9);
        pub static ref L_BG_5: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
        pub static ref L_FG_0: Color32 = *D_BG_0;
    }
}
use colors::*;

pub fn drag_source(ui: &mut Ui, id: Id, body: impl FnOnce(&mut Ui)) {
    let is_being_dragged = ui.memory().is_being_dragged(id);

    if !is_being_dragged {
        let response = ui.scope(body).response;

        // Check for drags:
        let response = ui.interact(response.rect, id, Sense::drag());
        if response.hovered() {
            ui.output().cursor_icon = CursorIcon::Grab;
        }
    } else {
        ui.output().cursor_icon = CursorIcon::Grabbing;

        // Paint the body to a new layer:
        let layer_id = LayerId::new(Order::Tooltip, id);
        let response = ui.with_layer_id(layer_id, body).response;

        // Now we move the visuals of the body to where the mouse is.
        // Normally you need to decide a location for a widget first,
        // because otherwise that widget cannot interact with the mouse.
        // However, a dragged component cannot be interacted with anyway
        // (anything with `Order::Tooltip` always gets an empty `Response`)
        // So this is fine!

        if let Some(pointer_pos) = ui.input().pointer.interact_pos() {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().translate_layer(layer_id, delta);
        }
    }
}

pub fn drop_target<R>(
    ui: &mut Ui,
    can_accept_what_is_being_dragged: bool,
    body: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    let is_being_dragged = ui.memory().is_anything_being_dragged();

    let margin = Vec2::splat(4.0);

    let outer_rect_bounds = ui.available_rect_before_wrap();
    let inner_rect = outer_rect_bounds.shrink2(margin);
    let where_to_put_background = ui.painter().add(Shape::Noop);
    let mut content_ui = ui.child_ui(inner_rect, *ui.layout());
    let ret = body(&mut content_ui);
    let outer_rect = Rect::from_min_max(outer_rect_bounds.min, content_ui.min_rect().max + margin);
    let (rect, response) = ui.allocate_at_least(outer_rect.size(), Sense::hover());

    let style = if is_being_dragged && can_accept_what_is_being_dragged && response.hovered() {
        ui.visuals().widgets.active
    } else {
        ui.visuals().widgets.inactive
    };

    let mut fill = style.bg_fill;
    let mut stroke = style.bg_stroke;
    if is_being_dragged && !can_accept_what_is_being_dragged {
        // gray out:
        fill = color::tint_color_towards(fill, ui.visuals().window_fill());
        stroke.color = color::tint_color_towards(stroke.color, ui.visuals().window_fill());
    }

    ui.painter().set(
        where_to_put_background,
        Shape::Rect {
            corner_radius: style.corner_radius,
            fill,
            stroke,
            rect,
        },
    );

    InnerResponse::new(ret, response)
}

pub fn color_tooltip(color: &Color, display_format: DisplayFormat) -> String {
    format!(
        "{}\n\nPrimary click: set current\nMiddle click: save color\nSecondary click: copy color",
        color.display(display_format)
    )
}

pub fn light_visuals() -> Visuals {
    let mut widgets = Widgets::light();
    widgets.noninteractive.bg_fill = *L_BG_4;
    widgets.inactive.bg_fill = *L_BG_3;
    widgets.inactive.bg_stroke = Stroke::new(0.5, *D_BG_3);
    widgets.inactive.fg_stroke = Stroke::new(0.5, *D_BG_3);
    widgets.hovered.bg_fill = *L_BG_4;
    widgets.hovered.bg_stroke = Stroke::new(1., *D_BG_1);
    widgets.hovered.fg_stroke = Stroke::new(1., *D_BG_1);
    widgets.active.bg_fill = *L_BG_5;
    widgets.active.fg_stroke = Stroke::new(1.5, *D_BG_0);
    widgets.active.bg_stroke = Stroke::new(1.5, *D_BG_0);

    Visuals {
        dark_mode: false,
        override_text_color: Some(*L_FG_0),
        extreme_bg_color: Color32::WHITE,
        selection: Selection {
            bg_fill: *L_BG_5,
            stroke: Stroke::new(0.7, *D_BG_0),
        },
        widgets,
        ..Default::default()
    }
}

pub fn dark_visuals() -> Visuals {
    let mut widgets = Widgets::dark();
    widgets.noninteractive.bg_fill = *D_BG_2;
    widgets.inactive.bg_fill = *D_BG_1;
    widgets.hovered.bg_fill = *D_BG_2;
    widgets.active.bg_fill = *D_BG_3;

    Visuals {
        dark_mode: true,
        override_text_color: Some(*D_FG_0),
        selection: Selection {
            bg_fill: *D_BG_3,
            stroke: Stroke::new(0.7, *D_FG_0),
        },
        widgets,
        ..Default::default()
    }
}
