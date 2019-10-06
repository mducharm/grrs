use std::fs::File;
use std::io::{BufRead, BufReader, Error, Read};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::from_args();
    println!("{:?}", args);

    let content = File::open(&args.path)?;
    let mut reader = BufReader::new(content);

    for line in reader.lines() {
        let s: String = line.unwrap();
        if s.contains(&args.pattern) {
            println!("{}", s)
        }
    }

    Ok(())
}
