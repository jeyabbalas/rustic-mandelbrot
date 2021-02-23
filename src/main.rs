fn main() {
    if let Err(e) = rustic_mandelbrot::run() {
        eprintln!("Error generating mandelbrot plot: {}", e);
    }
}
