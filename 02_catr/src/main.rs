use clap::Parser;
use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead, BufReader};

// A result type is able to return any type of Ok, represented as a generic T.
type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, clap::Parser)]
#[command(name = "catr", author = "gg", version = "1.0.0")]
#[command(about = "rust clone of cat", long_about = None)]

struct Args {
    #[arg(required = true, value_name = "FILE", default_value = "-")]
    files: Vec<String>,

    // https://docs.rs/clap/latest/clap/struct.Arg.html#method.conflicts_with
    // NOTE: Defining a conflict is two-way, but does not need to defined for both arguments (i.e.
    // if A conflicts with B, defining A.conflicts_with(B) is sufficient. You do not need to also
    // do B.conflicts_with
    #[arg(short = 'n', long = "number", conflicts_with = "number_nonblank_lines")]
    number_lines: bool,

    #[arg(short = 'b', long = "number-nonblank")]
    number_nonblank_lines: bool,
}

fn main() {
    // The functions in src/lib.rs are available through the crate catr.
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e); // Error print line prints to STDERR.
        std::process::exit(1); // Here 1 will be run if an error occured, if not $? will be 0.
    } 
}


fn run(args: Args) -> MyResult<()> {
    // If required arguments are not provided, dbg! will not work.
    //dbg!(&args);

    // Below approach is non-sense since line_no belongs to code block.
    // I believe there is way to add if block to encapsulate for line in reader.line()
    // to save memory.
    //if args.number_lines || args.number_nonblank_lines {
    //    let mut line_no: u32 = 1;
    //} 
    let mut line_no: u32 = 1;

    for filename in args.files {
        match open(&filename) {
            Err(e) => eprintln!("Failed to open {}: {}", filename, e),
            Ok(reader) => {
                // println!("Opened {}!\nReading its contents:", filename);
                for line in reader.lines() {
                    match line {
                        Ok(v) => {
                            if args.number_lines {
                                println!("{:>6}\t{}", line_no, v);
                                line_no += 1;
                            } else if args.number_nonblank_lines {
                                if v.is_empty() {
                                    println!("");
                                } else {
                                    println!("{:>6}\t{}", line_no, v);
                                    line_no += 1;
                                }

                            } else {
                                println!("{}", v);
                            } 
                        },
                        Err(e) => eprintln!("Error reading line: {}", e),
                    }
                    
                }
            }
        }
    }
    Ok(())
}

// Box is needed because size has to be known at compile time. Box is used to allocate mem.
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    // We match to filename, if - is given we open up standard input. Moreover, we're buffering
    // a chunk of data to improve reading performance instead of reading byte by byte.
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

//fn error_run(args: Args) -> MyResult<()> {
//    Err("An error occured.".into())
//}
