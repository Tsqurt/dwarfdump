use std::env;
use std::io::Read;

mod elf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 1 {
        print_help_message();
    } else if argc == 2 {
        // 1. Open the file argv[1] as `inputfile`
        let input_file = &args[1];
        let mut file = match std::fs::File::open(input_file) {
            Ok(file) => file,
            Err(error) => {
            eprintln!("Error opening file: {}", error);
            return;
            }
        };

        // 2. Read the content of `inputfile` into `buffer`
        let mut buffer = Vec::new();
        match file.read_to_end(&mut buffer) {
            Ok(_) => (),
            Err(error) => {
            eprintln!("Error reading file: {}", error);
            return;
            }
        }

        // 3. Parse the content of `buffer` as an ELF64 file format and find DWARF sections
        let elf = elf::elf64::decode_buffer_to_elf64(&buffer);
        println!("{:#?}", elf);

        // 4. Print the DWARF sections

    } else {
        println!("Invalid number of arguments");
    }
}

fn print_help_message() {
    println!("Usage: dwarfdump <filename> [options]");
}
