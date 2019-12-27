use std::fs::{File};
use std::io::{Read};

fn main() {
    let mut file = match File::open("input.txt") {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");

    let image = AoCImageLayers::new(file_contents, 6, 25);

    let num_layers = image.clone().get_layers();
    println!("Number of layers: {}", num_layers);

    let mut zeros_per_layer = Vec::new();

    for layer in 0..num_layers {
        let layer_vec = image.clone().output_layer_digits(layer);

        let mut num_zeros = 0;
        for char in layer_vec.into_iter() {
            if char == 0 {
                num_zeros += 1;
            }
        }

        zeros_per_layer.push((layer, num_zeros));
    }

    let mut layer_min_zeros = 0;
    let mut min_zeros = 9999;
    for layer in zeros_per_layer {
        if layer.1 < min_zeros {
            min_zeros = layer.1;
            layer_min_zeros = layer.0;
        }
    }
    let best_layer = image.clone().output_layer_digits(layer_min_zeros);
    let mut num_ones = 0;
    let mut num_twos = 0;

    for val in best_layer {
        if val == 1 {
            num_ones += 1;
        }

        if val == 2 {
            num_twos += 1;
        }
    }

    let result = num_ones * num_twos;

    println!("Result = {}", result);


    let mut result_vec = vec![2; 6*25];

    for layer in 0..num_layers {
        let test_layer = image.clone().output_layer_digits(layer);
        for val in 0..6*25 {
            if test_layer[val] != 2 && result_vec[val] == 2 {
                result_vec[val] = test_layer[val];
            }
        }
    }
    let row1 = &result_vec[0..24];
    let row2 = &result_vec[25..25*2-1];
    let row3 = &result_vec[25*2..25*3-1];
    let row4 = &result_vec[25*3..25*4-1];
    let row5 = &result_vec[25*4..25*5-1];
    let row6 = &result_vec[25*5..25*6-1];
    println!("{:?}", row1);
    println!("{:?}", row2);
    println!("{:?}", row3);
    println!("{:?}", row4);
    println!("{:?}", row5);
    println!("{:?}", row6);

}
/*
The image you received is 25 pixels wide and 6 pixels tall.

To make sure the image wasn't corrupted during transmission, the Elves would like you to find the layer that contains the fewest 0 digits.
On that layer, what is the number of 1 digits multiplied by the number of 2 digits?

*/
#[derive(Default, Clone)]
struct AoCImageLayers {
    height: u32,
    width: u32,
    raw_buffer: Vec<u32>,
    number_of_layers: usize,
}

impl AoCImageLayers {
    fn get_layers(self) -> usize {
        self.number_of_layers
    }

    fn output_layer_digits(self, layer_index: usize) -> Vec<u32> {
        let num_vals_to_skip = layer_index * (self.height * self.width) as usize;
        let output_vector = self.raw_buffer[(num_vals_to_skip) .. (num_vals_to_skip+(self.height*self.width) as usize)].to_vec();
        output_vector
    }

    fn new(image_string: String, pix_height: u32, pix_width: u32) -> AoCImageLayers {
        let digit_vec = image_string.trim().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<u32>>();

        let width = pix_width;
        let height = pix_height;

        let number_of_layers = digit_vec.len()/ (pix_height * pix_width) as usize;

        AoCImageLayers{ height: height, width: width, number_of_layers: number_of_layers, raw_buffer: digit_vec }
    }
}
enum Day8Error {
    WrongSize,
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn image_test() {
        // some assert here: input string "123456789012123456789012" This image is 3x2
        /* Output format:
        Layer 1: 123
                 456

        Layer 2: 789
                 012
         */

        //
    }
}
