use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use anyhow::Result;

#[derive(Debug, clap::Parser)]
#[command(author, version)]
#[command(about = "rust clone of wc", long_about = None)]

struct Args {
    #[arg(value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'w', long = "words", default_value = false)]
    words: bool,

    #[arg(short = 'c', long = "bytes", default_value_t = false)]
    bytes: bool,

    #[arg(short = 'm', long = "chars", conflicts_with = "bytes", default_value_t = false)]
    chars: bool,

    #[arg(short = 'l', long = "lines", default_value_t = false)]
    lines: bool,
}


fn main() {
    let mut args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}

fn run(mut args: Args) -> Result<()> {
    dbg!(&args);
    // When there are no explicit arguments, every argument will kept as false. 
    // Then, we can set the default behavior as having words, lines and bytes.
    // In book this is done by 
    // - creating a slice [ ... ]
    // - adding an iterator to it to find false values
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false)
    {
        args.words = true;
        args.byte = true;
        args.lines = true;
    }

    set_variables(args);
    return Ok(())
}



fn open(filename: &str) -> Result<Box<dyn BufRead>>{
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
