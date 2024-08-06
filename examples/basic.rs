use bevy::{
    prelude::*,
    time::{Time, Timer, TimerMode},
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use bevy_noti_box::{NotiBoxEvent, NotiBoxPlugin, NotiPosition};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, States)]
enum GameState {
    #[default]
    Menu,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .init_state::<GameState>()
        .add_plugins(NotiBoxPlugin::new(vec![GameState::Menu])) // Add the plugin
        .add_systems(OnEnter(GameState::Menu), setup)
        .add_systems(Update, spam_noti)
        .run();
}

#[derive(Component, Deref, DerefMut)]
struct SpamTimer {
    timer: Timer,
}

fn setup(mut commands: Commands, mut event: EventWriter<NotiBoxEvent>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpamTimer {
        timer: Timer::from_seconds(6., TimerMode::Repeating),
    });

    let msg = TextSection::new(
        "Bello! La la la!",
        TextStyle {
            font_size: 32.,
            ..default()
        },
    );
    event.send(NotiBoxEvent {
        msg: vec![msg],
        pos: NotiPosition::TopRight,
        ..default()
    });
}

fn spam_noti(time: Res<Time>, mut event: EventWriter<NotiBoxEvent>, mut query: Query<&mut SpamTimer>) {
    for mut spam in query.iter_mut() {
        spam.timer.tick(time.delta());
        if spam.timer.just_finished() {
            event.send(NotiBoxEvent::from_message("Bello".into()));
        }
    }
}
