use crate::color::Color;

/// BSDF materials based on Blender's Principled BSDF
pub struct Bsdf {
    pub base_color: Color,
    pub metallic: f32,
    pub roughness: f32,
    pub ior: f32,
    // pub alpha: f32,

    pub emission: Emission,
}

pub struct Emission {
    pub color: Color,
    pub strength: f32,
}
