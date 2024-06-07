mod subroutines;

fn main() {
    subroutines::greet();
    
    let zero_turn_tuple = subroutines::wheel_2_zero_turn_tuple();
    println!("{:?}", zero_turn_tuple);

    let mut var_test = 0;
    subroutines::wheel_2_zero_turn_reference(&mut var_test,5);
    println!("{}", var_test);
}