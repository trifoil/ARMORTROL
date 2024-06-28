use clap::{Parser, ValueEnum};

mod inputs;
mod subroutines;

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
            println!("Vehicle Mode");
        }
        Mode::One => {
            println!("Client Mode");
                // inputs variables
            let mut accelerator: f32 = 0.0;
            let mut stearing_wheels: f32 = 0.0;
            let mut is_using_brake: bool = false;
            let mut is_going_forward: bool = false;

            // input system, used in the loop
            let mut input_system = inputs::setup();

            // output variables
            let mut left:f32 = 0.0;
            let mut right:f32 =0.0;    

            loop {
                // update inputs values
                input_system = inputs::update(
                    input_system,
                    &mut accelerator,
                    &mut stearing_wheels,
                    &mut is_using_brake,
                    &mut is_going_forward
                );
                
                subroutines::input_2_tank(accelerator, stearing_wheels,
                    &mut left, &mut right);

                // Test for trifoil:
                // println!("left: {}, right: {}, is_using_break: {}, is_going_forward: {}", left, right, is_using_break, is_going_forward);
            }
        }
        Mode::Two => {            
            println!("Server Mode");
        }
    }

}

