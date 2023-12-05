#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

use colored::{ColoredString, Colorize};
use reqwest;
use semver::Version;
use serde::{Deserialize, Serialize};

mod interpreter;
mod lexer;

pub enum Colors {
    GREEN,
    BLUE,
    BRIGHTGREEN,
    BRIGHTBLUE,
    RESET,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    flags: std::collections::HashMap<String, u32>,
}

fn main() {
    let semversion = "1.1.1";

    // Variables
    let message: &str = "IASM - Interpreted Assembly

Usage: iasm [options] <file>

Options:
  -h, --help     Print help message
  -V, --version  Print version info
      --list     List all available commands
  -v, --verbose  Allow printing of IASM log messages
  -c, --config   Specify a config file that the interpreter will use to interpret your specific circumstances [DOESN'T WORK]
";
    let version: &str = &format!("iasm {} by SaphirDeFeu", semversion);
    let mut loud: bool = false;

    // Fetch and interpret the CLI arguments
    let mut cli_args: Vec<String> = std::env::args().collect();
    cli_args.push(String::from(""));
    for mut i in 1..cli_args.len() {
        match cli_args[i].as_str() {
            "-V" | "--version" => {
                println!("{}", version);
                // Fetch latest version
                louden("CLI".on_yellow(), "Fetching versions", true);
                let url = "https://iasm-version-control.saphirdefeu.repl.co/";

                let response: String = match reqwest::blocking::get(url) {
                    Ok(value) => match value.text() {
                        Ok(textvalue) => textvalue,
                        Err(e) => semversion.to_string(),
                    },
                    Err(e) => semversion.to_string(),
                };

                if let Ok(version) = Version::parse(&response) {
                    let latest_major = version.major;
                    let latest_minor = version.minor;
                    let latest_patch = version.patch;

                    if let Ok(current) = Version::parse(semversion) {
                        let current_major = current.major;
                        let current_minor = current.minor;
                        let current_patch = current.patch;

                        check_version(
                            latest_major,
                            latest_minor,
                            latest_patch,
                            current_major,
                            current_minor,
                            current_patch,
                            &response,
                            &semversion
                        );
                    }
                }
                std::process::exit(0);
            }
            "-h" | "--help" => {
                println!("{}", message);
                std::process::exit(0);
            }
            "-v" | "--verbose" => {
                loud = true;
            }
            "-c" | "--config" => {
                louden("CLI".on_yellow(), "Reading CONFIG flag", loud);
                if cli_args.len() <= i + 2 {
                    throw(
                        "ERR_BAD_ARGUMENTS",
                        "`-c` flag must be followed by a file path!",
                        0x2,
                        file!(),
                        version,
                        line!(),
                        true,
                    );
                }
                let content = match std::fs::read_to_string(&cli_args[i + 1]) {
                    Ok(content) => content,
                    Err(err) => {
                        throw(
                            "ERR_GENERAL",
                            &format!("{}", err),
                            0x1,
                            file!(),
                            version,
                            line!(),
                            false,
                        );
                        std::process::exit(0x1C);
                    }
                };

                let config: Config = match serde_json::from_str(&content) {
                    Ok(config) => config,
                    Err(err) => {
                        throw(
                            "ERR_JSON_PARSE_ERROR",
                            &format!("{}", err),
                            0x4,
                            file!(),
                            version,
                            line!(),
                            false,
                        );
                        std::process::exit(0x1C);
                    }
                };

                println!("{:#?}", config);
                i += 2;
                continue;
            }
            "" => {
                if cli_args.len() == 2 {
                    println!("{}", message);
                    std::process::exit(0);
                }
            }
            _ => {
                let file_path = cli_args[i].as_str();
                if let Ok(metadata) = std::fs::metadata(file_path) {
                    if metadata.is_file() {
                        let content: String =
                            std::fs::read_to_string(file_path).expect("Error reading file");
                        let _ = interpreter::interpret(&content, version, loud);
                    } else {
                        throw(
                            "ERR_FILE_NOT_FOUND",
                            "Given path is not a file!",
                            0x3,
                            file!(),
                            version,
                            line!(),
                            true,
                        );
                    }
                } else {
                    throw(
                        "ERR_FILE_NOT_FOUND",
                        "Unable to find file!",
                        0x3,
                        file!(),
                        version,
                        line!(),
                        true,
                    );
                }
            }
        }
    }
}

pub fn throw(
    err_type: &str,
    err_msg: &str,
    err_code: i32,
    file_path: &str,
    v: &str,
    line: u32,
    exit: bool,
) {
    println!(
        "
{0}
    at {1}:{2}
  type: '{3}', err_type
  code: {4} (0x{4:X})

{5}",
        err_msg, file_path, line, err_type, err_code, v
    );
    if exit {
        std::process::exit(err_code);
    }
}

pub fn louden(debug_type: ColoredString, debug_msg: &str, loud: bool) {
    if !loud {
        return;
    }
    println!("   {0} {1}", debug_type, debug_msg);
}

fn check_version(lmm: u64, lm: u64, lp: u64, cmm: u64, cm: u64, cp: u64, latest_version: &str, version: &str) {
    if lmm <= cmm {
        // Same major version
        if lm <= cm {
            // Same minor version
            if lp <= cp {
                // Same patch version
                println!(
                    " You are running the latest version of iasm ({}) ",
                    version,
                );
                return;
            }
        }
    }
    // One of them is later
    println!(
        " {0} >> {1} (https://github.com/SaphirDeFeu/iasm/releases/tag/v{1}) ",
        "Newer version available".bright_green(),
        latest_version
    );
}
