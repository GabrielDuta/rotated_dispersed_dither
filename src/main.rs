fn main() {
    let matrix = generate_bayer_matrix(8);
    print_matrix(&matrix);
    println!();
    println!();
    let rot = rotate_matrix(&matrix, 36.87);
    print_matrix(&rot);

    println!();
    println!();
    let rot = rotate_matrix2(&matrix, 36.87);
    print_matrix(&rot);
}

/*
fn main() {
    let mut img = image::open("grayscale2.jpg").unwrap().into_luma8();
    let bayer_matrix = generate_bayer_matrix(8);
    let rotated_matrix = rotate_matrix(&bayer_matrix, std::f64::consts::PI / 4.0);
    apply_dither(&mut img, &rotated_matrix);
    //apply_dither(&mut img, &bayer_matrix);
    img.save("output.png").unwrap();
}
*/


/** 
*   Generate a Bayer matrix of size n x n
*/
fn generate_bayer_matrix(size: usize) -> Vec<Vec<u8>> {
    if size % 2 != 0 {
        panic!("Size must be a power of 2");
    }

    let mut matrix = vec![vec![0; size]; size];
    let mut n = 2;
    while n <= size {
        for y in 0..n/2 {
            for x in 0..n/2 {
                let value = matrix[y][x];
                matrix[y][x] = 4 * value;
                matrix[y][x + n/2] = 4 * value + 2;
                matrix[y + n/2][x] = 4 * value + 3;
                matrix[y + n/2][x + n/2] = 4 * value + 1;
            }
        }
        n *= 2;
    }
    matrix
}

fn rotate_matrix(matrix: &Vec<Vec<u8>>, angle: f64) -> Vec<Vec<u8>> {
    let size = matrix.len();
    let mut rotated = vec![vec![0; size * 3]; size * 3];

    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    for y in 0..size {
        for x in 0..size {
            // multiply by 2 or add 10
            let new_x = (x as f64 * cos_theta - y as f64 * sin_theta).round() as isize;
            let new_y = (x as f64 * sin_theta + y as f64 * cos_theta).round() as isize;

            if new_x >= 0 && new_x < size as isize && new_y >= 0 && new_y < size as isize {
                rotated[new_y as usize][new_x as usize] = matrix[y][x];
            }
        }
    }

    rotated
}

fn rotate_matrix2(matrix: &Vec<Vec<u8>>, angle: f64) -> Vec<Vec<u8>> {
    let size = matrix.len();
    let mut rotated = vec![vec![0; size * 3]; size * 3];

    let cos_theta = angle.cos();
    let sin_theta = angle.sin();
    let c = 5.0; let a = 4.0; let b = 3.0;

    for y in 0..size {
        for x in 0..size {
            let new_x = (a/c * x as f64 - b/c * y as f64).round() as isize;
            let new_y = (b/c * x as f64 - a/c * y as f64).round() as isize;

            if new_x >= 0 && new_x < size as isize && new_y >= 0 && new_y < size as isize {
                rotated[new_y as usize][new_x as usize] = matrix[y][x];
            }
        }
    }

    rotated
}


fn print_matrix(matrix: &Vec<Vec<u8>>) {
    for row in matrix {
        for value in row {
            print!("{:3} ", value);
        }
        println!();
    }
}

use image::{GrayImage, Luma};

fn apply_dither(image: &mut GrayImage, dither: &Vec<Vec<u8>>) {
    let (width, height) = image.dimensions();
    println!("Image dimension {} {}", width, height);
    let size = dither.len();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y)[0];
            let threshold = dither[(y as usize) % size][(x as usize) % size];

            let new_pixel = if pixel as u16 > threshold as u16 * 255 / (size * size) as u16 {
                255
            } else {
                0
            };

            image.put_pixel(x, y, Luma([new_pixel]));
        }
    }
}

