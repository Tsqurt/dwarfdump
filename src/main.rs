use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let argc = args.len();

    if argc == 1 {
        print_help_message();
    } else if argc == 2 {
        // TODO: Implement the logic for argc == 2
        println!("Argument: {}", args[1]);
    } else {
        println!("Invalid number of arguments");
    }
}

fn print_help_message() {
    println!("Usage: dwarfdump <filename> [options]");
}
