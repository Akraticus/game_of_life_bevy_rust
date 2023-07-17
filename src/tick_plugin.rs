use std::time::Duration;

use bevy::prelude::*;
use crate::AppState;

pub struct TickPlugin;

#[derive(Default)]
pub enum IterationSpeed{
    Paused,
    VerySlow,
    Slow,
    #[default]
    Regular,
    Fast,
    VeryFast
}

#[derive(Resource)]
pub struct SpeedConfiguration{
    pub current_speed:IterationSpeed,
    very_slow_duration:f32,
    slow_duration:f32,
    regular_duration:f32,
    fast_duration:f32,
    very_fast_duration:f32
}

impl Default for SpeedConfiguration{
    fn default() -> Self {
        Self { current_speed: IterationSpeed::default(), very_slow_duration: 8.0, slow_duration: 4.0, regular_duration: 2.0, fast_duration: 0.5, very_fast_duration: 0.1 }
    }
}

impl SpeedConfiguration{
    fn increase_speed(&mut self){
        match self.current_speed{
            IterationSpeed::Paused => self.current_speed = IterationSpeed::VerySlow,
            IterationSpeed::VerySlow => self.current_speed = IterationSpeed::Slow,
            IterationSpeed::Slow => self.current_speed = IterationSpeed::Regular,
            IterationSpeed::Regular => self.current_speed = IterationSpeed::Fast,
            IterationSpeed::Fast => self.current_speed = IterationSpeed::VeryFast,
            IterationSpeed::VeryFast => ()  // can't increase further
        }   
    }

    fn decrease_speed(&mut self){
        match self.current_speed{
            IterationSpeed::Paused => (),   // can't decrease further
            IterationSpeed::VerySlow => self.current_speed = IterationSpeed::Paused,
            IterationSpeed::Slow => self.current_speed = IterationSpeed::VerySlow,
            IterationSpeed::Regular => self.current_speed = IterationSpeed::Slow,
            IterationSpeed::Fast => self.current_speed = IterationSpeed::Regular,
            IterationSpeed::VeryFast => self.current_speed = IterationSpeed::Fast,
        }   
    }
}

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
            .insert_resource(SpeedConfiguration::default())
            .insert_resource(TickTimer(Timer::from_seconds(0.5, TimerMode::Repeating)))
            .add_event::<IterationSpeedIncreaseEvent>()
            .add_event::<IterationSpeedDecreaseEvent>()
            .add_event::<IterationPauseEvent>()
            .add_event::<IterationPauseToggleEvent>()
            .add_startup_system(set_tick_timer_duration_to_iteration_speed)
            .add_system(iteration_pause_toggle_event_handler)
            .add_system(iteration_pause_event_handler)
            .add_system(iteration_speed_increase_event_handler)
            .add_system(iteration_speed_decrease_event_handler)
            .configure_set(TurnSystemSet
                .run_if(tick_timer_finished)
            )
            .add_system(iterate_tick_timer)
            .add_system(set_tick_timer_duration_to_iteration_speed.run_if(resource_changed::<SpeedConfiguration>()));
    }
}

fn set_tick_timer_duration_to_iteration_speed(speed_configuration:Res<SpeedConfiguration>, mut tick_timer:ResMut<TickTimer>){
    fn unpause_and_set_duration(mut timer:ResMut<TickTimer>, duration:f32){
        timer.0.unpause();
        timer.0.set_duration(Duration::from_secs_f32(duration));
    }
    
    match speed_configuration.current_speed{
        IterationSpeed::Paused => tick_timer.0.pause(),
        IterationSpeed::VerySlow => unpause_and_set_duration(tick_timer, speed_configuration.very_slow_duration),
        IterationSpeed::Slow => unpause_and_set_duration(tick_timer, speed_configuration.slow_duration),
        IterationSpeed::Regular => unpause_and_set_duration(tick_timer, speed_configuration.regular_duration),
        IterationSpeed::Fast => unpause_and_set_duration(tick_timer, speed_configuration.fast_duration),
        IterationSpeed::VeryFast => unpause_and_set_duration(tick_timer, speed_configuration.very_fast_duration),
    }
}

fn iterate_tick_timer(time:Res<Time>, mut tick_timer:ResMut<TickTimer>){
    tick_timer.0.tick(time.delta());
}

fn tick_timer_finished(tick_timer:Res<TickTimer>) -> bool{
    tick_timer.0.just_finished()
}

fn iteration_speed_increase_event_handler(mut speed_increase_event:EventReader<IterationSpeedIncreaseEvent>, mut speed_configuration:ResMut<SpeedConfiguration>){
    if let Some(_) = speed_increase_event.iter().last(){
        speed_configuration.increase_speed();
    }
}

fn iteration_speed_decrease_event_handler(mut speed_increase_event:EventReader<IterationSpeedDecreaseEvent>, mut speed_configuration:ResMut<SpeedConfiguration>){
    if let Some(_) = speed_increase_event.iter().last(){
        speed_configuration.decrease_speed();
    }
}

fn iteration_pause_event_handler(mut pause_event:EventReader<IterationPauseEvent>, mut speed_configuration:ResMut<SpeedConfiguration>){
    // we handle all pauses simultaneously, so we skip to the last one since it would overwrite all others anyway
    match pause_event.iter().last(){

        Some(pause) => match pause.will_pause{
            true => speed_configuration.current_speed = IterationSpeed::Paused,
            false => speed_configuration.current_speed = IterationSpeed::Regular,
        },
        None => ()
    }
}

fn iteration_pause_toggle_event_handler(mut pause_event:EventReader<IterationPauseToggleEvent>, mut speed_configuration:ResMut<SpeedConfiguration>){
    // we handle all pauses simultaneously, so we skip to the last one since it would overwrite all others anyway
    if let Some(_) = pause_event.iter().last(){
        match speed_configuration.current_speed{
            IterationSpeed::Paused => speed_configuration.current_speed = IterationSpeed::Regular,
            _ => speed_configuration.current_speed = IterationSpeed::Paused
        }
    }
}