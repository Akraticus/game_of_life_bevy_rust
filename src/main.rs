use bevy::{prelude::*};

mod tick_plugin;
use tick_plugin::*;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(tick_plugin::TickPlugin)
        .add_system(test_system.run_if(resource_exists_and_equals(WillTick(true))))
        .run();
}

fn test_system(time:Res<Time>){
    info!("Test System - Tick delta: {:?}", time.elapsed());
}