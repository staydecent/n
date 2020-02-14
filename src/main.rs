use std::fs::read_to_string;
use std::path::PathBuf;
use structopt::StructOpt;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(StructOpt)]
struct Cli {
    /// The pattern to look for
    pattern: String,

    /// The path to the file to read
    #[structopt(parse(from_os_str))]
    file: PathBuf,
}

fn main() {
    let args = Cli::from_args();
    let content = read_to_string(&args.file).expect("could not read file");

    content
        .lines()
        .filter(|x| x.contains(&args.pattern))
        .for_each(|x| println!("{}", x));
}
