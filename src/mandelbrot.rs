use num_complex::Complex;

fn escape_iter(number: Complex<i32>, iter: i32) -> i32 {
    const ESCAPE: i32 = 100;

    let mut curr = Complex::new(0, 0);

    for i in 0..iter {
        curr = curr.powi(2) + number;
        if curr.norm_sqr() > ESCAPE.pow(2) {
            return i;
        }
    }

    -1
}

fn calculate_mandelbrot(
    x_min: i32,
    y_min: i32,
    x_max: i32,
    y_max: i32,
    max_iter: i32,
) -> Vec<Vec<i32>> {

    let width = (x_max - x_min) as usize + 1;
    let height = (y_max - y_min) as usize + 1;

    println!(" --> {} {}", width, height);
    let mut ret_val = Vec::with_capacity(width);

    for x in x_min..=x_max {
        let idx_x = (x + x_max) as usize;
        ret_val.push(Vec::with_capacity(height));
        for y in y_min..=y_max {
            let _idx_y = (y + y_max) as usize;
            let ecape_value = escape_iter(Complex::new(x, y), max_iter);
            ret_val[idx_x].push(ecape_value);
        }
    }

    ret_val
}

fn render_mandelbrot() {
    let mandelbrot = calculate_mandelbrot(-10, -10, 10, 10, 1000);

    let max = mandelbrot.iter().map(|row| row.iter().max()).flatten().max().unwrap();

    let max_len = format!("{}", max).len();

    for row in mandelbrot {
        for cell in row {
            print!("{number:>width$}", number = cell, width = max_len);
        }
        println!();
    }
}

fn main() {
    render_mandelbrot()
}