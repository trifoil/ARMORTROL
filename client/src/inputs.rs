use gilrs::{Gilrs, Event, EventType, Axis, Button};

pub fn setup() -> Gilrs {
    let input_system: Gilrs = Gilrs::new().unwrap();
    return input_system;
}

pub fn update(
    mut input_system: Gilrs,
    accelerator: &mut f32,
    stearing_wheels: &mut f32,
    is_using_brake: &mut bool,
    is_going_forward: &mut bool
) -> Gilrs {

    while let Some(Event { id, event, time }) = input_system.next_event() {
        match event  {
            EventType::AxisChanged(axis, value, _) => {
                if matches!(axis, Axis::LeftStickX) {
                    *stearing_wheels = value;
                }
            },
            EventType::ButtonPressed(button, _) => {
                if matches!(button, Button::East) {
                    *is_going_forward = !*is_going_forward
                }
            },
            EventType::ButtonChanged(button, value, _) => {
                if matches!(button, Button::LeftTrigger2) {
                    *is_using_brake = value >= 0.20;
                } else if matches!(button, Button::RightTrigger2) {
                    *accelerator = value;
                }
            },
            _ => {}
        }
    }
    return input_system
}