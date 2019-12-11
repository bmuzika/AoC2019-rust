use std::fs::{File, read_to_string};
use std::io::Read;
use std::char::from_digit;

fn main() {
    let mut file = match File::open("input.txt") {
        Ok(file) => file,
        Err(_) => panic!("no file"),
    };

    let mut file_contents = String::new();
    file.read_to_string(&mut file_contents)
        .ok()
        .expect("failed to read");

    let mut noun = 0;
    let mut verb = 0;

    'outer: for noun in (0 .. 100) {
        for verb in (0..100) {
            let mut working_input_chars = file_contents.clone().trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>();
            working_input_chars[1] = noun.to_string();
            working_input_chars[2] = verb.to_string();

            let working_input = working_input_chars.into_iter().collect::<Vec<String>>().join(",");

            let output: String = process_input(working_input.as_str());

            let result = output.trim().split(',').map(|s| s.to_string()).next().unwrap().parse::<u32>().unwrap();
            if result == 19690720 {
                println!("noun: {}, verb: {}",noun, verb );
                break 'outer;
            }
        }
    }
}

enum IntcodeParseError {
    Halt,
    InvalidOpcode,
}

fn process_input(input: &str) -> String {
    // creates the vector from the program code
    let input_vector = input.trim()
        .split(',')
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut cursor = 0;
    let mut code_to_execute = input_vector.clone();
    while (cursor < code_to_execute.len() && code_to_execute[cursor] != 99) {
        let mut slice_to_work_on = [0,0,0,0];
        slice_to_work_on.clone_from_slice(&code_to_execute[cursor..cursor + 4]);

        let opcode = slice_to_work_on[0];
        let calc: Result<u64, IntcodeParseError> = match opcode {
            1 => {
                let reg1: usize = slice_to_work_on[1] as usize;
                let reg2: usize = slice_to_work_on[2] as usize;
                Ok(code_to_execute[reg1] + code_to_execute[reg2])
            }, // Add opcode
            2 => {
                let reg1: usize = slice_to_work_on[1] as usize;
                let reg2: usize = slice_to_work_on[2] as usize;
                Ok(code_to_execute[reg1] * code_to_execute[reg2])
            },// Mult opcode
            99 => Err(IntcodeParseError::Halt),// Halt
            _ => Err(IntcodeParseError::InvalidOpcode),// Err
        };

        let value = match calc {
            Ok(calc) => calc,
            Err(IntcodeParseError::Halt) => break,
            Err(IntcodeParseError::InvalidOpcode) => panic!("Invalid opcode"),
        };

        let out_reg: usize = slice_to_work_on[3] as usize;
        code_to_execute[out_reg] = value;
        cursor += 4;
    }

    // creates a String for the output, separated by commas
    let result = code_to_execute.into_iter()
        .map(|int| int.to_string())
        .collect::<Vec<String>>()
        .join(",");

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn process_tests() {
        assert_eq!(process_input("1,0,0,0,99"), "2,0,0,0,99");
        assert_eq!(process_input("2,3,0,3,99"), "2,3,0,6,99");
        assert_eq!(process_input("2,4,4,5,99,0"), "2,4,4,5,99,9801");
        assert_eq!(process_input("1,1,1,4,99,5,6,0,99"), "30,1,1,4,2,5,6,0,99");
    }
}