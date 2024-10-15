use smolmatrix::*;

pub fn pdf(metallic: f32, d: f32, g1_n_dot_v: f32, n_dot_l: f32) -> f32 {
    crate::lambertian::pdf(n_dot_l) * (1.0 - metallic) + (d * g1_n_dot_v / (4.0 * n_dot_l)).max(0.001) * metallic
}

pub fn g1(alpha: f32, x_dot_n: f32) -> f32 {
    2.0 / (1.0 + (1.0 + alpha * alpha * ((1.0 - x_dot_n * x_dot_n) / (x_dot_n * x_dot_n))).sqrt())
}

pub fn d(alpha: f32, h_dot_n: f32) -> f32 {
    let sq = alpha / (h_dot_n * h_dot_n * (alpha * alpha - 1.0) + 1.0);
    core::f32::consts::FRAC_1_PI * sq * sq
}

pub fn sample_vndf(v_tangent: &Vector<3>, alpha: f32) -> Vector<3> {
    let v_tangent_stretched = vector!(3 [v_tangent.x() * alpha, v_tangent.y() * alpha, v_tangent.z()]).unit();
    let phi = 2.0 * core::f32::consts::PI * fastrand::f32();

    let mut hemisphere = vector!(3 [phi.cos(), phi.sin(), (1.0 - fastrand::f32()) * (1.0 + v_tangent.z()) - v_tangent_stretched.z()]);
    let coeff = ((1.0 - hemisphere.z() * hemisphere.z()).max(0.0).min(1.0)).sqrt();
    *hemisphere.x_mut() *= coeff;
    *hemisphere.y_mut() *= coeff;
    hemisphere += &v_tangent_stretched;

    vector!(3 [hemisphere.x() * alpha, hemisphere.y() * alpha, hemisphere.z()]).unit()
}
