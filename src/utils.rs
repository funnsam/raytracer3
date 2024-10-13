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

pub fn get_basis(normal: &Vector<3>) -> Matrix<3, 3> {
    let t = 1.0 - normal[2].abs() > 0.0001;
    let normal = if t {
        normal.clone()
    } else {
        vector!(3 [normal[2], normal[0], normal[1]])
    };

    let tc = vector!(2 [normal[0], normal[1]]);
    let tc = tc.clone() * &tc;
    let nz_p1 = normal[2] + 1.0;
    let tc = vector!(3 [nz_p1 - tc[0], nz_p1 - tc[1], nz_p1]);

    let uu = vector!(3 [tc[0], tc[2], -normal[0]]);
    let vv = vector!(3 [tc[2], tc[1], -normal[1]]);

    if t {
        matrix!(3 x 3
            [uu[0], vv[0], normal[0]]
            [uu[1], vv[1], normal[1]]
            [uu[2], vv[2], normal[2]]
        )
    } else {
        matrix!(3 x 3
            [uu[1], vv[1], normal[1]]
            [uu[2], vv[2], normal[2]]
            [uu[0], vv[0], normal[0]]
        )
    }
}
