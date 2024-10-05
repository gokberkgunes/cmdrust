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

    #[arg(short = 'l', long = "lines", default_value_t = false)]
    lines: bool,

    #[arg(short = 'w', long = "words", default_value_t = false)]
    words: bool,

    #[arg(short = 'c', long = "bytes", default_value_t = false)]
    bytes: bool,

    #[arg(short = 'm', long = "chars", conflicts_with = "bytes", default_value_t = false)]
    chars: bool,

}


fn main() {
    let args = Args::parse();
    if let Err(e) = run(args) {
        eprintln!("ERROR: {e}");
        std::process::exit(1);
    }
}

fn run(mut args: Args) -> Result<()> {
    //dbg!(&args);

    // When there are no explicit arguments, every argument will kept as false. 
    // Then, we can set the default behavior as having words, lines and bytes.
    // In book this is done by 
    // - creating a slice [ ... ]
    // - adding an iterator to it to find false values
    if [args.words, args.bytes, args.chars, args.lines]
        .iter()
        .all(|v| v == &false) // .all() tests and if any element is not, returns false
    {
        args.words = true;
        args.bytes = true;
        args.lines = true;
    }

    let mut sum_lines = 0;
    let mut sum_words = 0;
    let mut sum_bytes = 0;

    for filename in &args.files {
        match open(filename) {
            Err(e) => eprintln!("ERROR: {}, {}", filename, e),
            Ok(file) => {
                //println!("{}", count_bytes_buffered(file));
                let line_word_byte = count_buffered(file);
                println!("{}{}{}{}{}",
                    if args.lines { format!("{:>8}",line_word_byte[0]) } else { "".to_string() },
                    if args.words { format!("{:>8}",line_word_byte[1]) } else { "".to_string() },
                    if args.bytes { format!("{:>8}",line_word_byte[2]) } else { "".to_string() },
                    if args.chars { format!("{:>8}",line_word_byte[3]) } else { "".to_string() },
                    if filename != "-" { format!{" {}", filename} } else { "".to_string() },
                );
                sum_lines += line_word_byte[0];
                sum_words += line_word_byte[1];
                sum_bytes += line_word_byte[2];
            }
        }
    }
    if &args.files.len() > &1  {
                println!("{}{}{} total",
                    if args.lines { format!("{:>8}", sum_lines) } else { "".to_string() },
                    if args.words { format!("{:>8}", sum_words) } else { "".to_string() },
                    if args.bytes { format!("{:>8}", sum_bytes) } else { "".to_string() },
                )
    }
    Ok(())
}


fn count_buffered(mut file: Box<dyn BufRead>) -> [usize; 4] {
    let mut buffer = [0; 1024]; // Fill this array over and over. 1kB per turn.
    let mut total_chars: usize = 0;
    let mut total_bytes: usize = 0;
    let mut total_lines: usize = 0;
    let mut total_words: usize = 0;
    let mut in_word: bool = false;
    loop {
        match file.read(&mut buffer) {
            Ok(0) => return [total_lines, total_words, total_bytes, total_chars],
            Ok(v) => {
                total_bytes += v;
                total_chars += String::from_utf8_lossy(&buffer[..v]).len();
                total_lines += buffer[..v].iter().filter(|&&byte| byte == b'\n').count(); 

                // Count characters, C style.
                for &c in &buffer[..v] {
                    if c == b'\n' || c == b' ' || c == b'\t' {
                        if in_word == true  {
                            total_words += 1;
                            in_word = false;
                        }
                    } else {
                            in_word = true;
                    }

                }
                // Below approach fails cause we read 1kB which could cut words into two; thus,
                // counting one word as two. We could've fixed it if we stop reading at whitspace.
                //total_words = std::str::from_utf8(&buffer[..v]).unwrap().split_whitespace().count();
            },
            Err(e) => panic!("ERROR: {e}"),
        }
    }
}

fn open(filename: &str) -> Result<Box<dyn BufRead>>{
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
