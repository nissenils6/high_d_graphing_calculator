#[derive(Clone, Copy)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn from_rgb(r: f32, g: f32, b: f32) -> Color {
        Color {
            r, g, b, a: 1.0
        }
    }
    
    pub const fn from_rgba(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color {
            r, g, b, a
        }
    }

    pub fn from_hsv(h: f32, s: f32, v: f32) -> Color {
        let (r, g, b) = Color::hsv_to_rgb(h, s, v);
        Color {
            r, g, b, a: 1.0
        }
    }

    pub fn from_hsva(h: f32, s: f32, v: f32, a: f32) -> Color {
        let (r, g, b) = Color::hsv_to_rgb(h, s, v);
        Color {
            r, g, b, a
        }
    }

    pub fn to_hsv(self) -> (f32, f32, f32) {
        Color::rgb_to_hsv(self.r, self.g, self.b)
    }

    fn hsv_to_rgb(h: f32, s: f32, v: f32) -> (f32, f32, f32) {
        let i = (h * 6.0) as u32;
        let f = h * 6.0 - i as f32;
        let p = v * (1.0 - s);
        let q = v * (1.0 - f * s);
        let t = v * (1.0 - (1.0 - f) * s);

        match i {
            0 => (v, t, p),
            1 => (q, v, p),
            2 => (p, v, t),
            3 => (p, q, v),
            4 => (t, p, v),
            5 => (v, p, q),
            _ => unreachable!()
        }
    }

    fn rgb_to_hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
        let cmax = r.max(g).max(b);
        let cmin = r.min(g).min(b);
        let delta = cmax - cmin;

        let h = if r == cmax {
            ((g - b) / delta % 6.0) / 6.0
        } else if g == cmax {
            ((b - r) / delta % 6.0) / 6.0
        } else if b == cmax {
            ((r - g) / delta % 6.0) / 6.0
        } else {
            unreachable!()
        };

        let s = if cmax == 0.0 {
            0.0
        } else {
            delta / cmax
        };

        let v = cmax;

        (h, s, v)
    }
}