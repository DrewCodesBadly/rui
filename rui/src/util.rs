use bytemuck::{AnyBitPattern, NoUninit};

#[repr(C)]
#[derive(Clone, Copy, NoUninit, AnyBitPattern)]
pub struct Color {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

impl Default for Color {
    fn default() -> Self {
        Self {
            r: 1.0,
            g: 1.0,
            b: 1.0,
            a: 1.0,
        }
    }
}

impl Color {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r: r as f32 / 255.0,
            g: g as f32 / 255.0,
            b: b as f32 / 255.0,
            a: a as f32 / 255.0,
        }
    }
}
