use smolmatrix::*;

pub struct Color(pub Vector<3>);

impl Color {
    pub fn get_rgb(&self) -> [u8; 3] {
        [
            (255.0 * self.0.x().min(1.0).max(0.0).sqrt()).round() as u8,
            (255.0 * self.0.y().min(1.0).max(0.0).sqrt()).round() as u8,
            (255.0 * self.0.z().min(1.0).max(0.0).sqrt()).round() as u8,
        ]
    }
}
