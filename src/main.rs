use std::env::args;
use std::fs;
use std::io;
use std::process;

const VERSION: &str = "0.0.1";

fn help() {
    let usage = "
Usage: sed-rust [help h] [version v]... {script-only-if-no-other-script} [input-file]...
";
    print!("{usage}");
}

fn version() {
    print!("{}", VERSION);
}

fn parse_script(s: &str) -> &str {
    s
}

struct FilesBuffer {
    files: Vec<String>,
}

fn handle_script(script: &str, files: Option<&[String]>) {
    let fb = match files {
        Some([]) | None => {
            let buf = io::read_to_string(io::stdin()).unwrap();
            FilesBuffer { files: vec![buf] }
        }
        Some(files) => {
            let mut buffers = vec![];
            for f in files {
                let file = fs::read_to_string(f.to_string()).unwrap_or_else(|err| {
                    eprintln!("{}: {}", f.to_string(), err);
                    process::exit(1);
                });
                buffers.push(file);
            }
            FilesBuffer { files: buffers }
        }
    };
    let cmd = parse_script(script);
    println!("{}", cmd);
    println!("{:?}", fb.files)
}

fn handle_args() {
    let args = args();
    let mut pos: Vec<String> = vec![];

    for arg in args.skip(1) {
        match arg.as_str() {
            "--help" | "-h" => {
                help();
                process::exit(0);
            }
            "--version" | "-v" => {
                version();
                process::exit(0);
            }
            _ => pos.push(arg),
        }
    }
    if pos.len() == 0 {
        eprint!("Missing script positional argument");
        help();
        process::exit(1);
    }
    handle_script(pos.last().unwrap(), pos.get(1..));
}

fn main() {
    handle_args()
}
