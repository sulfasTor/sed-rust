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

struct ScriptCommands {
    addr: String,
    sed_cmd: String,
    options: String,
}

fn parse_script(s: &str) -> Result<ScriptCommands, String> {
    // See https://www.gnu.org/software/sed/manual/sed.html#Introduction
    // TODO: Implement regex addresses
    let mut addr = String::new();
    let mut sed_cmd = String::new();
    let mut options = String::new();

    println!("{s}");
    let s = s.chars().into_iter().peekable().enumerate();
    while let Some((i, c)) = s.next() {
        if c.is_numeric() {
            addr.push(c);
            continue
        }
        if c == ',' {
            addr.push(c);
            continue
        }
        break
    }
    sed_cmd = s.next().map(|(i, c)| {
        if !c.is_numeric() {
            return Err(format!("sed: -e char {}: unknown command: {}", i, c))
        }
        Ok(c.to_string())
    }).unwrap()?;

    println!("ADDR: {}", addr);
    println!("X: {}", sed_cmd);
    println!("OPTIONS: {}", options);
    Ok(ScriptCommands {
        addr,
        sed_cmd,
        options,
    })
}

fn eval_script(cmd: ScriptCommands, fb: &FilesBuffer) -> Result<(), String> {
    Ok(())
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
    let res = eval_script(
        cmd.unwrap_or_else(|err| {
            eprint!("{}", err);
            process::exit(1);
        }),
        &fb,
    );
    println!("{:?}", res.unwrap());
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
    handle_script(pos.first().unwrap(), pos.get(1..));
}

fn main() {
    handle_args()
}
