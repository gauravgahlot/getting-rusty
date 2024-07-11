use std::{
    env,
    fs::File,
    io::{self, Read},
};

mod lexer;
mod parser;

fn main() -> io::Result<()> {
    /*
        let input = read_input()?;
        println!("INPUT: {}", input);
    */

    let tokens = lexer::tokenize(String::from("{}"));

    for token in &tokens {
        println!("{token:?}");
    }

    Ok(())
}

fn read_input() -> io::Result<String> {
    let mut input = String::new();
    let args: Vec<_> = env::args().skip(1).collect();

    if args.len() > 0 {
        let mut file = File::open(&args[0])?;
        file.read_to_string(&mut input)?;
    } else {
        let mut reader = io::BufReader::new(io::stdin());
        reader.read_to_string(&mut input)?;
    }

    Ok(input)
}
