use regex::Regex;
use std::{fs, process};

pub struct Config<'a> {
    pub query: &'a str,
    pub file_path: &'a str,
    pub case: bool,
    pub regex: bool,
}

struct Flags {
    case: bool,
    regex: bool,
}

impl Flags {
    fn cheak_args(args: &[String]) -> Self {
        let mut case = false;
        let mut regep = false;

        for i in args.iter() {
            let i = i.as_str();
            match i {
                "-c" => case = true,
                "-r" => regep = true,
                _ => (),
            }
        }

        Self { case, regex: regep }
    }
}

#[cfg(target_os = "linux")]
const NUM_ARGS: usize = 3;

#[cfg(target_os = "windows")]
const NUM_ARGS: usize = 2;

#[cfg(target_os = "mac")]
const NUM_ARGS: usize = 2;

impl<'a> Config<'a> {
    pub fn parce_args(args: &'a [String]) -> Self {
        let flag = Flags::cheak_args(args);
        if args.len() <= NUM_ARGS {
            eprintln!("There must be greater then or equal to {NUM_ARGS} arguments.");
            process::exit(2)
        } else {
            Self {
                query: &args[2],
                file_path: &args[1],
                case: flag.case,
                regex: flag.regex,
            }
        }
    }

    pub fn read_file(&self) -> String {
        fs::read_to_string(self.file_path).unwrap_or_else(|err| {
            eprintln!("An error has ocurred ({err})");
            process::exit(2);
        })
    }

    pub fn search(&self) -> Vec<String> {
        let mut result = Vec::new();
        let mut query_ = Regex::new("").unwrap_or_else(|_| process::exit(255));
        let contents = self.read_file();

        if self.regex {
            query_ = Regex::new(self.query).unwrap_or_else(|err| {
                println!("An error has ocurred ({})", err);
                process::exit(17)
            });
        }

        if !self.case {
            contents.lines().for_each(|line| {
                if self.regex {
                    if query_.is_match(line) {
                        result.push(line.to_string());
                    }
                } else if line.contains(self.query) {
                    result.push(line.to_string());
                }
            });
        } else {
            contents.lines().for_each(|line| {
                if self.regex {
                    if query_.is_match(line) {
                        result.push(line.to_string());
                    }
                } else if line.to_lowercase().contains(&self.query.to_lowercase()) {
                    result.push(line.to_string());
                }
            });
        }
        result
    }
}
