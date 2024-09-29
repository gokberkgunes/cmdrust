use clap::Parser;
use std::fs::File;
use std::error::Error;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, clap::Parser)]
#[command(author, version)]
#[command(about = "rust clone of head", long_about = None)]

struct Args {
    #[arg(required = false, value_name = "FILE", default_value = "-")]
    files: Vec<String>,
    // https://docs.rs/clap/latest/clap/struct.Arg.html#method.conflicts_with
    // NOTE: Defining a conflict is two-way, but does not need to defined for both arguments (i.e.
    // if A conflicts with B, defining A.conflicts_with(B) is sufficient. You do not need to also
    // do B.conflicts_with
    //u32 could be used. GNU version accepts negative values
    #[arg(
        required = false,
        short = 'n',
        long = "lines",
        value_name = "LINES",
        default_value_t = 10,
        conflicts_with = "bytes",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    lines: u64,

    #[arg(
        required = false,
        short = 'c',
        long = "bytes",
        value_name = "BYTES",
        conflicts_with = "lines",
        value_parser = clap::value_parser!(u64).range(1..),
    )]
    bytes: Option<u64>, // We to use Option since there is no default value, I guess.
}

fn main() {
    let args = Args::parse();

    if let Err(e) = run(args) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    } 
    //Debug
    //println!("{:?}", args.lines);
}


fn run(args: Args) -> MyResult<()> {
    let number_of_files = args.files.len();


    for (file_no, filename) in args.files.iter().enumerate() {
        if number_of_files > 1 {
            if file_no > 0 {
                println!("\n==> {filename} <==");
            } else {
                println!("==> {filename} <==");
            }
        } 
        match open(filename) {
            Err(e) => eprintln!("{filename}: {e}"),
            Ok(file) => {
                match args.bytes {
                    Some(v) => {
                        read_write_bytes_buffered(file, v);
                    }
                    None => {
                        read_write_str_win_style(file, args.lines);
                        // This loop doesn't will not print \r because println! doesn't know
                        //for (line_no, line) in file.lines().enumerate() {
                        //    // We do casts here, if line_no exceeds u64, it'll set to u64::MAX. Maybe cast
                        //    // args.lines to usize befor entering to the loop. (At the moment, we cannot
                        //    // initialize args.lines as usize because clap::value_parser! gives error.)
                        //    if (line_no as u64) >= args.lines {
                        //        break;
                        //    }
                        //    match line {
                        //        Ok(v) => {
                        //            println!("{}", v);

                        //        },
                        //        Err(e) => eprintln!("Error reading line: {}", e),
                        //    }
                        //}
                    }
                }
            }
        }
    }
    Ok(())
}

fn read_write_str_win_style(mut file: Box<dyn BufRead>, total_lines: u64) {
    let mut str_to_read_line = String::new();
    for _ in 0..total_lines {

        /* Read all bytes until a newline (the 0xA byte) is reached, and append them to the
        * provided String buffer.
        * Previous content of the buffer will be preserved. To avoid appending to the buffer, you
        * need to clear it first.
        * This function will read bytes from the underlying stream until the newline delimiter (the
        * 0xA byte) or EOF is found. Once found, all bytes up to, and including, the delimiter (if
        * found) will be appended to buf.
        *
        * If successful, this function will return the total number of bytes read.
        *
        * If this function returns Ok(0), the stream has reached EOF.
        */
        match file.read_line(&mut str_to_read_line) {
            Ok(0) => break,
            Err(err) => eprintln!("{err}"),
            Ok(_) => {
                print!("{str_to_read_line}");
                str_to_read_line.clear();
            }
        }
    }
}


// This function reads and prints at maximum of 1 kilobytes of file. This is to avoid reading every
// single byte to memory and printing it at once. This is done out of instinct, it is possible that
// there is no need to do such an operation.
fn read_write_bytes_buffered(mut file: Box<dyn BufRead>, total_bytes: u64) {
    let buffered_bytes = 1024; // read 1kb at a time
    let mut buffer = vec![0; buffered_bytes as usize];
    let mut bytes_to_read = total_bytes;

    while bytes_to_read > buffered_bytes {
        match file.read(&mut buffer) {
            // [..v] is needed because if line is empty, we print \0.
            Ok(v) => print!("{}", String::from_utf8_lossy(&buffer[..v])),
            Err(e) => eprintln!("ERROR: {}", e),
        }
        bytes_to_read = bytes_to_read - buffered_bytes;
    }
    // Here, we either
    // - flush last bytes left
    // - or read what is needed since bytes_to_read is small
    buffer = vec![0; bytes_to_read as usize];
    match file.read(&mut buffer) {
        // v means total bytes read? [..v] is needed because if 0 bytes read, we mistakenly print \0.
        Ok(v) => print!("{}", String::from_utf8_lossy(&buffer[..v])),
        Err(e) => eprintln!("ERROR: {}", e),
    }
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    // We match to filename, if - is given we open up standard input. Moreover, we're buffering
    // a chunk of data to improve reading performance instead of reading byte by byte.
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

// This function can be replaced with clap's own parser.
// fn parse_positive_int(val: &str) -> MyResult<u32> {
//     match val.parse::<u32>() {
//         Ok(num) if num > 0 => Ok(num),
//         Ok(_) => Err(From::from(val)), // when equal to 0
//         _ => Err(From::from(val)), // when not a u32,u64 (usize)
//     }
//     // How to return MyResult<usize> ?
// }

//#[test]
//fn test_parse_positive_int() {
//
//    // 3 must work, it's a positive integer
//    let res = parse_positive_int("3");
//    assert!(res.is_ok());
//    assert_eq!(res.unwrap(), 3);
//    // a string should not wokr
//    let res = parse_positive_int("foo");
//    assert!(res.is_err());
//    assert_eq!(res.unwrap_err().to_string(), "foo".to_string());
//
//    // Assure 0 is an error it's not positive.
//    let res = parse_positive_int("0");
//    assert!(res.is_err());
//    assert_eq!(res.unwrap_err().to_string(), "0".to_string());
//
//    // Assure 3.1415 is an error it's not an integer.
//    let res = parse_positive_int("3.1415");
//    assert!(res.is_err());
//    assert_eq!(res.unwrap_err().to_string(), "3.1415".to_string());
//
//}
