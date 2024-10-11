use raytracer3::{*, objects::*};
use smolmatrix::*;

const WIDTH: usize = HEIGHT * 16 / 9;
const HEIGHT: usize = 480;

fn main() {
    let sphere_1 = objects::sphere::Sphere {
        center: vector!(3 [-1.0, 0.0, -1.0]),
        radius: 0.5,
        bsdf: &materials::Bsdf {
            base_color: color::Color(vector!(3 [0.5, 0.2, 0.2])),
            metallic: 0.33,
            roughness: 0.0,

            emission: materials::Emission {
                color: color::Color(Vector::new_zeroed()),
                strength: 0.0,
            },
        },
    };
    let sphere_2 = objects::sphere::Sphere {
        center: vector!(3 [0.0, 0.0, -1.0]),
        radius: 0.5,
        bsdf: &materials::Bsdf {
            base_color: color::Color(vector!(3 [0.2, 0.5, 0.2])),
            metallic: 0.66,
            roughness: 0.0,

            emission: materials::Emission {
                color: color::Color(Vector::new_zeroed()),
                strength: 0.0,
            },
        },
    };
    let sphere_3 = objects::sphere::Sphere {
        center: vector!(3 [1.0, 0.0, -1.0]),
        radius: 0.5,
        bsdf: &materials::Bsdf {
            base_color: color::Color(vector!(3 [0.2, 0.2, 0.5])),
            metallic: 1.0,
            roughness: 0.0,

            emission: materials::Emission {
                color: color::Color(Vector::new_zeroed()),
                strength: 0.0,
            },
        },
    };
    let plane = objects::plane::Plane {
        origin: vector!(3 [0.0, -0.5, -1.0]),
        normal: vector!(3 [0.0, -1.0, 0.0]),
        bsdf: &materials::Bsdf {
            base_color: color::Color(vector!(3 [0.2, 0.5, 0.0])),
            metallic: 0.0,
            roughness: 0.0,

            emission: materials::Emission {
                color: color::Color(Vector::new_zeroed()),
                strength: 0.0,
            },
        },
    };

    let state = State {
        camera: Camera::new(WIDTH, HEIGHT),
        scene: Scene {
            world: list::List {
                objects: vec![
                    &sphere_1,
                    &sphere_2,
                    &sphere_3,
                    &plane,
                ],
            },
        },
        settings: Settings::default().rays_per_px(32),
    };

    let path = std::path::Path::new("image.png");
    let file = std::fs::File::create(path).unwrap();
    let w = std::io::BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, WIDTH as _, HEIGHT as _);
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455));
    encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2));
    let source_chromaticities = png::SourceChromaticities::new(
        (0.31270, 0.32900),
        (0.64000, 0.33000),
        (0.30000, 0.60000),
        (0.15000, 0.06000)
    );
    encoder.set_source_chromaticities(source_chromaticities);
    let mut writer = encoder.write_header().unwrap();

    let mut data = Vec::new();
    let framing = state.camera.get_framing_info();

    println!("0.00%");
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = state.get_color(framing.clone(), x, y).get_rgb();
            data.extend(&c);
        }
        println!("\x1b[1A{:.02}%", (y + 1) as f32 / HEIGHT as f32 * 100.0);
    }

    writer.write_image_data(&data).unwrap();
}
