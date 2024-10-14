pub fn pdf(d: f32, g1_n_dot_v: f32, n_dot_l: f32) -> f32 {
    (d * g1_n_dot_v / (4.0 * n_dot_l)).max(0.001)
}

pub fn g1(alpha: f32, x_dot_n: f32) -> f32 {
    2.0 / (1.0 + (1.0 + alpha * alpha * ((1.0 - x_dot_n * x_dot_n) / (x_dot_n * x_dot_n))).sqrt())
}

pub fn d(alpha: f32, h_dot_n: f32) -> f32 {
    let sq = alpha / (h_dot_n * h_dot_n * (alpha * alpha - 1.0) + 1.0);
    core::f32::consts::FRAC_1_PI * sq * sq
}
