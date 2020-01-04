extern crate intcode_computer_lib;

use std::fs::{File};
use std::io::{Read};
use std::thread;

use intcode_computer_lib::IntcodeComputer::*;

fn main() {

    let mut file_contents = read_file("../day9/input.txt");

    let mut working_input_chars = file_contents.clone().trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>();

    let working_input = working_input_chars.into_iter().collect::<Vec<String>>().join(",");

    let mut max_thrust = 0;

    let attempt = run_amplifiers(working_input.clone(), "");

    if attempt > max_thrust {
        max_thrust = attempt;
    }
    println!("result = {}", attempt);
}


fn run_amplifiers(program: String, phase_order: &str) -> i64 {
    let (tx0, rx0) = shared_channel();
    let (tx1, mut rx1) = shared_channel();

    let mut children = Vec::new();


    let child = thread::spawn(move || {
        let (_, val) = process_input(program.as_str(), rx0, tx1, 0 as i64);

    });

    tx0.send(2);
    children.push(child);

    let child2 = thread::spawn(move || {
        loop {
            let val = rx1.next();
            match val {
                Some(_) => print!("{},", val.unwrap()),
                None => break,
            }
        }
    });

    let mut i = 0;
    for child in children {
        child.join().expect(format!("Child panic, {}", i).as_str());
        i += 1;
    }

    let mut ret_val = 0;
    ret_val
}

fn read_file(path: &str) -> String {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");

    file_contents
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_test() {
        //assert_eq!(run_amplifiers("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string(), "4,3,2,1,0"), 43210);
        assert_eq!(run_amplifiers("33,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string(), "9,8,7,6,5"), 139629729);
    }
}


