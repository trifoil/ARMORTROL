mod input;
mod subroutines;

fn main() {
    let thrl:f32 = 1.0;
    let wheel:f32 = 0.75;
    let mut left:f32 = 0.0;
    let mut right:f32 =0.0;


    //input::input_test();
    subroutines::input_2_tank(thrl, wheel, 
        &mut left, &mut right);

    println!("{},{}", left, right);
}

