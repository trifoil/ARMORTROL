mod subroutines;

fn main() {

    //var declaration and init
    let mut wheel_input = 0.0;
    let mut wheel_input_corrected:f32 = 0.0;

    let mut accelerator_input:f32 = 0.0;
    let mut brake_input:f32 = 0.0;
    let mut reverse_or_not_input:f32 = 0.0;

    let mut throttle:f32 = 0.0;

    let mut left_track_motor:f32 = 0.0;
    let mut right_track_motor:f32 = 0.0;

    //unit test saying hello (ok)
    subroutines::greet();
    
    //wheel input correction (ok)
    subroutines:: wheel_input_correction(wheel_input_corrected, &mut wheel_input_corrected);
    println!("{}", wheel_input_corrected);

    //zero turn implementation
    subroutines::wheel_2_zero_turn_reference(wheel_input_corrected, throttle, &mut left_track_motor,&mut right_track_motor);
    println!("{}", left_track_motor);
    println!("{}", right_track_motor);
}

// A  IMPLEMENTER!!!
//il faut que quand on lache l'accélérateur, le tank diminue de vitesse lentement 
//si on freine, alors, on diminue plus vite en proportion du freinage