extern crate intcode_computer_lib;

use std::fs::{File};
use std::io::{Read};
use std::thread;

use intcode_computer_lib::IntcodeComputer::*;

fn main() {

    let mut file_contents = read_file("input.txt");

    //let mut noun = 0;
    //let mut verb = 0;

    let mut working_input_chars = file_contents.clone().trim().split(',').map(|s| s.to_string()).collect::<Vec<String>>();
    //working_input_chars[1] = noun.to_string();
    //working_input_chars[2] = verb.to_string();

    let working_input = working_input_chars.into_iter().collect::<Vec<String>>().join(",");

    //let mut in_vec = vec![0,4];
    //let mut out_vec=vec![];
    //let output: String = process_input(working_input.clone().as_str(), &mut in_vec, &mut out_vec);

    let mut max_thrust = 0;
    for i in 5..10 {
        for j in 5..10 {
            if j == i {
                continue;
            }
            for k in 5..10 {
                if k == i || k == j {
                    continue;
                }
                for l in 5..10 {
                    if l == i || l == j || l == k {
                        continue;
                    }
                    for m in 5..10 {
                        if m == i || m == j || m == k || m == l {
                            continue;
                        }
                        let phase_str = format!("{},{},{},{},{}", i, j, k, l, m);
                        let attempt = run_amplifiers(working_input.clone(), phase_str.as_str());

                        if attempt > max_thrust {
                            max_thrust = attempt;
                        }
                    }
                }
            }
        }
    }
    println!("Max thrust = {}", max_thrust);
}


fn run_amplifiers(program: String, phase_order: &str) -> i64 {
    let phases = phase_order.split(",").map(|s| s.to_string().parse::<i64>().unwrap()).collect::<Vec<i64>>();

    let (tx0, rx1) = shared_channel();
    let (tx1, rx2) = shared_channel();
    let (tx2, rx3) = shared_channel();
    let (tx3, rx4) = shared_channel();
    let (tx4, rx0) = shared_channel();

    let (out_tx, out_rx) = shared_channel();

    let rx_arr = vec![rx0, rx1, rx2, rx3, rx4];
    let tx_arr = vec![tx0, tx1, tx2, tx3, tx4];
    let mut children = Vec::new();

    for id in 0..5 {
        let rx = rx_arr[id].clone();
        let tx = tx_arr[id].clone();
        let prog_clone = program.clone();
        let otx = out_tx.clone();

        let child = thread::spawn(move || {
            let (_, val) = process_input(prog_clone.as_str(), rx, tx, id as i64);

            otx.clone().send((id, val));
        });
        tx_arr[id].clone().send(phases[id]);
        if id == 0 {
            tx_arr[id].clone().send(0);
        }
        children.push(child);
    }
    let mut i = 0;
    for child in children {
        child.join().expect(format!("Child panic, {}", i).as_str());
        i += 1;
    }

    let mut ret_val = 0;
    for val_pair in out_rx{
        if val_pair.0 == 0 {
            ret_val = val_pair.1;
            break;
        }
    }
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

   /* #[test]
    fn process_tests() {
        let mut in_vec = vec![];
        let mut out_vec = vec![];
        assert_eq!(process_input("1,0,0,0,99", &mut in_vec, &mut out_vec), "2,0,0,0,99");
        assert_eq!(process_input("2,3,0,3,99", &mut in_vec, &mut out_vec), "2,3,0,6,99");
        assert_eq!(process_input("2,4,4,5,99,0", &mut in_vec, &mut out_vec), "2,4,4,5,99,9801");
        assert_eq!(process_input("1,1,1,4,99,5,6,0,99", &mut in_vec, &mut out_vec), "30,1,1,4,2,5,6,0,99");
    }

    #[test]
    fn immediate_tests(){
        let mut in_vec = vec![];
        let mut out_vec = vec![];
        assert_eq!(process_input("1002,4,3,4,33", &mut in_vec, &mut out_vec), "1002,4,3,4,99");
    }

    #[test]
    fn param_mode_tests() {
        //assert_eq!(parse_parameter_mode(1002), Ok((ParameterMode::Position, ParameterMode::Immediate, ParameterMode::Position)));
    }*/

    #[test]
    fn phase_test() {
        //assert_eq!(run_amplifiers("3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0".to_string(), "4,3,2,1,0"), 43210);
        assert_eq!(run_amplifiers("33,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5".to_string(), "9,8,7,6,5"), 139629729);
    }
}

