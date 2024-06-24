use std::{
    fs::{self, File},
    io::{self, BufRead},
};

use clap::Parser;

#[derive(Parser)]
#[command(name = "wc-rs")]
#[command(version = "0.1.0")]
#[command(about="The wc-rs utility displays the count of lines, words, characters, and bytes contained in each input file", long_about=None)]
struct CLI {
    /// The number of bytes in each input file is written to the
    /// standard output. This will cancel out any prior usage of the
    /// -m option.
    #[arg(short = 'c')]
    bytes: bool,

    /// The number of lines in each input file is written to the
    /// standard output.
    #[arg(short)]
    lines: bool,

    /// The number of words in each input file is written to the
    /// standard output.
    #[arg(short)]
    words: bool,

    /// The number of characters in each input file is written to the
    /// standard output.
    #[arg(short = 'm')]
    chars: bool,
    files: Option<Vec<String>>,
}

#[derive(Default)]
struct Output {
    bytes: u64,
    lines: u64,
    words: u64,
    chars: u64,
    file: Option<String>,
}

impl Output {
    fn new(file: &str) -> Self {
        let mut out = Self::default();
        out.file = Some(String::from(file));

        out
    }
}

fn main() -> io::Result<()> {
    let cli = CLI::parse();
    let mut output: Vec<Output> = vec![];

    if let Some(files) = &cli.files {
        for file in files.iter() {
            let f = File::open(file)?;
            let reader = io::BufReader::new(f);

            let mut out = Output::new(file);
            out.bytes = fs::metadata(file)?.len();

            process_lines(reader, false, &mut out);
            output.push(out);
        }
    } else {
        let stdin = io::stdin();
        let reader = stdin.lock();
        let mut out = Output::default();

        process_lines(reader, true, &mut out);
        output.push(out);
    }

    print_output(&cli, output);

    Ok(())
}

fn process_lines<T: BufRead>(reader: T, from_stdin: bool, out: &mut Output) {
    for line in reader.lines() {
        match line {
            Ok(input) => {
                out.lines += 1;

                if from_stdin {
                    out.bytes += input.as_bytes().len() as u64;
                }

                let line_words: Vec<_> = input.split_terminator(" ").collect();
                out.words += line_words.len() as u64;

                line_words.iter().for_each(|w| out.chars += w.len() as u64);
            }
            Err(e) => eprintln!("{}", e),
        }
    }
}

fn print_output(cli: &CLI, output: Vec<Output>) {
    let mut print_all = false;
    let mut total = Output::default();

    for out in &output {
        if !cli.lines && !cli.words && !cli.chars && !cli.bytes {
            print_all = true;
            if let Some(f) = &out.file {
                print!("\t{}\t{}\t{}\t{}\n", out.lines, out.words, out.bytes, f);
            } else {
                print!("\t{}\t{}\t{}\n", out.lines, out.words, out.bytes);
            }
        } else {
            if cli.lines {
                print!("\t{}", out.lines);
            }
            if cli.words {
                print!("\t{}", out.words);
            }
            if cli.bytes {
                print!("\t{}", out.bytes);
            } else if cli.chars {
                print!("\t{}", out.chars);
            }
            if let Some(f) = &out.file {
                print!("\t{}\n", f);
            } else {
                println!();
            }
        }
    
        total.lines += out.lines;
        total.words += out.words;
        total.bytes += out.bytes;
        total.chars += out.chars;
    }

    if output.len() > 1 {
        if print_all {
            print!("\t{}\t{}\t{}\t{}\n", total.lines, total.words, total.bytes, "total");
        } else {
            if cli.lines {
                print!("\t{}", total.lines);
            }
            if cli.words {
                print!("\t{}", total.words);
            }
            if cli.bytes {
                print!("\t{}", total.bytes);
            } else if cli.chars {
                print!("\t{}", total.chars);
            }
            
            println!("\ttotal");
        }
    }
}
