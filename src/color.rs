use smolmatrix::*;

pub struct Color(pub Vector<3>);

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{} {} {} ",
            (255.0 * self.0[0]).round() as u8,
            (255.0 * self.0[1]).round() as u8,
            (255.0 * self.0[2]).round() as u8,/*
            (255.0 * self.0[0].max(0.0).sqrt()).round() as u8,
            (255.0 * self.0[1].max(0.0).sqrt()).round() as u8,
            (255.0 * self.0[2].max(0.0).sqrt()).round() as u8,*/
        )
    }
}
