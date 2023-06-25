
use bevy::{prelude::*};

mod tick_plugin;
use tick_plugin::*;
mod game_of_life_plugin;
mod utility;

#[derive(States, Default, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppState{
    InMenu,
    #[default]
    InGame,
    Paused
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(TickPlugin)
        .add_plugin(game_of_life_plugin::GameOfLifePlugin)
        .add_system(test_system.in_set(TurnSystemSet))
        .add_startup_system(setup)
        .add_system(movement)

        .run();
}

fn test_system(time: Res<Time>){
    info!("Time: {}", time.elapsed_seconds());
}

fn setup(mut commands:Commands){
    commands.spawn(Camera2dBundle::default());
}

fn movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera>>,
) {
    for (mut transform, mut ortho) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
        }

        if keyboard_input.pressed(KeyCode::Z) {
            ortho.scale += 0.1;
        }

        if keyboard_input.pressed(KeyCode::X) {
            ortho.scale -= 0.1;
        }

        if ortho.scale < 0.5 {
            ortho.scale = 0.5;
        }

        let z = transform.translation.z;
        transform.translation += time.delta_seconds() * direction * 500.;
        // Important! We need to restore the Z values when moving the camera around.
        // Bevy has a specific camera setup and this can mess with how our layers are shown.
        transform.translation.z = z;
    }
}