use std::ops::Deref;

fn main() {
    println!("Hello, world! It's day2!!!!");
}

enum IntcodeParseError {
    Halt,
    InvalidOpcode,
}

fn process_input(input: &str) -> String {
    // creates the vector from the program code
    let input_vector = input.split(',')
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let mut cursor = 0;
    let mut code_to_execute = input_vector.clone();
    let slice_to_work_on = code_to_execute[cursor..cursor+4];

    let mut position = code_to_execute.iter();

    while let Some(opcode) = position.next() {
        let calc: Result<u64, IntcodeParseError> = match opcode {
            1 => {
                let reg1: usize = *(position.next().expect("Invalid index")) as usize;
                let reg2: usize = *(position.next().expect("Invalid index")) as usize;
                Ok(code_to_execute[reg1]+code_to_execute[reg2])
            }, // Add opcode
            2 => {
                let reg1: usize = *(position.next().expect("Invalid index")) as usize;
                let reg2: usize = *(position.next().expect("Invalid index")) as usize;
                Ok(code_to_execute[reg1]*code_to_execute[reg2])
            },// Mult opcode
            99 => Err(IntcodeParseError::Halt),// Halt
            _ => Err(IntcodeParseError::InvalidOpcode),// Err
        };

        let value = match calc {
            Ok(calc) => calc,
            Err(IntcodeParseError::Halt) => break,
            Err(IntcodeParseError::InvalidOpcode) => panic!("Invalid opcode"),
        };

        let out_reg: usize = *(position.next().expect("Invalid index")) as usize;
        code_to_execute[out_reg] = value;
    }

    // creates a String for the output, separated by commas
    let result = input_vector.into_iter()
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