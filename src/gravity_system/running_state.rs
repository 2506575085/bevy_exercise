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

#[derive(Resource)]
struct LongPressTimer(Timer);

pub struct RunningStatePlugin;
impl Plugin for RunningStatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LongPressTimer(Timer::from_seconds(2.5, TimerMode::Repeating)));
        app.init_state::<RunningState>().enable_state_scoped_entities::<RunningState>();
        app.add_event::<ResetEvent>();
        app.add_systems(Update, handle_reset_operation);
        app.add_systems(PostUpdate, handle_running_state_change);
        app.add_systems(OnEnter(RunningState::Paused), spawn_status_texts);
        app.add_systems(OnEnter(RunningState::Resetting), spawn_status_texts);
        app.add_systems(OnEnter(RunningState::End), spawn_status_texts);
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

fn handle_reset_operation(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut reset_event_writer: EventWriter<ResetEvent>,
    mut timer: ResMut<LongPressTimer>,
    mut next_state: ResMut<NextState<RunningState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyR) {
        timer.0.reset();
        next_state.set(RunningState::Resetting);
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

fn spawn_status_texts(mut commands: Commands, state: Res<State<RunningState>>) {
    let (paused_text_bundle, paused_node_bundle) = get_status_texts_bundles(state.get());
    let text_entity = commands.spawn(paused_text_bundle).id();
    commands.spawn(paused_node_bundle).add_child(text_entity);
}

fn get_status_texts_bundles(running_state: &RunningState) -> (TextBundle, (NodeBundle, StateScoped<RunningState>)) {
    let text = match running_state {
        RunningState::Paused => "Paused",
        RunningState::Resetting => "Resetting",
        RunningState::End => "End",
        _ => ""
    };
    (
        TextBundle::from_section(
        text,
        TextStyle {
                font_size: 180.,
                color: Color::WHITE,
                ..default()
            }
        ),
        (
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
            StateScoped(*running_state),
        )
    )
}