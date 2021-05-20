use egui::color::Color32;
use egui::widgets::Button;

pub enum Tab {
    Active,
    Inactive,
}

impl Tab {
    pub fn text_color() -> Color32 {
        Color32::from_rgb(229, 222, 214)
    }

    pub fn bg_color(&self) -> Color32 {
        match self {
            Tab::Active => Color32::from_rgb(49, 63, 78),
            Tab::Inactive => Color32::from_rgb(35, 45, 56),
        }
    }

    pub fn btn<T: AsRef<str>>(&self, text: T) -> Button {
        Button::new(text.as_ref())
            .text_color(Self::text_color())
            .fill(Some(self.bg_color()))
    }
}
