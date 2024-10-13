use smolmatrix::*;

pub fn random_unit_vector() -> Vector<3> {
    loop {
        let x = fastrand::f32() * 2.0 - 1.0;
        let y = fastrand::f32() * 2.0 - 1.0;
        let z = fastrand::f32() * 2.0 - 1.0;
        let v = vector!(3 [x, y, z]);

        if v.length_squared() <= 1.0 {
            return v.unit();
        }
    }
}

pub fn random_hemisphere_vector(n: &Vector<3>) -> Vector<3> {
    let p = random_unit_vector();
    if p.dot(n) > 0.0 { p } else { -p }
}

pub fn make_orthonormals(n: &Vector<3>) -> (Vector<3>, Vector<3>) {
    let mut a = if n[0] != n[1] || n[0] != n[2] {
        vector!(3 [n[2] - n[1], n[0] - n[2], n[1] - n[0]])
    } else {
        vector!(3 [n[2] - n[1], n[0] + n[2], -n[1] - n[0]])
    };
    a = a.unit();
    let b = n.cross(&a);
    (a, b)
}
