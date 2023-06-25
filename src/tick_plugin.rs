use bevy::prelude::*;

use crate::AppState;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct TurnSystemSet;

pub struct TickPlugin;

impl Plugin for TickPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(TickTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .configure_set(TurnSystemSet
                .run_if(in_state(AppState::InGame)
                    .and_then(tick_timer_finished))
            )
            .add_system(iterate_tick_timer.run_if(in_state(AppState::InGame)));
    }
}

#[derive(Resource)]
pub struct TickTimer(pub Timer);

fn iterate_tick_timer(time:Res<Time>, mut tick_timer:ResMut<TickTimer>){
    tick_timer.0.tick(time.delta());
}

fn tick_timer_finished(tick_timer:Res<TickTimer>) -> bool{
    tick_timer.0.just_finished()
}