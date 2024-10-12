use crate::color::Color;

/// BSDF materials based on Blender's Principled BSDF
pub struct Bsdf {
    pub base_color: Color,
    pub metallic: f32,
    pub roughness: f32,
    pub ior: f32,
    // pub alpha: f32,

    pub transmission: Transmission,
    pub emission: Emission,
}

pub struct Transmission {
    pub weight: f32,
}

pub struct Emission {
    pub color: Color,
    pub strength: f32,
}
