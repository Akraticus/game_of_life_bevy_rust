use bevy::{prelude::*};

mod tick_plugin;
use bevy_ecs_tilemap::tiles::TileTextureIndex;
use tick_plugin::*;
mod game_of_life_plugin;
use rand::prelude::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TickPlugin)
        .add_plugin(game_of_life_plugin::GameOfLifePlugin)
        .add_startup_system(setup)
        .add_system(test_system.run_if(resource_exists_and_equals(WillTick(true))))
        .add_system(random_change_tile_texture_index.run_if(resource_exists_and_equals(WillTick(true))))
        .run();
}

fn test_system(time:Res<Time>){
    info!("Test System - Timestamp: {:?}", time.elapsed());
}

fn random_change_tile_texture_index(mut query: Query<&mut TileTextureIndex>){
    let mut rng = rand::thread_rng();

    for mut tile_texture_index in query.iter_mut(){
        tile_texture_index.0 = rng.gen_range(0..6);
    }
}

fn setup(mut commands:Commands){
    commands.spawn(Camera2dBundle::default());
}