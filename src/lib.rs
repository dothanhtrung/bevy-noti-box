// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//!
//!

use bevy::{
    app::{App, Plugin, Update},
    color::{palettes::css::BLACK, Color},
    prelude::{
        default, BuildChildren, Changed, Commands, Component, DespawnRecursiveExt, Entity, Event, EventReader,
        NodeBundle, Query, Res, TextBundle, With,
    },
    text::{Text, TextSection},
    time::{Time, Timer, TimerMode},
    ui::{AlignSelf, BackgroundColor, Interaction, JustifySelf, Style},
};

#[cfg(feature = "state")]
use bevy::prelude::{in_state, States, IntoSystemConfigs};

macro_rules! plugin_systems {
    ( ) => {
        (listen_event, listen_click, countdown)
    };
}

#[cfg(feature = "state")]
#[derive(Default)]
pub struct NotiBoxPlugin<T>
where
    T: States,
{
    /// List of game state that this plugin will run in
    pub states: Option<Vec<T>>,
}

#[cfg(feature = "state")]
impl<T> Plugin for NotiBoxPlugin<T>
where
    T: States,
{
    fn build(&self, app: &mut App) {
        if let Some(states) = &self.states {
            for state in states {
                app.add_systems(Update, plugin_systems!().run_if(in_state(state.clone())));
            }
        } else {
            app.add_systems(Update, plugin_systems!());
        }
    }
}

#[cfg(feature = "state")]
impl<T> NotiBoxPlugin<T>
where
    T: States,
{
    pub fn new(states: Vec<T>) -> Self {
        Self { states: Some(states) }
    }
}

/// Use this if you don't care to state and want this plugin's systems run all the time.
#[derive(Default)]
pub struct NotiBoxPluginNoState;

impl Plugin for NotiBoxPluginNoState {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, plugin_systems!());
    }
}

const BACKGROUND_COLOR: Color = Color::srgb(
    0x1d as f32 / u8::MAX as f32,
    0x20 as f32 / u8::MAX as f32,
    0x21 as f32 / u8::MAX as f32,
);

#[derive(Default)]
pub enum NotiPosition {
    #[default]
    TopLeft,
    TopMid,
    TopRight,
    MidLeft,
    Center,
    MidRight,
    BotLeft,
    BotMid,
    BotRight,
}

#[derive(Default)]
enum AnimationState {
    #[default]
    FadeIn,
    FadeOut,
}

#[derive(Event, Default)]
pub struct NotiEvent {
    pub msg: Vec<TextSection>,
    pub pos: NotiPosition,
    pub show_time: f32,
    pub background_color: BackgroundColor,
}

#[derive(Component, Default)]
struct NotiBox {
    state: AnimationState,
    timer: Timer,
}

fn listen_event(mut commands: Commands, mut event: EventReader<NotiEvent>) {
    for noti in event.read() {
        commands
            .spawn((
                NotiBox {
                    timer: Timer::from_seconds(noti.show_time, TimerMode::Once),
                    ..default()
                },
                Interaction::None,
                NodeBundle {
                    style: pos_to_style(&noti.pos),
                    background_color: noti.background_color,
                    ..default()
                },
            ))
            .with_children(|builder| {
                builder.spawn(TextBundle::from_sections(noti.msg.clone()));
            });
    }
}

fn listen_click(mut commands: Commands, query: Query<(&Interaction, Entity), (Changed<Interaction>, With<NotiBox>)>) {
    for (i, e) in query.iter() {
        if *i == Interaction::Pressed {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn countdown(mut commands: Commands, mut query: Query<(Entity, &mut NotiBox)>, time: Res<Time>) {
    for (e, mut noti_box) in query.iter_mut() {
        noti_box.timer.tick(time.delta());
        if noti_box.timer.just_finished() {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn pos_to_style(pos: &NotiPosition) -> Style {
    match pos {
        NotiPosition::TopLeft => Style {
            align_self: AlignSelf::Start,
            justify_self: JustifySelf::Start,
            ..default()
        },
        NotiPosition::TopMid => Style { ..default() },
        NotiPosition::TopRight => Style { ..default() },
        NotiPosition::MidLeft => Style { ..default() },
        NotiPosition::Center => Style { ..default() },
        NotiPosition::MidRight => Style { ..default() },
        NotiPosition::BotLeft => Style { ..default() },
        NotiPosition::BotMid => Style { ..default() },
        NotiPosition::BotRight => Style { ..default() },
    }
}
