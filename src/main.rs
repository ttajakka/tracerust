use tracerust::color::Color;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let mut stdout = std::io::stdout();
    print!("P3\n{} {}\n255\n", image_width, image_height);

    for i in 0..image_height {
        eprint!("\rScanlines remaining: {} ", (image_height - i));
        for j in 0..image_width {
            let color = Color::new(
                (i as f64) / ((image_width - 1) as f64),
                (j as f64) / ((image_height - 1) as f64),
                0.0_f64,
            );
            color.write_io(&mut stdout);
        }
    }
    eprint!("\rDone.                   \n");
}
