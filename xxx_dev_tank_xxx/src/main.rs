mod subroutines;
mod connectivity;
fn main() {

    //var declaration and init
    let mut wheel_input:f32 = 130.0;
    let mut wheel_input_corrected:f32 = 0.0;

    let mut accelerator_input:f32 = 200.0;
    let mut brake_input:f32 = 21.0;
    let mut reverse_or_not_input:bool = true;

    let mut throttle:f32 = 255.0;

    let mut left_track_motor:f32 = 0.0;
    let mut right_track_motor:f32 = 0.0;

    println!("---------------------------");
    println!("----- Input variables -----");
    println!("---------------------------");
    println!("Wheel input : {}",wheel_input);
    println!("Accelerator input : {}",accelerator_input);
    println!("Brake input : {}",brake_input);
    println!("Reverse : {}",reverse_or_not_input);
    println!("---------------------------");
    println!("----- Middle variables ----");
    println!("---------------------------");
    println!("Wheel input corrected : {}",wheel_input_corrected);
    println!("Throttle : {}",throttle);
    println!("---------------------------");
    println!("----- Output variables ----");
    println!("---------------------------");
    println!("Left track : {}",left_track_motor);
    println!("Right track : {}",right_track_motor);
    println!("---------------------------");

    //wheel input correction (ok)
    subroutines:: wheel_input_correction(wheel_input, &mut wheel_input_corrected);
    println!("WHEEL CORRECTED : {}", wheel_input_corrected);
    subroutines:: acc_brake_direction_2_throttle(accelerator_input, brake_input, reverse_or_not_input, &mut throttle);
    //zero turn implementation (ok)
    subroutines::wheel_2_zero_turn_reference(wheel_input_corrected, throttle, &mut left_track_motor,&mut right_track_motor);
    println!("LEFT : {}", left_track_motor);
    println!("RIGHT : {}", right_track_motor);
}
