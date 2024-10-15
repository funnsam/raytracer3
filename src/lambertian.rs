use core::f32::consts::*;
use smolmatrix::*;

pub fn sample(normal: &Vector<3>) -> Vector<3> {
    (crate::utils::random_unit_vector() + normal).unit()
}

pub fn pdf(n_dot_l: f32) -> f32 {
    FRAC_1_PI * n_dot_l
}
