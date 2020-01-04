#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub mod IntcodeComputer {
    use std::sync::{Arc, mpsc, Mutex};
    use std::sync::mpsc::{Receiver, Sender, channel};

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
        RelativeBaseOffset, //09
        Halt, // 99

        Dummy, // Not a normal opcode
    }

    fn parse(full_opcode: i64) -> ParsedOpcodes {
        let opcode_num = full_opcode % 100;
        let (mode1, mode2, mode3) = parse_parameter_mode(full_opcode/100).expect("Bad parameter mode!");
        let mut slice_length = 0;

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

            9 => {
                slice_length = 2;
                Ok(IntcodeOpcode::RelativeBaseOffset)
            },

            99 => Ok(IntcodeOpcode::Halt),// Halt

            _ => Err(IntcodeParseError::InvalidOpcode),// Err
        };

        let parsed_opcode = ParsedOpcodes{
            opcode: opcode.expect("Bad opcode"),
            parameter1_mode: mode1,
            parameter2_mode: mode2,
            parameter3_mode: mode3,
            slice_length,
        };

        parsed_opcode
    }

    #[derive(Debug, PartialEq)]
    enum ParameterMode{
        Immediate,
        Position,
        Relative,
        NoParam,
    }

    struct ParsedOpcodes {
        opcode: IntcodeOpcode,
        parameter1_mode: ParameterMode,
        parameter2_mode: ParameterMode,
        parameter3_mode: ParameterMode,
        slice_length: usize,
    }

    // input_vector is in reverse order
    pub fn process_input(input: &str, mut inputs_vector: SharedReceiver<i64>, outputs_vector: Sender<i64>, id: i64) -> (String, i64) {
        // creates the vector from the program code
        let input_vector = input.trim()
            .split(',')
            .map(|s| s.to_string())
            .filter(|s| s != "")
            .map(|s| s.parse::<i64>().expect("Prog parse fail"))
            .collect::<Vec<i64>>();

        let mut last_output = 0;

        let mut relative_offset: i64 = 0;

        let mut cursor = 0;
        let mut code_to_execute = input_vector.clone();
        let mut append_vec = vec![0; 100*code_to_execute.len()];

        code_to_execute.append(&mut append_vec);

        while cursor < code_to_execute.len() && code_to_execute[cursor] != 99 {
            //let mut slice_length = 0;
            let full_opcode = code_to_execute[cursor];

            let parsed_opcode = parse(full_opcode);

            let slice_to_work_on = &code_to_execute[cursor..cursor + parsed_opcode.slice_length];

            match parsed_opcode.opcode {
                IntcodeOpcode::Add => {
                    let v1 = match parsed_opcode.parameter1_mode {
                        ParameterMode::Position => {
                            let reg: usize = slice_to_work_on[1] as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[1] + relative_offset) as usize;
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

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[2] + relative_offset) as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Immediate => slice_to_work_on[2],

                        _ => panic!("There should be a parameter mode for V2"),
                    };



                    let value = v1 + v2;

                    let out_reg = match parsed_opcode.parameter3_mode {
                        ParameterMode::Position => {
                            let out_reg: usize = slice_to_work_on[3] as usize;
                            code_to_execute[out_reg] = value;
                            out_reg
                        },

                        ParameterMode::Relative => {
                            let out_reg = (slice_to_work_on[3] + relative_offset) as usize;
                            code_to_execute[out_reg ] = value;
                            out_reg
                        },

                        _ => panic!("Can't use param modes that aren't Relative or Position for an output"),
                    };


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

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[1] + relative_offset) as usize;
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

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[2] + relative_offset) as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Immediate => slice_to_work_on[2],

                        _ => panic!("There should be a parameter mode for V2"),
                    };

                    let value = v1 * v2;
                    let out_reg = match parsed_opcode.parameter3_mode {
                        ParameterMode::Position => {
                            let out_reg: usize = slice_to_work_on[3] as usize;
                            code_to_execute[out_reg] = value;
                            out_reg
                        },

                        ParameterMode::Relative => {
                            let out_reg = (slice_to_work_on[3] + relative_offset) as usize;
                            code_to_execute[out_reg ] = value;
                            out_reg
                        },

                        _ => panic!("Can't use param modes that aren't Relative or Position for an output"),
                    };

                    if out_reg != cursor {
                        cursor += 4;
                    }
                },

                IntcodeOpcode::ReadInteger => {
                    //let mut input_value = String::new();
                    let addr = match parsed_opcode.parameter1_mode {
                        ParameterMode::Position => slice_to_work_on[1] as usize,
                        ParameterMode::Relative => (slice_to_work_on[1] + relative_offset) as usize,
                        _ => panic!("Bad param mode"),
                    };
                    println!("Enter a single integer: ");
                    //io::stdout().flush();
                    //io::stdin().read_line(&mut input_value).ok().expect("Invalid input. Crashing now. Goodbye!\n");

                    //let value_to_save = input_value.trim().parse::<i64>().expect("Invalid integer. Goodbye!\n");

                    code_to_execute[addr] = inputs_vector.next().expect("panic at ReadInt");

                    if addr != cursor {
                        cursor += 2;
                    }
                },

                IntcodeOpcode::PrintOutput => {
                    let output = match parsed_opcode.parameter1_mode {
                        ParameterMode::Position => code_to_execute[slice_to_work_on[1] as usize],
                        ParameterMode::Relative => code_to_execute[(slice_to_work_on[1] + relative_offset) as usize],
                        ParameterMode::Immediate => slice_to_work_on[1],
                        _ => panic!("Bad param mode"),
                    };
                    cursor += 2;


                    //println!("Output value is {}", cursor-2);
                    outputs_vector.send(output.clone());
                    last_output = output.clone();
                },

                IntcodeOpcode::JumpIfTrue => {
                    // If param1 != 0, cursor = param2
                    let v1 = match parsed_opcode.parameter1_mode {
                        ParameterMode::Position => {
                            let reg: usize = slice_to_work_on[1] as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[1] + relative_offset as i64) as usize;
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

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[2] + relative_offset as i64) as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Immediate => slice_to_work_on[2],

                        _ => panic!("There should be a parameter mode for V2"),
                    };

                    if v1 != 0 {
                        cursor = v2 as usize;
                    } else {
                        cursor += parsed_opcode.slice_length;
                    }
                },

                IntcodeOpcode::JumpIfFalse => {
                    // If param2 == 0, cursor = param2
                    let v1 = match parsed_opcode.parameter1_mode {
                        ParameterMode::Position => {
                            let reg: usize = slice_to_work_on[1] as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[1] + relative_offset as i64) as usize;
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

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[2] + relative_offset as i64) as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Immediate => slice_to_work_on[2],

                        _ => panic!("There should be a parameter mode for V2"),
                    };

                    if v1 == 0 {
                        cursor = v2 as usize;
                    } else {
                        cursor += parsed_opcode.slice_length;
                    }
                },

                IntcodeOpcode::LessThan => {
                    let v1 = match parsed_opcode.parameter1_mode {
                        ParameterMode::Position => {
                            let reg: usize = slice_to_work_on[1] as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[1] + relative_offset as i64) as usize;
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

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[2] + relative_offset as i64) as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Immediate => slice_to_work_on[2],

                        _ => panic!("There should be a parameter mode for V2"),
                    };


                    let mut value = 0;
                    if v1 < v2 {
                        value = 1;
                    } else {
                        value = 0;
                    }

                    let out_reg = match parsed_opcode.parameter3_mode {
                        ParameterMode::Position => {
                            let out_reg: usize = slice_to_work_on[3] as usize;
                            code_to_execute[out_reg] = value;
                            out_reg
                        },

                        ParameterMode::Relative => {
                            let out_reg = (slice_to_work_on[3] + relative_offset as i64) as usize;
                            code_to_execute[out_reg ] = value;
                            out_reg
                        },

                        _ => panic!("Can't use param modes that aren't Relative or Position for an output"),
                    };

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

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[1] + relative_offset as i64) as usize;
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

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[2] + relative_offset as i64) as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Immediate => slice_to_work_on[2],

                        _ => panic!("There should be a parameter mode for V2"),
                    };


                    let mut value = 0;
                    if v1 == v2 {
                        value = 1;
                    } else {
                        value = 0;
                    }

                    let out_reg = match parsed_opcode.parameter3_mode {
                        ParameterMode::Position => {
                            let out_reg: usize = slice_to_work_on[3] as usize;
                            code_to_execute[out_reg] = value;
                            out_reg
                        },

                        ParameterMode::Relative => {
                            let out_reg = (slice_to_work_on[3] + relative_offset as i64) as usize;
                            code_to_execute[out_reg ] = value;
                            out_reg
                        },

                        _ => panic!("Can't use param modes that aren't Relative or Position for an output"),
                    };

                    if out_reg != cursor {
                        cursor += 4;
                    }
                },

                IntcodeOpcode::RelativeBaseOffset => {

                    let v1 = match parsed_opcode.parameter1_mode {
                        ParameterMode::Position => {
                            let reg: usize = slice_to_work_on[1] as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Relative => {
                            let reg: usize = (slice_to_work_on[1] + relative_offset) as usize;
                            code_to_execute[reg]
                        },

                        ParameterMode::Immediate => slice_to_work_on[1],

                        _ => panic!("There should be a parameter mode for V1"),
                    };

                    relative_offset += v1;
                    cursor += parsed_opcode.slice_length;
                },

                IntcodeOpcode::Halt => {
                    println!("Halted - ID: {}", id);
                    break
                },
                _ => (),
            };
        }

        // creates a String for the output, separated by commas
        let result = code_to_execute.into_iter()
            .map(|int| int.to_string())
            .collect::<Vec<String>>()
            .join(",");

        (result, last_output)
    }

    fn parse_parameter_mode(num: i64) -> Result<(ParameterMode, ParameterMode, ParameterMode), IntcodeParseError> {
        let param3_mode = num /100;
        let param2_mode = (num % 100)/10;
        let param1_mode = num % 10;

        let param1_mode = match param1_mode {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err(IntcodeParseError::InvalidParamMode),
        }?;

        let param2_mode = match param2_mode {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err(IntcodeParseError::InvalidParamMode),
        }?;

        let param3_mode = match param3_mode {
            0 => Ok(ParameterMode::Position),
            1 => Ok(ParameterMode::Immediate),
            2 => Ok(ParameterMode::Relative),
            _ => Err(IntcodeParseError::InvalidParamMode),
        }?;

        Ok( (param1_mode, param2_mode, param3_mode) )

    }

    #[derive(Clone)]
    pub struct SharedReceiver<T>(Arc<Mutex<Receiver<T>>>);

    impl<T> Iterator for SharedReceiver<T> {
        type Item = T;

        fn next(&mut self) -> Option<T> {
            let guard = self.0.lock().unwrap();
            guard.recv().ok()
        }
    }

    pub fn shared_channel<T>() -> (Sender<T>, SharedReceiver<T>) {
        let (s, r) = channel();
        (s, SharedReceiver(Arc::new(Mutex::new(r))))
    }
}