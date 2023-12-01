use clap::{Arg, ArgAction, Command, Parser};

/// Just a simple CLI tool to keep track of hours for my work.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Begin a new task
    #[arg(short, long, required_unless_present="end")]
    begin: bool,

    /// End the current task
    #[arg(short, long)]
    end: bool,

    /// Specify the issue
    #[arg(short, long)]
    issue: String,

    /// Add a more detailed description
    #[arg(short, long)]
    description: Option<String>
}

fn main() {
    let args = Args::parse();
    println!("Hello, world!");
}
