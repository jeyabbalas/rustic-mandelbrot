use num::Complex;
use ndarray::Array;


const SIZE: u32 = 800;

fn f(z: Complex<f64>, c: Complex<f64>) -> Complex<f64> {
    z.powu(2) + c
}

pub fn run(max_iter: u32) {
    let array = Array::linspace(0., 1., 5);
    println!("{:?}", array);

    println!("{}", f(Complex::new(2.0, 0.0), Complex::new(0.1, 0.4)));

    let mut imgbuf = image::ImageBuffer::new(SIZE, SIZE);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        *pixel = image::Rgb([0_u8, 0_u8, 0_u8]);
    }

    imgbuf.save("temp.png").unwrap();
}