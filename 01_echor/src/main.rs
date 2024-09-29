use clap::Parser;

#[derive(Parser)]
#[command(name = "echorust", author = "gg")]
#[command(version = "1.0.0")]
#[command(about = "prints given arguments", long_about = None)]

// Get arguments using clap's advanced options here.
//
// We get two arguments: `text` which is a required vector of strings.
//                       `no_new_line` which is an optional boolean.
struct Args {
    #[arg(required = true)]
    text: Vec<String>,

    // You can leave short = 'n' as simply short to assign first letter.
    #[arg(short = 'n', long)]
    no_new_line: bool,
}


fn main() {
    // Parse the arguments and put them to variable args.
    let args = Args::parse();

    // if args.no_new_line {
    //     print!("{}", args.text.join(" "));
    // } else {
    //     println!("{}", args.text.join(" "));
    // }

    // We can put this if statement directly into print! too, awesome!
    let last_char = if args.no_new_line { "" } else { "\n" };

    // We do not use println! since we do not want a new line automatically.
    print!("{}{}", args.text.join(" "), last_char)
}
