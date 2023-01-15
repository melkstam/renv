use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::env;
use tabled::{
    format::Format,
    object::{Columns, Rows},
    Modify, Style, Table, Tabled, Width,
};

#[derive(Tabled)]
#[tabled(rename_all = "PascalCase")]
struct EnvVar {
    variable: String,
    value: String,
}

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

    /// Print in a table format
    #[arg(long)]
    table: bool,
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

fn print_all_envvars(args: &Args) {
    let mut envvars = env::vars()
        .map(|v| EnvVar {
            variable: v.0,
            value: v.1,
        })
        .collect::<Vec<_>>();

    envvars.sort_by(|a, b| a.variable.cmp(&b.variable));

    match args.table {
        true => print_all_table(&envvars),
        false => print_all_key_value(&envvars),
    }
}

fn print_all_key_value(envvars: &Vec<EnvVar>) {
    for var in envvars {
        println!("{}={}", var.variable.green(), var.value.blue());
    }
}

fn print_all_table(envvars: &Vec<EnvVar>) {
    let mut table = Table::new(envvars);
    table
        .with(Style::sharp())
        .with(Modify::new(Rows::single(0)).with(Format::new(|s| s.bold().to_string())))
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(80)))
        .with(Modify::new(Columns::single(0)).with(Format::new(|s| s.green().to_string())))
        .with(Modify::new(Columns::single(1)).with(Format::new(|s| s.blue().to_string())));

    println!("{}", table);
}

fn print_single_envvar(args: &Args) {
    let var = args.variable.as_ref().expect("variable name not specified");

    let value = env::var(var).expect("variable not found");

    println!("{}", value.blue());
}
