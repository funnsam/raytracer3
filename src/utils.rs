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
