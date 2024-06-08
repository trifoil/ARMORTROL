pub fn greet() {
    println!("F U !!!");
}

pub fn wheel_input_correction(wheel_input: f32, wheel_input_corrected: &mut f32) {
    *wheel_input_corrected = wheel_input * 2.0 -255.0;
}

pub fn wheel_2_zero_turn_reference(wheel_input_corrected: f32, throttle: f32, left_track_motor: &mut f32,  right_track_motor: &mut f32) {
    *left_track_motor = wheel_input_corrected;
    *right_track_motor = throttle;
    //a finir
}