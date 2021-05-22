const SIZE: usize = 30_000;
const DEBUG: bool = false;
use std::io::{stdin, stdout, Write, Read};
fn main() {
    // Environment setup
    let mut mem: [u8;SIZE] = [0;SIZE]; // Memory buffer where the code will work
    let mut m_pointer: usize = 0; // Memory pointer
    let mut i_pointer: usize = 0; // instruction pointer
    let mut loops: Vec<usize> = Vec::new();

    let mut input = String::new();
    let _ = stdout().flush();
    stdin().read_to_string(&mut input).expect("Well you just suck, and this is brainfuck");
    input = input.split_whitespace().collect();

    let code_size = input.len();
    let instructions: Vec<char> = input.chars().collect();

    if DEBUG { println!("Source code: \n{}", input); }
    while i_pointer < code_size {
        if DEBUG {
            println!("IP: {}, SP: {}, SC: {}, IS: {}", i_pointer, m_pointer, mem[m_pointer], instructions[i_pointer]);
        }
        match instructions[i_pointer] {
            '.' => print!("{}", mem[m_pointer] as char),
            '>' => m_pointer += 1,
            '<' => m_pointer -= 1,
            '[' => loops.push(i_pointer),
            ']' => {
                if loops.len() == 0 {
                    std::process::exit(1);
                }
                if mem[m_pointer] == 0 {
                    loops.pop();
                } else {
                    i_pointer = *loops.last().unwrap();
                }
            }
            // ',' => { }, // Input Logic pending
            '-' => mem[m_pointer] = if mem[m_pointer] == 0 { 255 } else { mem[m_pointer] - 1 },
            '+' => mem[m_pointer] = if mem[m_pointer] == 255 { 0 } else { mem[m_pointer] + 1 },
            _ => {
                std::process::exit(-1);
            }
        }
        i_pointer += 1;
    }

}
