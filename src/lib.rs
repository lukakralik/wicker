use clap::{App, Arg};
use std::error::Error;
use std::io::{self, BufRead, BufReader};
use std::fs::File;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;
    let mut line = String::new();

    loop{
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        num_bytes += line_bytes;
        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo{
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wicker")
        .version("0.1.0")
        .author("Luka Kralik")
        .about("Rust version of unix wc command line utility.")
        .arg(
            Arg::with_name("files")
                .value_name("FILES")
                .help("Input file")
                .default_value("-")
                .multiple(true),
            )
        .arg(
            Arg::with_name("bytes")
                .help("Specify number of bytes written to the standart output. Will cancel any previous -m usage.")
                .short("c")
                .long("bytes")
                .takes_value(false)
                .conflicts_with("chars")
                .required(false),
            )
        .arg(
            Arg::with_name("lines")
                .takes_value(false)
                .help("Number of lines written to the standart output")
                .short("l")
                .long("lines")
                .required(false),
            )
        .arg(
            Arg::with_name("chars")
                .value_name("CHARS")
                .required(false)
                .help("The number of characters written to the standart output. Will cancel any previous -c usage.")
                .short("m")
                .long("chars")
                .conflicts_with("bytes"),
            )
        .arg(
            Arg::with_name("words")
                .required(false)
                .long("words")
                .takes_value(false)
                .help("The number of words in each file written to the standart output.")
                .short("w"),
            )
        .get_matches();
    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        // negate each boolean value v from iterator
        lines = true;
        words = true;
        bytes = true;
    }
    Ok(Config{
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(info) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(info.num_lines, config.lines),
                        format_field(info.num_words, config.words),
                        format_field(info.num_bytes, config.bytes),
                        format_field(info.num_chars, config.chars),
                        if filename == "-"{
                            "".to_string()
                        }
                        else {
                            format!(" {}", filename)
                        });
                    total_lines += info.num_lines;
                    total_words += info.num_words;
                    total_bytes += info.num_bytes;
                    total_chars += info.num_chars;
                }
            }
        }
    }
    Ok(())
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    }
    else {
        "".to_string()
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
