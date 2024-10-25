use bevy::input::common_conditions::input_just_pressed;
use bevy::prelude::*;

#[derive(States, Clone, Copy, Default, Eq, PartialEq, Hash, Debug)]
pub enum RunningState {
    #[default]
    Running,
    Paused,
    Resetting,
    End
}

#[derive(Event)]
pub struct ResetEvent;

pub struct RunningStatePlugin;
impl Plugin for RunningStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<RunningState>().enable_state_scoped_entities::<RunningState>();
        app.add_event::<ResetEvent>();
        app.add_systems(Update, handle_reset_operation);
        app.add_systems(Update, switch_running_and_paused.run_if(input_just_pressed(KeyCode::Escape)));
        app.add_systems(OnEnter(RunningState::Paused), spawn_status_text("Paused", RunningState::Paused));
        app.add_systems(OnEnter(RunningState::Resetting), spawn_status_text("Resetting...", RunningState::Resetting));
        app.add_systems(OnEnter(RunningState::End), spawn_status_text("End", RunningState::End));
    }
}

fn switch_running_and_paused(state: Res<State<RunningState>>, mut next_state: ResMut<NextState<RunningState>>) {
    match state.get() {
        RunningState::Running => next_state.set(RunningState::Paused),
        RunningState::Paused => next_state.set(RunningState::Running),
        _ => ()
    };
}

#[derive(Default)]
struct LongPressTimer(f32);
fn handle_reset_operation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut long_press_timer: Local<LongPressTimer>,
    mut reset_event_writer: EventWriter<ResetEvent>,
    state: Res<State<RunningState>>,
    mut next_state: ResMut<NextState<RunningState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        long_press_timer.0 = 0.0;
        next_state.set(RunningState::Resetting);
    }
    if keyboard_input.just_released(KeyCode::KeyR) {
        long_press_timer.0 = 0.0;
        next_state.set(RunningState::Running);
    }
    if keyboard_input.pressed(KeyCode::KeyR) {
        if *state.get() != RunningState::Resetting { return; }
        long_press_timer.0 += time.delta_seconds();
        if long_press_timer.0 >= 2.0 {
            long_press_timer.0 = 0.0;
            reset_event_writer.send(ResetEvent);
            next_state.set(RunningState::Running);
        }
    }
}

fn spawn_status_text(text: &'static str, running_state: RunningState) -> impl FnMut(Commands) {
    move |mut commands| {
        let text_bundle= TextBundle::from_section(
            text,
            TextStyle {
                font_size: 180.,
                color: Color::WHITE,
                ..default()
            }
        );
        let node_bundle = (
            NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                    ..default()
                },
                ..default()
            },
            StateScoped(running_state),
        );
        let text_entity = commands.spawn(text_bundle).id();
        commands.spawn(node_bundle).add_child(text_entity);
    }
}