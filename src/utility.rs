use bevy::prelude::*;
use std::marker::Send;

// run condition to test for pressing of a single keycode
pub fn keycode_just_pressed(key:KeyCode) -> impl FnMut(Res<Input<KeyCode>>) -> bool {
    move |input|{
        return input.just_pressed(key)
    }
}

// TODO: doesnt work
pub fn send_event<T:Send + Event + Default>(mut event_writer: EventWriter<T>){
    event_writer.send_default();
}