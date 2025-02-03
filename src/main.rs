use std::env::args;
use std::fs::read_to_string;
use std::process;

const VERSION: &str = "0.0.1";

fn help() {
    let usage = "
Usage: sed-rust [help h] [version v]... {script-only-if-no-other-script} [input-file]...
";
    println!("{usage}");
}

fn version() {
    println!("{}", VERSION);
}

fn parse_script(s: &str) -> &str {
    s
}

fn handle_script(path: &str, script: &str) {
    let file = read_to_string(path).unwrap_or_else(|err| {
      eprintln!("{}: {}", path, err);
      process::exit(1);
    });
    println!("{}", file)
}

fn handle_args() {
    let args = args();
    let mut pos: Vec<String> = vec![];

    for arg in args.skip(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                help();
                break;
            }
            "--version" | "-v" => {
                version();
                break;
            }
            _ => pos.push(arg),
        }
    }
    handle_script(pos.last().unwrap(), pos.first().unwrap());
}

fn main() {
    handle_args()
}
