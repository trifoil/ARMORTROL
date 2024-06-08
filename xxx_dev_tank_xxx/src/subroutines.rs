pub fn wheel_input_correction(wheel_input: f32, wheel_input_corrected: &mut f32) {
    *wheel_input_corrected = wheel_input * 2.0 - 255.0;
}

pub fn acc_brake_direction_2_throttle(accelerator_input:f32, brake_input:f32, 
    reverse_or_not_input:bool, throttle: &mut f32) {
        if brake_input < 20.0 {
            if reverse_or_not_input == false {
                *throttle = accelerator_input;
            } else {
                *throttle = - accelerator_input;
            }
        } else {
            *throttle = 0.0;
        }
}

pub fn wheel_2_zero_turn_reference(wheel_input_corrected: f32, throttle: f32, 
    left_track_motor: &mut f32,  right_track_motor: &mut f32) {
    
    if wheel_input_corrected < 0.0 {
        *right_track_motor = throttle;
        if wheel_input_corrected < -127.5 {
            *left_track_motor = throttle * (255.0 + 2.0 * wheel_input_corrected) / 255.0;
        } else {
            *left_track_motor = throttle * (wheel_input_corrected + 128.0) * 2.0 / 255.0;
        }
        
    } else if wheel_input_corrected > 0.0 {
        *left_track_motor = throttle;
        if wheel_input_corrected > 127.5 {
            *right_track_motor = throttle * ( - ( 2.0 * wheel_input_corrected -255.0)) / 255.0;
        } else {
            *right_track_motor = throttle * (-((wheel_input_corrected * 2.0 ) - 255.0)) / 255.0;
        }

    } else {
        *left_track_motor = throttle;
        *right_track_motor = throttle;
    }
}
