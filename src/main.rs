use raytracer3::{*, objects::*};
use smolmatrix::vector;

const WIDTH: usize = 320;
const HEIGHT: usize = 256;

fn main() {
    println!("P3\n{WIDTH} {HEIGHT}\n255\n");


    let mut state = State {
        camera: Camera::new(WIDTH, HEIGHT),
        scene: Scene {
            world: list::List {
                objects: vec![
                    Box::new(objects::sphere::Sphere {
                        center: vector!(3 [0.0, 0.0, -1.0]),
                        radius: 0.5,
                    }),
                    Box::new(objects::sphere::Sphere {
                        center: vector!(3 [0.0, -100.5, -1.0]),
                        radius: 100.0,
                    }),
                ],
            },
        },
    };

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = state.get_color(x, y);
            print!("{c}");
        }
    }
}
