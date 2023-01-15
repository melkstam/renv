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
    /// List a specific environment variable
    variable: Option<String>,

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

    match args.variable {
        Some(_) => print_single_envvar(&args),
        None => print_all_envvars(&args),
    }
}

fn print_all_envvars(_: &Args) {
    let mut envvars: Vec<(String, String)> = env::vars().collect();

    envvars.sort_by(|a, b| a.0.cmp(&b.0));

    for (key, value) in envvars {
        println!("{}={}", key.green(), value.blue());
    }
}

fn print_single_envvar(args: &Args) {
    let var = args.variable.as_ref().expect("variable name not specified");

    let value = env::var(var).expect("variable not found");

    println!("{}", value.blue());
}
