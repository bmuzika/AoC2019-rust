use std::fs::File;
use std::io::{BufReader, BufRead, Error, Read};

fn main() {
    let mut file = match File::open("input.txt") {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");

    let fuel: i32 = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .filter(|s| s != "")
        .map(|s| s.parse::<i32>().unwrap())
        .map(|m| fuel_calculation(m))
        .sum();

    println!("{}", fuel);
}

fn fuel_calculation(mass: i32) -> i32 {
    let fuel = (mass/3) - 2;
    fuel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fuels_examples() {
        assert_eq!(fuel_calculation(12), 2);
        assert_eq!(fuel_calculation(14), 2);
        assert_eq!(fuel_calculation(1969), 654);
        assert_eq!(fuel_calculation(100756), 33583);
    }
}