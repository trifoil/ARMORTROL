use std::fmt::Display;

// pub fn input_2_tank(accelerator: f32,steering_wheel: f32, 
//     left_output: &mut f32, right_output: &mut f32){
//     *left_output = (steering_wheel + 1.0 - steering_wheel.abs()) * accelerator;
//     *right_output = (-steering_wheel + 1.0 - steering_wheel.abs()) * accelerator;
// }

// float inversion
pub fn inversion_float(x: f32) -> f32 {
    -x
}

// range from (-1 to 1) to (0 to 1)
pub fn convert_range(x: f32) -> f32 {
    (x + 1.0) / 2.0
}


pub fn concatenate<T: Display>(args: &[T]) -> String {
    let mut result = String::new();
    for arg in args {
        result.push_str(&arg.to_string());
    }
    result
}

// pub fn unit_2_arduino(input:f32, output: &mut i32){
//     let temp = (255.0 / 2.0) * (input + 1.0);
//     let temp = temp as i32;
//     *output = temp;
// }

pub fn input_2_tank(throttle_input:f32,wheel_input:f32, 
    left_output: &mut f32, right_output: &mut f32){
        
    if wheel_input == 0.0 {
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