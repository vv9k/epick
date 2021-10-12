use crate::color::Color;

#[derive(Debug, Clone)]
pub struct ColorSliders {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub hue: f32,
    pub sat: f32,
    pub val: f32,
    pub c: f32,
    pub m: f32,
    pub y: f32,
    pub k: f32,
    pub lch_l: f32,
    pub lch_c: f32,
    pub lch_h: f32,
    pub hsl_h: f32,
    pub hsl_s: f32,
    pub hsl_l: f32,
}

impl Default for ColorSliders {
    fn default() -> Self {
        Self {
            r: 0.,
            g: 0.,
            b: 0.,
            hue: 0.,
            sat: 0.,
            val: 0.,
            c: 0.,
            m: 0.,
            y: 0.,
            k: 100.,
            lch_l: 0.,
            lch_c: 0.,
            lch_h: 0.,
            hsl_h: 0.,
            hsl_s: 0.,
            hsl_l: 0.,
        }
    }
}

impl ColorSliders {
    pub fn set_color(&mut self, color: Color) {
        let rgb = color.rgb();
        self.r = rgb.r_scaled();
        self.g = rgb.g_scaled();
        self.b = rgb.b_scaled();
        let cmyk = color.cmyk();
        self.c = cmyk.c_scaled();
        self.m = cmyk.m_scaled();
        self.y = cmyk.y_scaled();
        self.k = cmyk.k_scaled();
        let hsl = color.hsl();
        self.hsl_h = hsl.h_scaled();
        self.hsl_s = hsl.s_scaled();
        self.hsl_l = hsl.l_scaled();
        let hsv = color.hsv();
        self.hue = hsv.h_scaled();
        self.sat = hsv.s_scaled();
        self.val = hsv.v_scaled();
        let lch = color.lch();
        self.lch_l = lch.l;
        self.lch_h = lch.c;
        self.lch_c = lch.h;
    }

    pub fn restore(&mut self, other: Self) {
        self.r = other.r;
        self.g = other.g;
        self.b = other.b;
        self.hue = other.hue;
        self.sat = other.sat;
        self.val = other.val;
        self.c = other.c;
        self.m = other.m;
        self.y = other.y;
        self.k = other.k;
        self.hsl_h = other.hsl_h;
        self.hsl_s = other.hsl_s;
        self.hsl_l = other.hsl_l;
    }
}
