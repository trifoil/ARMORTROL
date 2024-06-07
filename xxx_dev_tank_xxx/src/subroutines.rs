pub fn greet() {
    println!("EUROBEAT!!!");
}

pub fn wheel_2_zero_turn_tuple() -> (i32,i32) {
    println!("wheel_2_zero_turn");
    let valeur_a = 2;
    let valeur_b = 3;
    (valeur_a, valeur_b) //NO SEMICOLON!!!
}

pub fn wheel_2_zero_turn_reference(var: &mut i32, new_value: i32) {
    *var = new_value;
}