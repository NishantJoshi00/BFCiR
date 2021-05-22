const SIZE: usize = 30_000;
const DEBUG: bool = false;
use std::io::{stdin, stdout, Write};
use clap::{Arg, App};
fn main() {
    // CLI parsing
    let matches = App::new("BrainFuck Interpreter")
                    .version("0.1.0")
                    .author("Nishant J. <nishantjo.12@gmail.com>")
                    .about("Run Brainfuck files here")
                    .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1)
                    ).get_matches();
    
    // Environment setup
    let mut mem: [u8;SIZE] = [0;SIZE]; // Memory buffer where the code will work
    let mut m_pointer: usize = 0; // Memory pointer
    let mut i_pointer: usize = 0; // instruction pointer
    let mut loops: Vec<usize> = Vec::new();


    let input = std::fs::read_to_string(matches.value_of("INPUT").unwrap()).expect("That file's messed up");

    // let _ = stdout().flush();
    // stdin().read_to_string(&mut input).expect("Well you just suck, and this is brainfuck");
    let input: String = input.split_whitespace().collect();

    let code_size = input.len();
    let instructions: Vec<char> = input.chars().collect();
    let mut input_buf: std::str::Chars;
    let mut input_ = String::new();

    if input.contains(',') {
        let _ = stdout().flush();
        stdin().read_line(&mut input_).expect("Well that's bad");
    }
    input_buf = input_.chars();
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
            ',' => {
                mem[m_pointer] = input_buf.next().unwrap() as u8;
             }, // Input Logic pending
            '-' => mem[m_pointer] = if mem[m_pointer] == 0 { 255 } else { mem[m_pointer] - 1 },
            '+' => mem[m_pointer] = if mem[m_pointer] == 255 { 0 } else { mem[m_pointer] + 1 },
            _ => {
                std::process::exit(-1);
            }
        }
        i_pointer += 1;
    }

}
