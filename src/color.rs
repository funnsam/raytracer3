use smolmatrix::*;

pub struct Color(pub Vector<3>);

impl Color {
    pub fn get_rgb(&self) -> [u8; 3] {
        [
            (255.0 * self.0[0].min(1.0).max(0.0).sqrt()).round() as u8,
            (255.0 * self.0[1].min(1.0).max(0.0).sqrt()).round() as u8,
            (255.0 * self.0[2].min(1.0).max(0.0).sqrt()).round() as u8,
        ]
    }
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c = self.get_rgb();
        write!(
            f,
            "{} {} {} ",
            c[0],
            c[1],
            c[2],
        )
    }
}
