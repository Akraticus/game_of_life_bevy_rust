use bevy::{prelude::*};

#[derive(Resource, PartialEq)]
struct GameState(f32);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(GameState(0.0))
        .add_system(read_input)
        .add_system(test_system.run_if(resource_exists_and_equals(GameState(1.0))))
        .run();
}

fn test_system(){
    info!("test_system ran.");
}

fn read_input(input:Res<Input<KeyCode>>, mut game_state:ResMut<GameState>){
    if input.just_pressed(KeyCode::Space) {
        game_state.0 = 1.0;
    }
    else {
        game_state.0 = 0.0;
    }
}