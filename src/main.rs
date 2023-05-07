use home::home_dir;
use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }
    
    // Locate the user's home directory
    if let Some(home_path) = home_dir() {
        println!("User's home directory: {:?}", home_path);
        // Do something with the home directory path
    } else {
        eprintln!("Failed to locate user's home directory");
        // Handle the error case
    }
}