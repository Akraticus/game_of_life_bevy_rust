use bevy::{prelude::*};

#[derive(Resource, PartialEq)]
struct WillTick(bool);

#[derive(Resource)]
struct TickTimer(Timer);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(TickTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
        .insert_resource(WillTick(false))
        .add_system(iterate_tick_timer)
        .add_system(update_will_tick.after(iterate_tick_timer))
        .add_system(test_system.run_if(resource_exists_and_equals(WillTick(true))).after(update_will_tick))
        .run();
}

fn test_system(time:Res<Time>){
    info!("Test System - Tick delta: {:?}", time.elapsed());
}

fn update_will_tick(tick_timer:Res<TickTimer>, mut will_tick: ResMut<WillTick>){
    if tick_timer.0.just_finished(){
        will_tick.0 = true;
    }
    else{
        will_tick.0 = false;
    }
}

fn iterate_tick_timer(time:Res<Time>, mut tick_timer:ResMut<TickTimer>){
    tick_timer.0.tick(time.delta());
}