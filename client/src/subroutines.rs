pub fn input_2_tank(accelerator: f32,steering_wheels: f32, 
    left_output: &mut f32, right_output: &mut f32){
    *left_output = (steering_wheels + 1.0 - steering_wheels.abs()) * accelerator;
    *right_output = (-steering_wheels + 1.0 - steering_wheels.abs()) * accelerator;
}

// pub fn unit_2_arduino(input:f32, output: &mut i32){
//     let temp = (255.0 / 2.0) * (input + 1.0);
//     let temp = temp as i32;
//     *output = temp;
// }