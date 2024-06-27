mod connectivity;
mod subroutines;

fn main() {

    // inputs variables
    let mut accelerator: f32 = 0.0;
    let mut stearing_wheels: f32 = 0.0;
    let mut is_using_break: bool = false;
    let mut is_going_forward: bool = false;

    // input system, used in the loop
    let mut input_system = connectivity::setup();

    let thrl:f32 = 1.0;
    let wheel:f32 = 0.75;
    let mut left:f32 = 0.0;
    let mut right:f32 =0.0;


    //input::input_test();
    subroutines::input_2_tank(thrl, wheel, 
        &mut left, &mut right);

    println!("{},{}", left, right);

    

    loop {
        // update inputs values
        input_system = connectivity::update(
            input_system,
            &mut accelerator,
            &mut stearing_wheels,
            &mut is_using_break,
            &mut is_going_forward
        );
        // Test for trifoil:
        // println!("accelerator: {}, stearing_wheels: {}, is_using_break: {}, is_going_forward: {}", accelerator, stearing_wheels, is_using_break, is_going_forward);
    }
}

