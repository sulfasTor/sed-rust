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

#[derive(Debug)]
struct ScriptCommands {
    addr: (i32, i32),
    sed_cmd: String,
    options: Vec<String>,
}

fn parse_script(s: &str) -> Result<ScriptCommands, String> {
    // See https://www.gnu.org/software/sed/manual/sed.html#Introduction
    // TODO: Implement regex addresses
    let mut range: (i32, i32) = (-1, -1);
    let mut sed_cmd = String::new();
    let mut cmd_separator = String::new();
    let mut options: Vec<String> = vec![];
    let mut s = s.chars().enumerate();
    let mut range_str = String::new();
    let mut options_str = String::new();
    while let Some((i, c)) = s.next() {
        match c {
            ',' => {
                range.0 = range_str
                    .parse()
                    .map_err(|_| format!("sed: -e char {}: unknown command: {}", i + 1, c))?;
                range_str = String::new();
            }
            '0'..='9' => range_str.push(c),
            //Valid sed script command
            'a' | 's' => {
                if !sed_cmd.is_empty() {
                    options_str.push(c);
                    continue;
                }
                range.1 = range_str.parse().unwrap_or(-1);
                sed_cmd = c.to_string();
            }
            _ => {
                if sed_cmd.is_empty() {
                    return Err(format!("sed: -e char {}: unknown command: {}", i + 1, c));
                }
                if cmd_separator.is_empty() {
                    cmd_separator = c.to_string();
                    continue;
                }

                if c.to_string() == cmd_separator {
                    // todo: Parse command flags
                    if !validate_options(&sed_cmd, &options) {
                        return Err(format!(
                            "sed: -e char {}: unknown option to {}",
                            i + 1,
                            sed_cmd
                        ));
                    }
                    options.push(options_str.clone());
                    options_str = String::new();
                    continue;
                }
                options_str.push(c);
            }
        }
    }

    Ok(ScriptCommands {
        addr: range,
        sed_cmd,
        options,
    })
}

fn validate_options(cmd: &str, options: &[String]) -> bool {
    match cmd {
        "s" => {
            if options.len() + 1 > 2 {
                return false;
            }
            return true;
        }
        _ => return false,
    }
}

fn eval_script(cmd: ScriptCommands, fb: &FilesBuffer) -> Result<(), String> {
    match cmd.sed_cmd.as_str() {
        "s" => {
            let (start, end) = cmd.addr;
            for f in &fb.files {
                for (i, l) in f.lines().enumerate() {
                    if i >= start as usize && i <= end as usize {
                        println!(
                            "{}",
                            l.replace(cmd.options.get(0).unwrap(), cmd.options.get(1).unwrap())
                        );
                        continue;
                    }
                    println!("{}", l)
                }
            }
        }
        _ => {}
    }
    Ok(())
}

#[derive(Debug)]
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
    let _res = eval_script(
        cmd.unwrap_or_else(|err| {
            eprint!("{}", err);
            process::exit(1);
        }),
        &fb,
    );
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
