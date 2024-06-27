mod inputs;
mod subroutines;

fn main() {

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

