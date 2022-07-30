use std::env;
use std::num::ParseIntError;

use image::{self, Rgba, GenericImage};
use image::GenericImageView;

fn main() {

    let args : Vec<String> = env::args().collect();

    check_number_of_parameters(args.len());

    let color_to_replace = argument_to_color(&args[2]);
    let new_color = argument_to_color(&args[3]);
    
    let mut image = image::open(&args[1]).expect("File not found!");
    let (width, height) = image.dimensions();

    for y in 0..height {
        for x in 0..width {
             if image.get_pixel(x, y) == color_to_replace
             {
                image.put_pixel(x, y, new_color);
             }
        }
    }

    image.save(&args[1]);
}

pub fn argument_to_color(s: &str) -> Rgba<u8> {
    let result: Result<Vec<u8>, ParseIntError> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect();
    
    let color = result.unwrap();

    Rgba([color[0], color[1], color[2], 255])
}

fn check_number_of_parameters( number_of_args : usize) {
    if number_of_args !=  4 {
        panic!("Invalid arguments number: {}.", number_of_args);
    }
}