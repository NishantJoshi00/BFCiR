
use std::io::{stdin, stdout, Read, Write};

fn allocate_from_stdin(mem: &mut [u8], m_pointer: usize) {
    let _ = stdout().flush();
    let _ = stdin().lock().read(&mut mem[m_pointer..m_pointer + 1]).expect("Can't read, but not illiterate");
}

pub fn interpreter(input: String, mem: &mut [u8], m_pointer: &mut usize, size: usize) {
    // Environment setup

    let mut i_pointer: usize = 0; // instruction pointer
    let mut loops: Vec<usize> = Vec::new();

    // Cleaning Input
    let input: String = input.split_whitespace().collect();

    let code_size = input.len(); // Storing the length of the source code
    let instructions: Vec<char> = input.chars().collect(); // Converting the instructions from string to Vector<char>
    

    while i_pointer < code_size {
        match instructions[i_pointer] {
            // Standard I/O operations
            '.' => print!("{}", mem[*m_pointer] as char),
            ',' => allocate_from_stdin(mem, *m_pointer),

            // moving through memory
            '>' => if *m_pointer == size - 1 { *m_pointer = 0; } else { *m_pointer += 1; }
            '<' => if *m_pointer == 0 { *m_pointer = size - 1; } else { *m_pointer -= 1; }

            // memory manuplation
            '+' => if mem[*m_pointer] == 255 { mem[*m_pointer] = 0; } else { mem[*m_pointer] += 1; }
            '-' => if mem[*m_pointer] == 0 { mem[*m_pointer] = 255; } else { mem[*m_pointer] -= 1; }

            // Control structures
            '[' => loops.push(i_pointer),
            ']' => {
                if loops.len() == 0 { std::process::exit(1); }
                if mem[*m_pointer] == 0 { loops.pop(); }
                else { i_pointer = *loops.last().unwrap(); }
            }
            _ => std::process::exit(-1)
        }
        i_pointer += 1;
    }
}
