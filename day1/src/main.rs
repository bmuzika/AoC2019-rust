use std::fs::File;
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

    let fuel: i32 = file_contents.split("\n")
        .map(|s: &str| s.to_string())
        .filter(|s| s != "")
        .map(|s| s.parse::<i32>().unwrap())
        .map(|m| fuel_calculation_part2(m))
        .sum();

    println!("{}", fuel);
}

fn fuel_calculation(mass: i32) -> i32 {
    let fuel = (mass/3) - 2;
    fuel
}

fn fuel_calculation_part2(mass: i32) -> i32 {
    let mut new_fuel: i32 = fuel_calculation(mass);
    let mut total: i32 = 0;

    while new_fuel > 0 {
        total += new_fuel;
        let old_fuel = new_fuel;
        new_fuel = fuel_calculation(old_fuel);
    }

    total
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

    #[test]
    fn fuels_part2_examples() {
        assert_eq!(fuel_calculation_part2(12), 2);
        assert_eq!(fuel_calculation_part2(14), 2);
        assert_eq!(fuel_calculation_part2(1969), 966);
        assert_eq!(fuel_calculation_part2(100756), 50346);
    }
}