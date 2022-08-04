use crate::color::Color;

pub const COLORBOX_PICK_TOOLTIP: &str =
    "Primary click: set current\nMiddle click: save color\nSecondary click: copy color";
pub const COLORBOX_DRAG_TOOLTIP: &str = "Drag and drop to change the order of colors";

#[derive(Clone)]
pub struct ColorBox {
    size: egui::Vec2,
    border: bool,
    color: Color,
    label: bool,
    hover_help: Option<String>,
}

impl ColorBox {
    pub fn builder() -> ColorBoxBuilder {
        ColorBoxBuilder::default()
    }

    pub fn size(&self) -> egui::Vec2 {
        self.size
    }

    pub fn border(&self) -> bool {
        self.border
    }

    pub fn color(&self) -> Color {
        self.color
    }

    pub fn label(&self) -> bool {
        self.label
    }

    pub fn hover_help(&self) -> Option<&str> {
        self.hover_help.as_deref()
    }
}

#[derive(Clone)]
pub struct ColorBoxBuilder {
    size: egui::Vec2,
    border: bool,
    color: Color,
    label: bool,
    hover_help: Option<String>,
}

impl Default for ColorBoxBuilder {
    fn default() -> Self {
        Self {
            size: (0., 0.).into(),
            border: false,
            color: Color::white(),
            label: false,
            hover_help: None,
        }
    }
}

impl ColorBoxBuilder {
    pub fn build(self) -> ColorBox {
        ColorBox {
            size: self.size,
            border: self.border,
            color: self.color,
            label: self.label,
            hover_help: self.hover_help,
        }
    }

    pub fn size(mut self, size: impl Into<egui::Vec2>) -> Self {
        self.size = size.into();
        self
    }

    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }

    pub fn label(mut self, label: bool) -> Self {
        self.label = label;
        self
    }

    pub fn color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn hover_help(mut self, hover_help: impl Into<String>) -> Self {
        self.hover_help = Some(hover_help.into());
        self
    }
}
