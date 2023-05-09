mod yaml;
mod shell;
use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Optional profile to select
    #[arg(short, long, value_name = "STRING")]
    profile: Option<String>,

    /// Optional cluster to switch to
    #[arg(short, long, value_name = "STRING")]
    cluster: Option<PathBuf>,

    /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Generates aws and k8s config files
    Generate {
        /// lists test values
        #[arg(short, long)]
        list: bool,
    },
}

fn main() {

    let mut config = yaml::get_config();

    match shell::command("aws --version", 3){
        Ok(output) => {
            println!("output: \n{output}" );
        },
        Err(e) => {
            println!("error: \n{e}");
        }
    }

    let cli = Cli::parse();

    // You can check the value provided by positional arguments, or option arguments
    if let Some(name) = cli.profile.as_deref() {
        println!("Value for name: {name}");
    }

    if let Some(config_path) = cli.cluster.as_deref() {
        println!("Value for config: {}", config_path.display());
    }

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    // match cli.debug {
    //     0 => println!("Debug mode is off"),
    //     1 => println!("Debug mode is kind of on"),
    //     2 => println!("Debug mode is on"),
    //     _ => println!("Don't be crazy"),
    // }

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Generate { list }) => {
            if *list {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
        None => {}
    }

    // Continued program logic goes here...
}