const SIZE: usize = 30_000;
use clap::{Arg, App};

mod components;
use components::interpreter;

fn main() {
    // CLI parsing
    let matches = App::new("BrainFuck Interpreter")
                    .version("1.1.0")
                    .author("Nishant J. <nishantjo.12@gmail.com>")
                    .about("Run Brainfuck files here")
                    .arg(Arg::with_name("INPUT")
                            .help("Sets the input file to use")
                            .required(true)
                            .index(1)
                    ).get_matches();
    

    let input = std::fs::read_to_string(matches.value_of("INPUT").unwrap()).expect("That file's messed up");
    let mut mem: [u8;SIZE] = [0;SIZE]; // Memory buffer where the code will work
    let mut m_pointer: usize = 0;
    interpreter(input, &mut mem, &mut m_pointer, SIZE);
}
