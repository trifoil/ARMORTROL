use crate::inputs;
use crate::config;
use crate::networking;
use crate::subroutines;
use std::fmt::Display;

pub fn run() {
    println!("Client Mode");

    // Network variables
    let host = "192.168.1.160".to_string();
    let port: u16 = 8069;

    // Input variables
    let mut accelerator: f32 = -1.0;
    let mut steering_wheel: f32 = 0.0;
    let mut is_using_brake: bool = false;
    let mut is_going_forward: bool = false;

    // Input system, used in the loop
    let mut input_system = inputs::setup();

    // Output variables
    let mut left: f32 = 0.0;
    let mut right: f32 = 0.0;

    // Establish the initial connection
    let mut stream = match networking::connect_to_server(&host, &port) {
        Ok(stream) => stream,
        Err(e) => {
            println!("Error connecting to server: {}", e);
            return;
        }
    };

    loop {
        // Update input values
        input_system = inputs::update(
            input_system,
            &mut accelerator,
            &mut steering_wheel,
            &mut is_using_brake,
            &mut is_going_forward
        );

        let accelerator = subroutines::convert_range(accelerator);

        subroutines::input_2_tank(accelerator, steering_wheel, &mut left, &mut right);

        let values: Vec<&dyn Display> = vec![&"A", &left, &"B", &right];
        let concatenated = subroutines::concatenate(&values);
        println!("{}", concatenated);

        // Attempt to send data
        if let Err(e) = networking::send_data(&mut stream, &concatenated) {
            println!("Error sending data: {}", e);
            // Attempt to reconnect if sending fails
            match networking::connect_to_server(&host, &port) {
                Ok(new_stream) => {
                    stream = new_stream;
                }
                Err(e) => {
                    println!("Error reconnecting to server: {}", e);
                    return;
                }
            }
        }
    }
}


// use crate::inputs;
// use crate::config;
// use crate::networking;
// use crate::subroutines;
// use std::fmt::Display;


// pub fn run() {
//     println!("Client Mode");

//     // network variables
//     let host = "192.168.1.160".to_string();
//     let port:u16 = 8069; 

//     // inputs variables
//     let mut accelerator: f32 = -1.0;
//     let mut steering_wheel: f32 = 0.0;
//     let mut is_using_brake: bool = false;
//     let mut is_going_forward: bool = false;

//     // input system, used in the loop
//     let mut input_system = inputs::setup();

//     // output variables
//     let mut left:f32 = 0.0;
//     let mut right:f32 =0.0;

//     loop {
//         // update inputs values
//         input_system = inputs::update(
//             input_system,
//             &mut accelerator,
//             &mut steering_wheel,
//             &mut is_using_brake,
//             &mut is_going_forward
//         );
//         //print!("{}", accelerator);
//         let accelerator = subroutines::convert_range(accelerator);

//         //println!("{}", accelerator);
//         subroutines::input_2_tank(accelerator, steering_wheel,
//             &mut left, &mut right);
        
//         let values: Vec<&dyn Display> = vec![&"A", &left,&"B", &right];
//         let concatenated = subroutines::concatenate(&values);
//         println!("{}", concatenated);

//         //networking::run(&host, &port, &concatenated);

//     }
// }