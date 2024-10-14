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

pub fn get_basis(mut nor: Vector<3>) -> Matrix<3, 3> {
    let t = 1.0 - nor.z().abs() > 0.00001;
    if !t {
        nor = vector_swap!(&nor, 2 0 1);
    }

    let xy = vector!(2 [nor.x(), nor.y()]);
    let xy = -xy.clone() * &xy + 1.0 + nor.z();
    let tc = vector!(3 [xy.x(), xy.y(), -nor.x() * nor.y()]) / (1.0 + nor.z());
    let uu = vector!(3 [tc.x(), tc.z(), -nor.x() ]);
    let vv = vector!(3 [tc.z(), tc.y(), -nor.y() ]);

    if t {
        matrix!(3 x 3
            [uu.x(), vv.x(), nor.x()]
            [uu.y(), vv.y(), nor.y()]
            [uu.z(), vv.z(), nor.z()]
        )
    } else {
        matrix!(3 x 3
            [uu.y(), vv.y(), nor.y()]
            [uu.z(), vv.z(), nor.z()]
            [uu.x(), vv.x(), nor.x()]
        )
    }
}

pub fn reflect(v: Vector<3>, n: Vector<3>) -> Vector<3> {
    let ndv = n.dot(&v);
    v - &(n * 2.0 * ndv)
}
