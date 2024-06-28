use clap::{Parser, ValueEnum};

#[derive(Parser)]
#[command(name = "MyApp")]
#[command(version = "0.1")]
#[command(about = "RC ground vehicle control software\n Use : \n 0 for vehicle \n 1 for client \n 2 for server", long_about = None)]
struct Cli {
    #[arg(value_enum)]
    operation_mode:Mode,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Mode {
    #[value(name = "0")]
    Zero,
    #[value(name = "1")]
    One,
    #[value(name = "2")]
    Two,
}

fn main() {
    let cli = Cli::parse();

    match cli.operation_mode {
        Mode::Zero => {
            println!("Mode 0");
        }
        Mode::One => {
            println!("Mode 1");
        }
        Mode::Two => {            
            println!("Mode 2");
        }
    }
}