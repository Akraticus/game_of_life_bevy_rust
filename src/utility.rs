use bevy::prelude::*;

// run condition to test for pressing of a single keycode
pub fn keycode_just_pressed(key:KeyCode) -> impl FnMut(Res<Input<KeyCode>>) -> bool {
    move |input|{
        return input.just_pressed(key)
    }
}