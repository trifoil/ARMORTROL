mod subroutines;

fn main() {
    subroutines::greet();
    let zero_turn = subroutines::wheel_2_zero_turn();
    println!("{:?}", zero_turn);
}
