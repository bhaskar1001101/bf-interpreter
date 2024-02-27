use std::io::{Read, Result};

fn main() -> Result<()> {
    let file_name = std::env::args().nth(1).unwrap();
    let source = std::fs::read(&file_name)?;

    let mut pointer = 0;
    let mut memory = vec![0u8; 30000];
    let mut program_counter = 0;
    'program: loop {
        match source[program_counter] {
            b'+' => memory[pointer] = memory[pointer].wrapping_add(1),
            b'-' => memory[pointer] = memory[pointer].wrapping_sub(1),
            b'.' => println!("{}", memory[pointer] as char),
            b',' => std::io::stdin().read_exact(&mut memory[pointer..pointer + 1])?,
            b'>' => pointer = (pointer + 1) % memory.len(),
            b'<' => pointer = (pointer + memory.len() - 1) % memory.len(),
            b'[' => {
                if memory[pointer] == 0 {
                    let mut depth = 1;
                    while depth != 0 && program_counter < source.len() {
                        program_counter += 1;
                        match source[program_counter] {
                            b'[' => depth += 1,
                            b']' => depth -= 1,
                            _ => {}
                        }
                        
                    } 
                }
            },
            b']' => if memory[pointer] != 0 {
                let mut depth = 1;
                while depth != 0 && program_counter > 0 {
                    program_counter -= 1;
                    match source[program_counter] {
                        b'[' => depth -= 1,
                        b']' => depth += 1,
                        _ => {}
                    }
                }
            },
            _ => {}
        }
        program_counter += 1;

        if source.len() <= program_counter {
            break 'program;
        }
    }

    Ok(())
}
