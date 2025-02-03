use std::env::args;

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

fn open_file(f: &str) -> &str {
  f
}

fn parse_script(s: &str) -> &str {
    s
}

fn handle() {
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
    println!("{:?}", pos)
}

fn main() {
    handle()
}
