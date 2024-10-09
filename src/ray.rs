use smolmatrix::*;

pub struct Ray {
    direction: Vector<3>,
    origin: Vector<3>,
}

impl Ray {
    pub fn new(direction: Vector<3>, origin: Vector<3>) -> Self {
        Self { direction, origin }
    }

    pub fn new_normalized(direction: Vector<3>, origin: Vector<3>) -> Self {
        Self { direction: direction.unit(), origin }
    }

    pub fn at(&self, t: f32) -> Vector<3> {
        (self.direction.clone() * t) + &self.origin
    }

    pub fn direction(&self) -> &Vector<3> { &self.direction }
    pub fn origin(&self) -> &Vector<3> { &self.origin }
}
