use num::Complex;
use std::error::Error;
use colorgrad;


const MAX_ITERATIONS: u32 = 100;
const RESOLUTION: u32 = 1024;
const RE_MIN: f64 = -2.0;
const RE_MAX: f64 = 1.0;
const IM_MIN: f64 = -1.5;
const IM_MAX: f64 = 1.5;

fn f(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z.powu(2) + c
}

fn calculate_escape(c: Complex<f64>) -> u32 {
    let mut z = Complex::new(0.0, 0.0);
    let mut n: u32 = 0;

    while (z.norm() <= 2.0) && (n < MAX_ITERATIONS) {
        z = f(z,c);
        n += 1;
    }

    n
}

fn escape_time() -> Result<(), Box<dyn Error>> {
    let re_range: f64 = RE_MAX - RE_MIN;
    let im_range: f64 = IM_MAX - IM_MIN;

    let mut imgbuf = image::ImageBuffer::new(RESOLUTION, RESOLUTION);
    let grad = colorgrad::CustomGradient::new()
                                         .html_colors(&["white", 
                                                        "lightblue", 
                                                        "darkblue",
                                                        "black"])
                                         .domain(&[0., MAX_ITERATIONS as f64])
                                         .build()?;

    for x in 0..RESOLUTION {
        for y in 0..RESOLUTION {
            let cx = re_range*(x as f64/RESOLUTION as f64) + RE_MIN;
            let cy = im_range*(y as f64/RESOLUTION as f64) + IM_MIN;

            let n_iter = calculate_escape(Complex::new(cx, cy));

            let (r, g, b, _a) = grad.at(n_iter as f64).rgba_u8();
            let pixel = imgbuf.get_pixel_mut(x, y);
            *pixel = image::Rgb([r, g, b]);
        }
    }

    imgbuf.save("mandelbrot.png")?;

    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    escape_time()?;

    Ok(())
}