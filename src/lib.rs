// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//!
//!

use bevy::{
    app::{App, Plugin, Update},
    color::{Alpha, Color},
    prelude::{
        Changed, Commands, Component, default, DespawnRecursiveExt, Entity, Event, EventReader, Query, Res, TextBundle,
        With,
    },
    text::{TextSection, TextStyle},
    time::{Time, Timer, TimerMode},
    ui::{AlignSelf, BackgroundColor, BorderColor, Interaction, JustifySelf, Style, UiRect, Val},
};
#[cfg(feature = "state")]
use bevy::prelude::{in_state, IntoSystemConfigs, States};
use bevy::prelude::{AlignItems, BuildChildren, JustifyItems, NodeBundle, Text};

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
        app.add_event::<NotiBoxEvent>();

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
        app.add_event::<NotiBoxEvent>().add_systems(Update, plugin_systems!());
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
    TopRight,
    TopLeft,
    TopMid,
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

#[derive(Event)]
pub struct NotiBoxEvent {
    pub msg: Vec<TextSection>,
    pub pos: NotiPosition,
    pub show_time: f32,
    pub background_color: BackgroundColor,
    pub width: Val,
    pub height: Val,
}

impl Default for NotiBoxEvent {
    fn default() -> Self {
        Self {
            msg: Default::default(),
            pos: Default::default(),
            show_time: 5.,
            background_color: BACKGROUND_COLOR.into(),
            width: Val::Percent(20.),
            height: Val::Percent(20.),
        }
    }
}

impl NotiBoxEvent {
    pub fn from_message(msg: String) -> Self {
        NotiBoxEvent {
            msg: vec![TextSection::new(
                msg,
                TextStyle {
                    font_size: 20.,
                    ..default()
                },
            )],
            ..default()
        }
    }
}

#[derive(Component, Default)]
struct NotiBox {
    state: AnimationState,
    timer: Option<Timer>,
}

fn listen_event(mut commands: Commands, mut event: EventReader<NotiBoxEvent>) {
    for noti in event.read() {
        let mut timer = None;
        if noti.show_time > 0. {
            timer = Some(Timer::from_seconds(noti.show_time, TimerMode::Once))
        }

        let mut border_color: BorderColor = noti.background_color.0.into();
        border_color.0.set_alpha(0.4);

        commands
            .spawn((
                NotiBox { timer, ..default() },
                Interaction::None,
                NodeBundle {
                    style: pos_to_style(&noti.pos),
                    background_color: noti.background_color,
                    border_color,
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
        if let Some(timer) = noti_box.timer.as_mut() {
            timer.tick(time.delta());
            if timer.just_finished() {
                commands.entity(e).despawn_recursive();
            }
        }
    }
}

fn pos_to_style(pos: &NotiPosition) -> Style {
    let mut ret = Style {
        width: Val::Percent(20.),
        height: Val::Percent(20.),
        margin: UiRect::all(Val::Px(5.)),
        justify_items: JustifyItems::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    match pos {
        NotiPosition::TopLeft => {
            ret.justify_self = JustifySelf::Start;
            ret.align_self = AlignSelf::FlexStart;
        }
        NotiPosition::TopMid => {
            ret.justify_self = JustifySelf::Center;
            ret.align_self = AlignSelf::FlexStart;
        }
        NotiPosition::TopRight => {
            ret.justify_self = JustifySelf::End;
            ret.align_self = AlignSelf::FlexStart;
        }
        NotiPosition::MidLeft => {
            ret.justify_self = JustifySelf::Start;
            ret.align_self = AlignSelf::Center;
        }
        NotiPosition::Center => {
            ret.justify_self = JustifySelf::Center;
            ret.align_self = AlignSelf::Center;
        }
        NotiPosition::MidRight => {
            ret.justify_self = JustifySelf::End;
            ret.align_self = AlignSelf::Center;
        }
        NotiPosition::BotLeft => {
            ret.justify_self = JustifySelf::Start;
            ret.align_self = AlignSelf::FlexEnd;
        }
        NotiPosition::BotMid => {
            ret.justify_self = JustifySelf::Center;
            ret.align_self = AlignSelf::FlexEnd;
        }
        NotiPosition::BotRight => {
            ret.justify_self = JustifySelf::End;
            ret.align_self = AlignSelf::FlexEnd;
        }
    }

    ret
}
