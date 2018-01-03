extern crate clap;

use std::fs::File;
use std::io::Read;

// Maximum size of the bf data buffer
const BUF_SIZE: usize = 30_000;
fn main() {
    // Handle arguments
    let matches = handle_system_arguments();

    // Extract filename from arguments
    let filename = matches.value_of("filename").unwrap_or("test.bf");

    // Contents of file, interpret as bytes instead of string.
    let bf_code = get_bf_code_from_file(filename).into_bytes();

    parse_bf(bf_code);
}

fn parse_bf(bf_code: Vec<u8>) {
    // The large data buffer
    let mut data: Vec<u8> = vec![0; BUF_SIZE];
    let mut index: usize = 0; // pointer into the data buffer

    // Stack, used to implement for loops. Stack contains return addresses for the end of the loops
    let mut stack: Vec<usize> = Vec::new();
    let mut instr: usize = 0; // Index of instruction under execution

    while instr < bf_code.len() {
        // Current instruction to execute
        let c= bf_code[instr] as char;
        match c {
            '>' => { // Move the data pointer to the right
                if index < BUF_SIZE{
                    index +=1;
                }
                instr+=1;
            },
            '<' => { // Move the data pointer to the left
                if index > 0 {
                    index -= 1;
                }
                instr+=1;
            },
            '+' =>{ // Increment current byte
                if data[index] < std::u8::MAX {
                    data[index] += 1;
                }
                instr+=1;
            }
            '-' => { // Decrement current byte
                if data[index] > std::u8::MIN {
                    data[index] -= 1;
                }
                instr+=1;
            },
            '.' => { // Output
                if index <= BUF_SIZE {
                    print!("{}", data[index] as char);
                }
                instr+=1;
            }
            ',' => { // Input
                if index <= BUF_SIZE {
                    let mut buf = String::new();
                    std::io::stdin().read_line(&mut buf).expect("This is garbage input");
                    data[index] = buf.as_bytes()[0] as u8;
                }
                instr+=1;
            }
            '[' => { // Start loop if current byte is nonzero
                if data[index] != 0 { // Enter the loop. Save the 'return' statement on the stack
                    stack.push(instr); // Save this address to jump back to
                    instr+=1;
                } else { // Skip forward to matching ] bracket
                    let mut count_brackets: isize = 0;
                    let mut count : usize= 1;
                    while count_brackets >=0 {
                        if instr + count >= bf_code.len() {
                            println!("OUT OF BOUNDS, invalid BrainYuck code");
                            std::process::exit(1);
                        }
                        let c = bf_code[instr + count] as char;
                        if c == '[' {
                            count_brackets +=1;
                        } else if c == ']' {
                            count_brackets -=1;
                        }

                        count +=1;
                    }
                    instr += count ;
                }
            }
            ']' => { // End loop if current byte is zero
                if stack.len() == 0{ // Fatal error in program
                    println!("CANNOT RESOLVE CORRESPONDING BRACKET");
                    std::process::exit(1)
                }
                if data[index as usize] != 0 { // Jump back
                    match stack.pop() {
                        None => println!("CANNOT POP"),
                        Some(x) => instr = x,
                    }
                } else { // End loop; go forward
                    stack.pop();
                    instr += 1;
                }
            }
            _ => {
                instr+=1;
            }
        }
    }
}

fn handle_system_arguments() -> clap::ArgMatches  {
    clap::App::new("BrainYuck Compilator")
        .version("0.1")
        .author("Kevin H. <gkh998@gmail.com>")
        .about("Executes BrainYuck code in a fast and efficient manner")
        .arg(clap::Arg::with_name("filename")
            .short("f")
            .long("filename")
            .value_name("FILE")
            .help("Parses brainyuck from custom file")
            .takes_value(true))
        .get_matches()
}

///
/// Read the brainfuck code from the given file
/// If the file exists, open it, and read the contents into a string.
/// return the contents of this string, or the Hello World string.
/// @param filename: &str reference to the name of the file
/// @returns String containing the file contents
///
///
fn get_bf_code_from_file (filename: & str) -> String {
    const HELLO_WORLD: String = " ++++++++++[>+++++++>++++++++++>+++>+<<<<-]>++.>+.+++++++..+++.>++.<<+++++++++++++++.>.+++.------.--------.>+.>.";
    let mut contents: String = String::new();

    let mut f = File::open(filename)?
        .read_to_string(&mut contents);

    match f { // Either an error, or
        res => f = res,
        Err(e) => {
            println!("Error opening file: {}", e);
            contents = HELLO_WORLD;
        }
    }

    return contents;
}
