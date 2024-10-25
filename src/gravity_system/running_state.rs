use bevy::prelude::*;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum RunningState {
    #[default]
    Running,
    Paused,
    End
}

#[derive(Event)]
pub struct ResetEvent;

#[derive(Resource)]
struct LongPressTimer(Timer);

pub struct RunningStatePlugin;
impl Plugin for RunningStatePlugin {
    fn build(&self, app: &mut App) {
        // TODOIMPORTANTE: init_state must be run after the `DefaultPlugins`
        app.insert_resource(LongPressTimer(Timer::from_seconds(2.5, TimerMode::Repeating)));
        app.init_state::<RunningState>();
        app.add_event::<ResetEvent>();
        app.add_systems(Update, handle_reset_opration);
        app.add_systems(PostUpdate, handle_running_state_change);
    }
}

fn handle_running_state_change(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    running_state: Res<State<RunningState>>,
    mut next_state: ResMut<NextState<RunningState>>
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match running_state.get() {
            RunningState::Running => next_state.set(RunningState::Paused),
            RunningState::Paused => next_state.set(RunningState::Running),
            _ => ()
        }
    }
}

fn handle_reset_opration(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut reset_event_writer: EventWriter<ResetEvent>,
    mut timer: ResMut<LongPressTimer>,
    mut next_state: ResMut<NextState<RunningState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        timer.0.reset();
        next_state.set(RunningState::Paused);
    }
    if keyboard_input.just_released(KeyCode::KeyR) {
        timer.0.reset();
        next_state.set(RunningState::Running);
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        let time_tick = timer.0.tick(time.delta());
        if time_tick.just_finished() {
            reset_event_writer.send(ResetEvent);
            next_state.set(RunningState::Running);
        }
    }
}