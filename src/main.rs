
/* 
fn main() {
    let matrix = generate_bayer_matrix(4);
    print_matrix(&matrix);

    println!();
    let completed = complete_matrix(&matrix, matrix.len() as f64 * 5.0, matrix.len() as f64 * 5.0);
    println!("Completed matrix:");
    print_matrix(&completed);

    // Triangle sides:
    let a = 4.0;
    let b = 3.0;

    println!();
    println!();
    let rot = rotate_matrix(&completed, a, b);
    println!("Rotated matrix:");
    print_matrix(&rot);

    println!();
    println!();
    let square = extract_square(&rot, 4.0, 20.0);
    println!("Extracted square:");
    print_matrix(&square);
}*/

fn main() {
    let bayer_sizes = vec![2, 4, 8, 16];
    let pairs = vec![
        (3, 4),
        (4, 3),
        (5, 12),
        (12, 5),
        (7, 24),
        (8, 15),
        (15, 8),
        (9, 40),
        (20, 21)
    ];

    let image_path = "Screenshot from 2025-04-03 15-43-23.png";

    for size in bayer_sizes {
        for pair in &pairs {
            work_on_image(image_path, size, 5.0, pair.0 as f64, pair.1 as f64, size as f64 * 5.0);
        }
    }
}

fn work_on_image(image_path: &str, bayer_size: usize, mult_for_completion: f64, side_a: f64, side_b: f64, width_of_squared: f64) {
    let mut img = image::open(image_path).unwrap().into_luma8();

    let bayer_matrix = generate_bayer_matrix(bayer_size);
    println!("Bayer matrix:");
    print_matrix(&bayer_matrix);
    println!();

    let completed = complete_matrix(&bayer_matrix, bayer_matrix.len() as f64 * mult_for_completion, bayer_matrix.len() as f64 * mult_for_completion);
    println!("Completed matrix:");
    print_matrix(&completed);
    println!();
    
    let rotated_matrix = rotate_matrix(&completed, side_a, side_b);
    println!("Rotated matrix:");
    print_matrix(&rotated_matrix);
    println!();

    let square = extract_square(&rotated_matrix, bayer_size as f64, width_of_squared);
    println!("Extracted square:");
    print_matrix(&square);
    println!();

    apply_dither(&mut img, &square, (bayer_size.pow(2 as u32) - 1) as usize);
    //apply_dither(&mut img, &square, bayer_size as usize);
    //apply_dither(&mut img, &bayer_matrix, (bayer_size.pow(2 as u32) - 1) as usize);
    img.save(format!("output{}_a{}_b{}.png", bayer_size, side_a, side_b)).unwrap();
}

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

    let factor = 255.0 / ((size * size) as f32 - 1.0);
    for y in 0..size {
        for x in 0..size {
            matrix[y][x] = (matrix[y][x] as f32 * factor) as u8;
        }
    }

    matrix
}

/**
 * Complete a matrix by mirroring it cover the whole image
 *
 * Parameters:
 * - matrix: The matrix to complete
 * - width: The width of the image
 * - height: The height of the image
*/
fn complete_matrix(matrix: &Vec<Vec<u8>>, width: f64, height: f64) -> Vec<Vec<u8>> {
    let size = matrix.len();
    let row_mult = (width / size as f64).ceil() as usize;
    let col_mult = (height / size as f64).ceil() as usize;

    let mut completed = vec![vec![0; size * row_mult]; size * col_mult];

    for cols in 0..row_mult {
        for rows in 0..col_mult {
            for y in 0..size {
                for x in 0..size {
                    completed[y + rows * size][x + cols * size] = matrix[y][x];
                }
            }
        }
    }

    completed
}

/**
* Rotate a matrix by a given angle
*/
fn rotate_matrix_angle(matrix: &Vec<Vec<u8>>, angle: f64) -> Vec<Vec<u8>> {
    let size = matrix.len();
    let mut rotated = vec![vec![0; size]; size];

    let cos_theta = angle.cos();
    let sin_theta = angle.sin();

    for y in 0..size {
        for x in 0..size {

            let half = matrix[0].len() as f64 / 2.0;
            let y_coord = size - y - 1;
            let xi = x as f64 - half;
            let yi = y_coord as f64 - half;

            let mut new_x = (xi as f64 * cos_theta - yi as f64 * sin_theta).round() as isize;
            let mut new_y = (xi as f64 * sin_theta + yi as f64 * cos_theta).round() as isize;
            new_x += half as isize;
            new_y += half as isize;
            new_y = size as isize - new_y - 1;

            if new_x >= 0 && new_x < size as isize && new_y >= 0 && new_y < size as isize {
                rotated[new_y as usize][new_x as usize] = matrix[y][x];
            }
        }
    }

    rotated
}


/**
* Rotate a matrix by a following a triangle hypotenuse
* This method allows for a precise one-to-one mapping of the pixels
* 
* Parameters:
* - matrix: The matrix to rotate
* - a: The legth of the first leg of the triangle
* - b: The legth of the second leg of the triangle
* - c: The length of the hypotenuse
*/
fn rotate_matrix(matrix: &Vec<Vec<u8>>, a: f64, b: f64) -> Vec<Vec<u8>> {
    let width = matrix[0].len();
    let height = matrix.len();
    let mut rotated = vec![vec![0; width]; height];
    let c = ((a * a + b * b) as f64).sqrt();
    
    let half_x = matrix[0].len() as f64 / 2.0;
    let half_y = matrix.len() as f64 / 2.0;

    for y in 0..height {
        for x in 0..width {

            let y_coord = height - y - 1;
            let xi = x as f64 - half_x;
            let yi = y_coord as f64 - half_y;

            let mut new_x = (a/c * xi as f64 - b/c * yi as f64).round() as isize;
            let mut new_y = (b/c * xi as f64 + a/c * yi as f64).round() as isize;
            new_x += half_x as isize;
            new_y += half_y as isize;
            new_y = height as isize - new_y - 1;

            if new_x >= 0 && new_x < width as isize && new_y >= 0 && new_y < height as isize {
                rotated[new_y as usize][new_x as usize] = matrix[y][x];
            }
        }
    }

    rotated
}

fn extract_square(matrix: &Vec<Vec<u8>>, height: f64, width: f64) -> Vec<Vec<u8>> {
    let start_x = ((matrix[0].len() as f64 / 2.0).floor() - (width / 2.0)) as usize;
    let start_y = ((matrix.len() as f64 / 2.0).floor() - (height / 2.0)) as usize;

    let height = height as usize;
    let width = width as usize;

    let mut square = vec![vec![0; width]; height];

    for j in 0..height {
        for i in 0..width {
            square[j][i] = matrix[start_y + j][start_x + i];
        }
    }
    square
}

fn print_matrix(matrix: &Vec<Vec<u8>>) {
    for row in matrix {
        for value in row {
            print!("{:3} ", value);
        }
        println!();
    }
}

use image::{GrayImage, Luma, Pixel};

fn apply_dither(image: &mut GrayImage, dither: &Vec<Vec<u8>>, dither_n: usize) {
    let (width, height) = image.dimensions();
    println!("Image dimension {} {}", width, height);
    let dither_height = dither.len();
    let dither_width = dither[0].len();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y)[0];
            //let threshold = dither[(y as usize) % size][(x as usize) % size];
            //let threshold = dither[y as usize][x as usize] ;

            let threshold = dither[(y as usize) % dither_height][(x as usize) % dither_width] as u8;

            let value = (pixel as f64 / 256.0 * dither_n as f64).floor() as u8;
            let new_pixel = if value > threshold {
                255
            } else {
                0
            };

            let new_value = if pixel > threshold { 255 } else { 0 };

            //image.put_pixel(x, y, Luma([new_pixel]));
            image.put_pixel(x, y, Luma([new_value]));
        }
    }
}
