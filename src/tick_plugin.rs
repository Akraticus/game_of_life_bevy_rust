use bevy::prelude::*;


pub struct TickPlugin;

impl Plugin for TickPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(TickTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .insert_resource(WillTick(false))
            .add_system(iterate_tick_timer)
            .add_system(update_will_tick.after(iterate_tick_timer));
    }
}

#[derive(Resource, PartialEq)]
pub struct WillTick(pub bool);

#[derive(Resource)]
pub struct TickTimer(pub Timer);

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