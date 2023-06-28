use std::time::Duration;

use bevy::prelude::*;
use crate::AppState;

pub struct TickPlugin;

#[derive(Default)]
pub struct IterationSpeedIncreaseEvent;

#[derive(Default)]
pub struct IterationSpeedDecreaseEvent;

#[derive(Default)]
pub struct IterationPauseEvent{
    pub will_pause:bool
}

#[derive(Default)]
pub struct IterationPauseToggleEvent;

#[derive(Resource)]
pub struct TickTimer(pub Timer);

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct TurnSystemSet;

impl Plugin for TickPlugin{
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .insert_resource(TickTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_event::<IterationSpeedIncreaseEvent>()
            .add_event::<IterationSpeedDecreaseEvent>()
            .add_event::<IterationPauseEvent>()
            .add_event::<IterationPauseToggleEvent>()
            .add_system(iteration_pause_toggle_event_handler)
            .add_system(iteration_pause_event_handler)
            .add_system(iteration_speed_increase_event_handler)
            .add_system(iteration_speed_decrease_event_handler)
            .configure_set(TurnSystemSet
                .run_if(in_state(AppState::InGame)
                    .and_then(tick_timer_finished))
            )
            .add_system(iterate_tick_timer.run_if(in_state(AppState::InGame)));
    }
}

fn iterate_tick_timer(time:Res<Time>, mut tick_timer:ResMut<TickTimer>){
    tick_timer.0.tick(time.delta());
}

fn tick_timer_finished(tick_timer:Res<TickTimer>) -> bool{
    tick_timer.0.just_finished()
}

fn iteration_speed_increase_event_handler(mut speed_increase_event:EventReader<IterationSpeedIncreaseEvent>, mut tick_timer:ResMut<TickTimer>){
    if let Some(_) = speed_increase_event.iter().last(){
        let duration = tick_timer.0.duration().as_secs_f32() * 0.75;
        info!("Increase Event: {:?}", duration);
        tick_timer.0.set_duration(Duration::from_secs_f32(duration));
    }
}

fn iteration_speed_decrease_event_handler(mut speed_increase_event:EventReader<IterationSpeedDecreaseEvent>, mut tick_timer:ResMut<TickTimer>){
    if let Some(_) = speed_increase_event.iter().last(){
        let duration = tick_timer.0.duration().as_secs_f32() * 1.25;
        info!("Decrease Event: {:?}", duration);
        tick_timer.0.set_duration(Duration::from_secs_f32(duration));
    }
}

fn iteration_pause_event_handler(mut pause_event:EventReader<IterationPauseEvent>, mut tick_timer:ResMut<TickTimer>){
    // we handle all pauses simultaneously, so we skip to the last one since it would overwrite all others anyway
    match pause_event.iter().last(){
        Some(pause) => match pause.will_pause{
            true => tick_timer.0.pause(),
            false => tick_timer.0.unpause()
        },
        None => ()
    }
}

fn iteration_pause_toggle_event_handler(mut pause_event:EventReader<IterationPauseToggleEvent>, mut tick_timer:ResMut<TickTimer>){
    // we handle all pauses simultaneously, so we skip to the last one since it would overwrite all others anyway
    if let Some(_) = pause_event.iter().last(){
        match tick_timer.0.paused(){
            true => tick_timer.0.unpause(),
            false => tick_timer.0.pause()
        }
    }
}