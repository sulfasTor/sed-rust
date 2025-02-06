use std::char;
use std::env::args;
use std::fs;
use std::io;
use std::io::Read;
use std::process;

const VERSION: &str = "0.0.1";

fn help() {
    let usage =
        "Usage: sed-rust [help h] [version v]... {script-only-if-no-other-script} [input-file]...";
    print!("{usage}");
}

fn version() {
    print!("{}", VERSION);
}

#[derive(Debug)]
struct ScriptCommands {
    addr: (i32, i32),
    cmd: String,
    flag: String,
    options: Vec<String>,
}

fn parse_script(s: &str) -> Result<ScriptCommands, String> {
    // See https://www.gnu.org/software/sed/manual/sed.html#Introduction
    // TODO: Implement regex addresses
    // TODO: Error out if addr is 0: invalid usage of line address 0
    let mut addr: (i32, i32) = (0, 0);
    let mut cmd = String::new();
    let mut flag = String::new();
    let mut options: Vec<String> = vec![];

    let cmd_len = s.len();
    let mut s = s.chars().enumerate();
    let mut temp_str = String::new();
    let mut cmd_separator: Option<char> = None;
    while let Some((i, c)) = s.next() {
        match c {
            ',' => {
                if cmd.is_empty() {
                    addr.0 = temp_str.parse().map_err(|_| {
                        format!(
                            "sed-rust: -e expression #1, char {}: unknown command: `{}'",
                            i + 1,
                            c
                        )
                    })?;
                }
                temp_str = String::new();
            }
            '0'..='9' => temp_str.push(c),
            's' => {
                if !cmd.is_empty() {
                    temp_str.push(c);
                    continue;
                }
                addr.1 = temp_str.parse().unwrap_or(0);
                temp_str = String::new();
                cmd = c.to_string();
            }
            _ if cmd.is_empty() => {
                return Err(format!(
                    "sed-rust: -e expression #1, char {}: unknown command: `{}'",
                    i + 1,
                    c
                ));
            }
            _ if valid_options(&cmd, &options) => {
                if !valid_option_flag(&cmd, &c) {
                    return Err(format!(
                        "sed-rust: -e expression #1, char {}: unknown option to `{}'",
                        i + 1,
                        cmd
                    ));
                }
                flag.push(c);
            }
            _ if Some(c) == cmd_separator => {
                options.push(temp_str.clone());
                temp_str = String::new();
            }
            _ if i == cmd_len - 1 => {
                if !valid_options(&cmd, &options) {
                    return Err(format!(
                        "sed-rust: -e expression #1, char {}: unterminated `{}' command",
                        i + 1,
                        cmd
                    ));
                }
            }
            _ => {
                if cmd_separator.is_none() {
                    cmd_separator = Some(c);
                    continue;
                }
                temp_str.push(c);
            }
        }
    }

    Ok(ScriptCommands {
        addr,
        cmd,
        flag,
        options,
    })
}

fn valid_options(cmd: &str, options: &[String]) -> bool {
    match cmd {
        "s" => options.len() == 2,
        _ => false,
    }
}

fn valid_option_flag(cmd: &str, flag: &char) -> bool {
    match cmd {
        "s" => *flag == 'g',
        _ => false,
    }
}

fn eval_s_command(cmd: &ScriptCommands, buffer: &str) -> Result<(), String> {
    let (start, end) = cmd.addr;
    for (i, l) in buffer.lines().enumerate() {
        let should_replace = match (start, end) {
            (0, 0) => true,                    // Replace all lines
            (0, end) => i + 1 == end as usize, // Replace only at end
            (start, end) if start >= end => i + 1 == start as usize,
            (start, end) => i + 1 >= start as usize && i + 1 <= end as usize,
        };
        let new_line = if should_replace {
            match cmd.flag.as_str() {
                "g" => l
                    .to_string()
                    .replace(cmd.options.get(0).unwrap(), cmd.options.get(1).unwrap()),
                _ => l.to_string().replacen(
                    cmd.options.get(0).unwrap(),
                    cmd.options.get(1).unwrap(),
                    1,
                ),
            }
        } else {
            l.to_string()
        };
        println!("{}", new_line)
    }
    Ok(())
}

fn eval_script(cmd: ScriptCommands, fb: &FilesBuffer) -> Result<(), String> {
    let buffer = fb.files.join("\n").to_string();
    match cmd.cmd.as_str() {
        "s" => eval_s_command(&cmd, &buffer)?,
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
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf).unwrap();
            FilesBuffer { files: vec![buf] }
        }
        Some(files) => {
            let mut buffers = vec![];
            for f in files {
                let file = fs::read_to_string(f).unwrap_or_else(|err| {
                    eprintln!("{}: {}", f, err);
                    process::exit(1);
                });
                buffers.push(file);
            }
            FilesBuffer { files: buffers }
        }
    };
    let cmd = parse_script(script);
    eval_script(
        cmd.unwrap_or_else(|err| {
            eprint!("{}", err);
            process::exit(1);
        }),
        &fb,
    )
    .unwrap();
}

fn handle_args() {
    let args: Vec<String> = args().skip(1).collect();
    if args.is_empty() {
        help();
        process::exit(0);
    }
    for arg in &args {
        match arg.as_str() {
            "--help" | "-h" => {
                help();
                process::exit(0);
            }
            "--version" | "-v" => {
                version();
                process::exit(0);
            }
            flag if flag.starts_with("--") || flag.starts_with("-") => {
                help();
                process::exit(1);
            }
            _ => continue,
        }
    }
    handle_script(&args[0], args.get(1..));
}

fn main() {
    handle_args()
}
