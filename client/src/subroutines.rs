pub fn input_2_tank(throttle_input:f32,wheel_input:f32, 
    left_output: &mut f32, right_output: &mut f32){
        
    if wheel_input ==0.0 {
        *left_output = throttle_input;
        *right_output = throttle_input;
    } else if  wheel_input < 0.0 {
        *left_output = throttle_input * (2.0 * wheel_input + 1.0);
        *right_output = throttle_input;
    } else {
        *left_output = throttle_input;
        *right_output = throttle_input * (-2.0 * wheel_input + 1.0);
    }
}