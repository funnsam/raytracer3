use raytracer3::{*, objects::*};
use smolmatrix::*;

const WIDTH: usize = 426;
const HEIGHT: usize = 240;

fn main() {
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
        settings: Settings::default(),
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

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let c = state.get_color(framing.clone(), x, y).get_rgb();
            data.extend(&c);
        }
    }

    writer.write_image_data(&data).unwrap();
}
