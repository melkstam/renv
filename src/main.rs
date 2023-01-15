use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::env;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum ColoringOption {
    ALWAYS,
    NEVER,
    AUTO,
}

/// Print your environment variables
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// If the output should be colored
    #[arg(value_enum, long, default_value_t = ColoringOption::AUTO)]
    color: ColoringOption,
}

fn main() {
    let args = Args::parse();

    match args.color {
        ColoringOption::ALWAYS => colored::control::set_override(true),
        ColoringOption::NEVER => colored::control::set_override(false),
        ColoringOption::AUTO => (),
    }

    let envvars: Vec<(String, String)> = env::vars().collect();

    for (key, value) in envvars {
        println!("{}={}", key.green(), value.blue());
    }
}
