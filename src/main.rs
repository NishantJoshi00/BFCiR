use clap::Parser;
use std::{
    error::Error,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
    process::Command,
};
mod cli;

#[derive(Debug)]
struct CError {}

impl Display for CError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CError")
    }
}

impl Error for CError {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = cli::Args::parse();
    let mut generated = BufWriter::new(File::create("a.rs")?);
    let mut pointer: u128 = 0;
    let size = 30000;
    write!(
        generated,
        r#"
    use std::io::Read;

    fn read_byte() -> u8 {{
        let input: Option<u8> = std::io::stdin()
            .bytes() 
            .next()
            .and_then(|result| result.ok())
            .map(|byte| byte as u8);
        input.unwrap()
    }}
    fn main() {{
        let mut buffer: [u8; {}] = [0; {}];
        let mut pointer: usize = 0;
    "#,
        size, size
    )?;

    let source_code = BufReader::new(File::open(args.filename.clone())?);
    for line in source_code.lines() {
        for ch in line?.chars() {
            match ch {
                '>' => {
                    if pointer + 1 == size {
                        pointer = 0;
                        writeln!(generated, "pointer = 0;")?;
                    } else {
                        pointer += 1;
                        writeln!(generated, "pointer += 1;")?;
                    }
                }
                '<' => {
                    if pointer == 0 {
                        pointer = size - 1;
                        writeln!(generated, "pointer = {} - 1;", size)?;
                    } else {
                        pointer -= 1;
                        writeln!(generated, "pointer -= 1;")?;
                    }
                }
                '+' => {
                    writeln!(generated, "buffer[pointer] += 1;")?;
                }
                '-' => {
                    writeln!(generated, "buffer[pointer] -= 1;")?;
                }
                '.' => {
                    writeln!(generated, "print!(\"{{}}\", buffer[pointer] as char);")?;
                }
                ',' => {
                    writeln!(generated, "buffer[pointer] = read_byte();")?;
                }
                '[' => {
                    writeln!(generated, "while buffer[pointer] != 0 {{")?;
                }
                ']' => {
                    writeln!(generated, "}}")?;
                }

                _ => Err(CError {})?,
            }
        }
    }
    writeln!(generated, "}}")?;
    generated.flush()?;
    Command::new("cargo")
        .args(&["fmt", "--", "a.rs"])
        .status()?;
    Command::new("rustc").args(&["a.rs"]).status()?;
    Command::new("rm").args(&["a.rs"]).status()?;

    Ok(())
}
