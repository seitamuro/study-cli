use structopt::StructOpt;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

#[derive(StructOpt, Debug)]
struct Cli {
    pattern: String,
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let f = File::open(args.path).expect("could not read file");
    let reader = BufReader::new(f);

    for line in reader.lines() {
        let line = line.unwrap_or("".to_string());
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
}
