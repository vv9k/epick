use egui::color::Color32;
use egui::widgets::Button;

pub enum Tab {
    Active,
    Inactive,
}

impl Tab {
    pub fn text_color(&self) -> Color32 {
        match self {
            Tab::Active => Color32::from_rgb(229, 222, 214),
            Tab::Inactive => Color32::from_rgb(142, 151, 162),
        }
    }

    pub fn bg_color(&self) -> Color32 {
        match self {
            Tab::Active => Color32::from_rgb(49, 63, 78),
            Tab::Inactive => Color32::from_rgb(22, 28, 35),
        }
    }

    pub fn btn<T: AsRef<str>>(&self, text: T) -> Button {
        Button::new(text.as_ref())
            .text_color(self.text_color())
            .fill(Some(self.bg_color()))
    }
}
