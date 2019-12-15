use std::fs::{File, read_to_string};
use std::io::{Read, Write};
use std::io;
use std::char::from_digit;
use crate::IntcodeParseError::InvalidOpcode;

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

    let mut working_input_chars = file_contents.clone().trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>();
    //working_input_chars[1] = noun.to_string();
    //working_input_chars[2] = verb.to_string();

    let working_input = working_input_chars.into_iter().collect::<Vec<String>>().join(",");

    let output: String = process_input(working_input.as_str());

    let result = output.trim().split(',').map(|s| s.to_string()).next().unwrap().parse::<i32>().unwrap();
    if result == 19690720 {
        println!("noun: {}, verb: {}",noun, verb );
    }
}

#[derive(Debug)]
enum IntcodeParseError {
    InvalidOpcode,
    InvalidParamMode,
}

enum IntcodeOpcode {
    Add, // 01
    Multiply, // 02
    ReadInteger, // 03
    PrintOutput, // 04
    JumpIfTrue, // 05
    JumpIfFalse, // 06
    LessThan, // 07
    Equals, // 08
    Halt, // 99

    Dummy, // Not a normal opcode
}

#[derive(Debug)]
enum ParameterMode{
    Immediate,
    Position,
    NoParam,
}

struct ParsedOpcodes {
    opcode: IntcodeOpcode,
    parameter1_mode: ParameterMode,
    parameter2_mode: ParameterMode,
    parameter3_mode: ParameterMode,
}

fn process_input(input: &str) -> String {
    // creates the vector from the program code
    let input_vector = input.trim()
        .split(',')
        .map(|s| s.to_string())
        .filter(|s| s != "")
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    let mut cursor = 0;
    let mut code_to_execute = input_vector.clone();
    while cursor < code_to_execute.len() && code_to_execute[cursor] != 99 {
        let mut slice_length = 0;
        let full_opcode = code_to_execute[cursor];
        let opcode_num = full_opcode % 100;
        let (mode1, mode2, mode3) = parse_parameter_mode(full_opcode/100).expect("Bad parameter mode!");

        let opcode: Result<IntcodeOpcode, IntcodeParseError> = match opcode_num {
            1 => {
                slice_length = 4;
                Ok(IntcodeOpcode::Add)
            }, // Add opcode

            2 => {
                slice_length = 4;
                Ok(IntcodeOpcode::Multiply)
            },// Mult opcode

            3 => {
                slice_length = 2;
                Ok(IntcodeOpcode::ReadInteger)
            },

            4 => {
                slice_length = 2;
                Ok(IntcodeOpcode::PrintOutput)
            },

            5 => {
                slice_length = 3;
                Ok(IntcodeOpcode::JumpIfTrue)
            },

            6 => {
                slice_length = 3;
                Ok(IntcodeOpcode::JumpIfFalse)
            },

            7 => {
                slice_length = 4;
                Ok(IntcodeOpcode::LessThan)
            },

            8 => {
                slice_length = 4;
                Ok(IntcodeOpcode::Equals)
            },

            99 => Ok(IntcodeOpcode::Halt),// Halt

            _ => Err(IntcodeParseError::InvalidOpcode),// Err
        };

        let parsed_opcode = ParsedOpcodes{
            opcode: opcode.expect("Bad opcode"),
            parameter1_mode: mode1,
            parameter2_mode: mode2,
            parameter3_mode: mode3,
        };

        let slice_to_work_on = &code_to_execute[cursor..cursor + slice_length];

        match parsed_opcode.opcode {
            IntcodeOpcode::Add => {
                let v1 = match parsed_opcode.parameter1_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[1] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[1],

                    _ => panic!("There should be a parameter mode for V1"),
                };

                let v2 = match parsed_opcode.parameter2_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[2] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[2],

                    _ => panic!("There should be a parameter mode for V2"),
                };



                let value = v1 + v2;
                let out_reg: usize = slice_to_work_on[3] as usize;
                code_to_execute[out_reg] = value;

                if out_reg != cursor {
                    cursor += 4;
                }
            },

            IntcodeOpcode::Multiply => {
                let v1 = match parsed_opcode.parameter1_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[1] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[1],

                    _ => panic!("There should be a parameter mode for V1"),
                };

                let v2 = match parsed_opcode.parameter2_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[2] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[2],

                    _ => panic!("There should be a parameter mode for V2"),
                };

                let value = v1 * v2;
                let out_reg: usize = slice_to_work_on[3] as usize;
                code_to_execute[out_reg] = value;

                if out_reg != cursor {
                    cursor += 4;
                }
            },

            IntcodeOpcode::ReadInteger => {
                let mut input_value = String::new();

                print!("Enter a single integer: ");
                io::stdout().flush();
                io::stdin().read_line(&mut input_value).ok().expect("Invalid input. Crashing now. Goodbye!\n");
                print!("\n");

                let addr: usize = slice_to_work_on[1] as usize;
                let value_to_save = input_value.trim().parse::<i64>().expect("Invalid integer. Goodbye!\n");

                code_to_execute[addr] = value_to_save;

                if addr != cursor {
                    cursor += 2;
                }
            },

            IntcodeOpcode::PrintOutput => {
                let addr: usize = slice_to_work_on[1] as usize;
                cursor += 2;

                println!("Value at addr {} is {}", addr, code_to_execute[addr]);
            },

            IntcodeOpcode::JumpIfTrue => {
                // If param1 != 0, cursor = param2
                let v1 = match parsed_opcode.parameter1_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[1] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[1],

                    _ => panic!("There should be a parameter mode for V1"),
                };

                let v2 = match parsed_opcode.parameter2_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[2] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[2],

                    _ => panic!("There should be a parameter mode for V2"),
                };

                if v1 != 0 {
                    cursor = v2 as usize;
                } else {
                    cursor += slice_length;
                }
            },

            IntcodeOpcode::JumpIfFalse => {
                // If param2 == 0, cursor = param2
                let v1 = match parsed_opcode.parameter1_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[1] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[1],

                    _ => panic!("There should be a parameter mode for V1"),
                };

                let v2 = match parsed_opcode.parameter2_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[2] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[2],

                    _ => panic!("There should be a parameter mode for V2"),
                };

                if v1 == 0 {
                    cursor = v2 as usize;
                } else {
                    cursor += slice_length;
                }
            },

            IntcodeOpcode::LessThan => {
                let v1 = match parsed_opcode.parameter1_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[1] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[1],

                    _ => panic!("There should be a parameter mode for V1"),
                };

                let v2 = match parsed_opcode.parameter2_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[2] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[2],

                    _ => panic!("There should be a parameter mode for V2"),
                };


                let out_reg: usize = slice_to_work_on[3] as usize;
                if v1 < v2 {
                    code_to_execute[out_reg] = 1;
                } else {
                    code_to_execute[out_reg] = 0;
                }

                if out_reg != cursor {
                    cursor += 4;
                }
            },

            IntcodeOpcode::Equals => {
                let v1 = match parsed_opcode.parameter1_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[1] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[1],

                    _ => panic!("There should be a parameter mode for V1"),
                };

                let v2 = match parsed_opcode.parameter2_mode {
                    ParameterMode::Position => {
                        let reg: usize = slice_to_work_on[2] as usize;
                        code_to_execute[reg]
                    },

                    ParameterMode::Immediate => slice_to_work_on[2],

                    _ => panic!("There should be a parameter mode for V2"),
                };


                let out_reg: usize = slice_to_work_on[3] as usize;
                if v1 == v2 {
                    code_to_execute[out_reg] = 1;
                } else {
                    code_to_execute[out_reg] = 0;
                }

                if out_reg != cursor {
                    cursor += 4;
                }
            },

            IntcodeOpcode::Halt => break,
            _ => (),
        };
    }

    // creates a String for the output, separated by commas
    let result = code_to_execute.into_iter()
        .map(|int| int.to_string())
        .collect::<Vec<String>>()
        .join(",");

    result
}

fn parse_parameter_mode(num: i64) -> Result<(ParameterMode, ParameterMode, ParameterMode), IntcodeParseError> {
    let param3_mode = num /100;
    let param2_mode = (num % 100)/10;
    let param1_mode = num % 10;

    let param1_mode = match param1_mode {
        0 => Ok(ParameterMode::Position),
        1 => Ok(ParameterMode::Immediate),
        _ => Err(IntcodeParseError::InvalidParamMode),
    }?;

    let param2_mode = match param2_mode {
        0 => Ok(ParameterMode::Position),
        1 => Ok(ParameterMode::Immediate),
        _ => Err(IntcodeParseError::InvalidParamMode),
    }?;

    let param3_mode = match param3_mode {
        0 => Ok(ParameterMode::Position),
        1 => Ok(ParameterMode::Immediate),
        _ => Err(IntcodeParseError::InvalidParamMode),
    }?;

    Ok( (param1_mode, param2_mode, param3_mode) )

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

    #[test]
    fn immediate_tests(){
        assert_eq!(process_input("1002,4,3,4,33"), "1002,4,3,4,99");
    }

    #[test]
    fn param_mode_tests()
    {
        //assert_eq!(parse_parameter_mode(1002), Ok((ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position)));
    }
}