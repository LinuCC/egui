use crate::math::clamp;

/// This format is used for space-efficient color representation (32 bits).
///
/// Instead of manipulating this directly it is often better
/// to first convert it to either [`Rgba`] or [`Hsva`].
///
/// Internally this uses 0-255 gamma space `sRGBA` color with premultiplied alpha.
/// Alpha channel is in linear space.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Color32(pub(crate) [u8; 4]);

impl std::ops::Index<usize> for Color32 {
    type Output = u8;
    fn index(&self, index: usize) -> &u8 {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Color32 {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.0[index]
    }
}

#[deprecated = "Replaced by Color32::from_rgb… family of functions."]
pub const fn srgba(r: u8, g: u8, b: u8, a: u8) -> Color32 {
    Color32::from_rgba_premultiplied(r, g, b, a)
}

impl Color32 {
    pub const TRANSPARENT: Color32 = Color32::from_rgba_premultiplied(0, 0, 0, 0);
    pub const BLACK: Color32 = Color32::from_rgb(0, 0, 0);
    pub const LIGHT_GRAY: Color32 = Color32::from_rgb(220, 220, 220);
    pub const GRAY: Color32 = Color32::from_rgb(160, 160, 160);
    pub const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
    pub const RED: Color32 = Color32::from_rgb(255, 0, 0);
    pub const GREEN: Color32 = Color32::from_rgb(0, 255, 0);
    pub const BLUE: Color32 = Color32::from_rgb(0, 0, 255);
    pub const YELLOW: Color32 = Color32::from_rgb(255, 255, 0);
    pub const LIGHT_BLUE: Color32 = Color32::from_rgb(140, 160, 255);

    pub const fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b, 255])
    }

    pub const fn from_rgb_additive(r: u8, g: u8, b: u8) -> Self {
        Self([r, g, b, 0])
    }

    /// From `sRGBA` with premultiplied alpha.
    pub const fn from_rgba_premultiplied(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    /// From `sRGBA` WITHOUT premultiplied alpha.
    pub fn from_rgba_unmultiplied(r: u8, g: u8, b: u8, a: u8) -> Self {
        if a == 255 {
            Self::from_rgba_premultiplied(r, g, b, a) // common-case optimization
        } else {
            Rgba::from(Self::from_rgb(r, g, b))
                .multiply(a as f32 / 255.0)
                .into()
        }
    }

    #[deprecated = "Use from_rgb(..), from_rgba_premultiplied(..) or from_srgba_unmultiplied(..)"]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self([r, g, b, a])
    }

    pub const fn gray(l: u8) -> Self {
        Self([l, l, l, 255])
    }

    pub const fn black_alpha(a: u8) -> Self {
        Self([0, 0, 0, a])
    }

    pub fn white_alpha(a: u8) -> Self {
        Rgba::white_alpha(linear_from_alpha_byte(a)).into()
    }

    pub const fn additive_luminance(l: u8) -> Self {
        Self([l, l, l, 0])
    }

    pub fn is_opaque(&self) -> bool {
        self.a() == 255
    }

    pub fn r(&self) -> u8 {
        self.0[0]
    }
    pub fn g(&self) -> u8 {
        self.0[1]
    }
    pub fn b(&self) -> u8 {
        self.0[2]
    }
    pub fn a(&self) -> u8 {
        self.0[3]
    }

    /// Returns an opaque version of self
    pub fn to_opaque(self) -> Self {
        Rgba::from(self).to_opaque().into()
    }

    pub fn to_array(&self) -> [u8; 4] {
        [self.r(), self.g(), self.b(), self.a()]
    }

    pub fn to_tuple(&self) -> (u8, u8, u8, u8) {
        (self.r(), self.g(), self.b(), self.a())
    }
}

// ----------------------------------------------------------------------------

/// 0-1 linear space `RGBA` color with premultiplied alpha.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Rgba(pub(crate) [f32; 4]);

impl std::ops::Index<usize> for Rgba {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        &self.0[index]
    }
}

impl std::ops::IndexMut<usize> for Rgba {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        &mut self.0[index]
    }
}

impl Rgba {
    pub const TRANSPARENT: Rgba = Rgba::new(0.0, 0.0, 0.0, 0.0);
    pub const BLACK: Rgba = Rgba::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0);
    pub const RED: Rgba = Rgba::new(1.0, 0.0, 0.0, 1.0);
    pub const GREEN: Rgba = Rgba::new(0.0, 1.0, 0.0, 1.0);
    pub const BLUE: Rgba = Rgba::new(0.0, 0.0, 1.0, 1.0);

    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self([r, g, b, a])
    }

    pub const fn rgb(r: f32, g: f32, b: f32) -> Self {
        Self([r, g, b, 1.0])
    }

    pub const fn gray(l: f32) -> Self {
        Self([l, l, l, 1.0])
    }

    pub fn luminance_alpha(l: f32, a: f32) -> Self {
        debug_assert!(0.0 <= l && l <= 1.0);
        debug_assert!(0.0 <= a && a <= 1.0);
        Self([l * a, l * a, l * a, a])
    }

    /// Transparent black
    pub fn black_alpha(a: f32) -> Self {
        debug_assert!(0.0 <= a && a <= 1.0);
        Self([0.0, 0.0, 0.0, a])
    }

    /// Transparent white
    pub fn white_alpha(a: f32) -> Self {
        debug_assert!(0.0 <= a && a <= 1.0);
        Self([a, a, a, a])
    }

    /// Multiply with e.g. 0.5 to make us half transparent
    pub fn multiply(self, alpha: f32) -> Self {
        Self([
            alpha * self[0],
            alpha * self[1],
            alpha * self[2],
            alpha * self[3],
        ])
    }

    pub fn r(&self) -> f32 {
        self.0[0]
    }
    pub fn g(&self) -> f32 {
        self.0[1]
    }
    pub fn b(&self) -> f32 {
        self.0[2]
    }
    pub fn a(&self) -> f32 {
        self.0[3]
    }

    /// How perceptually intense (bright) is the color?
    pub fn intensity(&self) -> f32 {
        0.3 * self.r() + 0.59 * self.g() + 0.11 * self.b()
    }

    /// Returns an opaque version of self
    pub fn to_opaque(&self) -> Self {
        if self.a() == 0.0 {
            // additive or fully transparent
            Self::new(self.r(), self.g(), self.b(), 1.0)
        } else {
            // un-multiply alpha
            Self::new(
                self.r() / self.a(),
                self.g() / self.a(),
                self.b() / self.a(),
                1.0,
            )
        }
    }
}

impl std::ops::Add for Rgba {
    type Output = Rgba;
    fn add(self, rhs: Rgba) -> Rgba {
        Rgba([
            self[0] + rhs[0],
            self[1] + rhs[1],
            self[2] + rhs[2],
            self[3] + rhs[3],
        ])
    }
}

impl std::ops::Mul<Rgba> for Rgba {
    type Output = Rgba;
    fn mul(self, other: Rgba) -> Rgba {
        Rgba([
            self[0] * other[0],
            self[1] * other[1],
            self[2] * other[2],
            self[3] * other[3],
        ])
    }
}

impl std::ops::Mul<f32> for Rgba {
    type Output = Rgba;
    fn mul(self, factor: f32) -> Rgba {
        Rgba([
            self[0] * factor,
            self[1] * factor,
            self[2] * factor,
            self[3] * factor,
        ])
    }
}

impl std::ops::Mul<Rgba> for f32 {
    type Output = Rgba;
    fn mul(self, rgba: Rgba) -> Rgba {
        Rgba([
            self * rgba[0],
            self * rgba[1],
            self * rgba[2],
            self * rgba[3],
        ])
    }
}

// ----------------------------------------------------------------------------
// Color conversion:

impl From<Color32> for Rgba {
    fn from(srgba: Color32) -> Rgba {
        Rgba([
            linear_from_gamma_byte(srgba[0]),
            linear_from_gamma_byte(srgba[1]),
            linear_from_gamma_byte(srgba[2]),
            linear_from_alpha_byte(srgba[3]),
        ])
    }
}

impl From<Rgba> for Color32 {
    fn from(rgba: Rgba) -> Color32 {
        Color32([
            gamma_byte_from_linear(rgba[0]),
            gamma_byte_from_linear(rgba[1]),
            gamma_byte_from_linear(rgba[2]),
            alpha_byte_from_linear(rgba[3]),
        ])
    }
}

/// [0, 255] -> [0, 1]
fn linear_from_gamma_byte(s: u8) -> f32 {
    if s <= 10 {
        s as f32 / 3294.6
    } else {
        ((s as f32 + 14.025) / 269.025).powf(2.4)
    }
}

fn linear_from_alpha_byte(a: u8) -> f32 {
    a as f32 / 255.0
}

/// [0, 1] -> [0, 255]
fn gamma_byte_from_linear(l: f32) -> u8 {
    if l <= 0.0 {
        0
    } else if l <= 0.0031308 {
        (3294.6 * l).round() as u8
    } else if l <= 1.0 {
        (269.025 * l.powf(1.0 / 2.4) - 14.025).round() as u8
    } else {
        255
    }
}

fn alpha_byte_from_linear(a: f32) -> u8 {
    clamp(a * 255.0, 0.0..=255.0).round() as u8
}

#[test]
fn test_srgba_conversion() {
    #![allow(clippy::float_cmp)]
    for b in 0..=255 {
        let l = linear_from_gamma_byte(b);
        assert!(0.0 <= l && l <= 1.0);
        assert_eq!(gamma_byte_from_linear(l), b);
    }
}

// ----------------------------------------------------------------------------

/// Hue, saturation, value, alpha. All in the range [0, 1].
/// No premultiplied alpha.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Hsva {
    /// hue 0-1
    pub h: f32,
    /// saturation 0-1
    pub s: f32,
    /// value 0-1
    pub v: f32,
    /// alpha 0-1
    pub a: f32,
}

impl Hsva {
    pub fn new(h: f32, s: f32, v: f32, a: f32) -> Self {
        Self { h, s, v, a }
    }

    /// From `sRGBA` with premultiplied alpha
    pub fn from_srgba_premultiplied(srgba: [u8; 4]) -> Self {
        Self::from_rgba_premultiplied([
            linear_from_gamma_byte(srgba[0]),
            linear_from_gamma_byte(srgba[1]),
            linear_from_gamma_byte(srgba[2]),
            linear_from_alpha_byte(srgba[3]),
        ])
    }

    /// From `sRGBA` without premultiplied alpha
    pub fn from_srgba_unmultiplied(srgba: [u8; 4]) -> Self {
        Self::from_rgba_unmultiplied([
            linear_from_gamma_byte(srgba[0]),
            linear_from_gamma_byte(srgba[1]),
            linear_from_gamma_byte(srgba[2]),
            linear_from_alpha_byte(srgba[3]),
        ])
    }

    /// From linear RGBA with premultiplied alpha
    pub fn from_rgba_premultiplied(rgba: [f32; 4]) -> Self {
        #![allow(clippy::many_single_char_names)]
        let [r, g, b, a] = rgba;
        if a == 0.0 {
            Hsva::default()
        } else {
            let (h, s, v) = hsv_from_rgb((r / a, g / a, b / a));
            Hsva { h, s, v, a }
        }
    }

    /// From linear RGBA without premultiplied alpha
    pub fn from_rgba_unmultiplied(rgba: [f32; 4]) -> Self {
        #![allow(clippy::many_single_char_names)]
        let [r, g, b, a] = rgba;
        let (h, s, v) = hsv_from_rgb((r, g, b));
        Hsva { h, s, v, a }
    }

    pub fn to_rgba_premultiplied(&self) -> [f32; 4] {
        let [r, g, b, a] = self.to_rgba_unmultiplied();
        [a * r, a * g, a * b, a]
    }

    pub fn to_rgba_unmultiplied(&self) -> [f32; 4] {
        let Hsva { h, s, v, a } = *self;
        let (r, g, b) = rgb_from_hsv((h, s, v));
        [r, g, b, a]
    }

    pub fn to_srgba_premultiplied(&self) -> [u8; 4] {
        let [r, g, b, a] = self.to_rgba_premultiplied();
        [
            gamma_byte_from_linear(r),
            gamma_byte_from_linear(g),
            gamma_byte_from_linear(b),
            alpha_byte_from_linear(a),
        ]
    }

    pub fn to_srgba_unmultiplied(&self) -> [u8; 4] {
        let [r, g, b, a] = self.to_rgba_unmultiplied();
        [
            gamma_byte_from_linear(r),
            gamma_byte_from_linear(g),
            gamma_byte_from_linear(b),
            alpha_byte_from_linear(a),
        ]
    }
}

impl From<Hsva> for Rgba {
    fn from(hsva: Hsva) -> Rgba {
        Rgba(hsva.to_rgba_premultiplied())
    }
}
impl From<Rgba> for Hsva {
    fn from(rgba: Rgba) -> Hsva {
        Self::from_rgba_premultiplied(rgba.0)
    }
}

impl From<Hsva> for Color32 {
    fn from(hsva: Hsva) -> Color32 {
        Color32::from(Rgba::from(hsva))
    }
}
impl From<Color32> for Hsva {
    fn from(srgba: Color32) -> Hsva {
        Hsva::from(Rgba::from(srgba))
    }
}

/// All ranges in 0-1, rgb is linear.
pub fn hsv_from_rgb((r, g, b): (f32, f32, f32)) -> (f32, f32, f32) {
    #![allow(clippy::float_cmp)]
    #![allow(clippy::many_single_char_names)]
    let min = r.min(g.min(b));
    let max = r.max(g.max(b)); // value

    let range = max - min;

    let h = if max == min {
        0.0 // hue is undefined
    } else if max == r {
        (g - b) / (6.0 * range)
    } else if max == g {
        (b - r) / (6.0 * range) + 1.0 / 3.0
    } else {
        // max == b
        (r - g) / (6.0 * range) + 2.0 / 3.0
    };
    let h = (h + 1.0).fract(); // wrap
    let s = if max == 0.0 { 0.0 } else { 1.0 - min / max };
    (h, s, max)
}

/// All ranges in 0-1, rgb is linear.
pub fn rgb_from_hsv((h, s, v): (f32, f32, f32)) -> (f32, f32, f32) {
    #![allow(clippy::many_single_char_names)]
    let h = (h.fract() + 1.0).fract(); // wrap
    let s = clamp(s, 0.0..=1.0);

    let f = h * 6.0 - (h * 6.0).floor();
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);

    match (h * 6.0).floor() as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        5 => (v, p, q),
        _ => unreachable!(),
    }
}

#[test]
#[ignore] // a bit expensive
fn test_hsv_roundtrip() {
    for r in 0..=255 {
        for g in 0..=255 {
            for b in 0..=255 {
                let srgba = Color32::from_rgb(r, g, b);
                let hsva = Hsva::from(srgba);
                assert_eq!(srgba, Color32::from(hsva));
            }
        }
    }
}
