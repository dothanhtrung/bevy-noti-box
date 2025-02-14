// Copyright 2024 Trung Do <dothanhtrung@pm.me>

//!
//!
use bevy::{
    app::{App, Plugin, Update},
    color::{Alpha, Color},
    prelude::*,
};

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

const BACKGROUND_COLOR: Color = Color::BLACK;

const DEFAULT_ANIMATION_DURATION: f32 = 0.5;

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

#[derive(Default, PartialEq)]
enum AnimationState {
    #[default]
    Start,
    Middle,
    End,
}

#[derive(Event)]
pub struct NotiBoxEvent {
    pub msg: String,
    pub font: TextFont,
    pub text_color: Color,
    pub pos: NotiPosition,
    pub show_time: f32,
    pub background_color: BackgroundColor,
    pub width: Val,
    pub height: Val,
}

impl Default for NotiBoxEvent {
    fn default() -> Self {
        Self {
            msg: String::new(),
            font: TextFont::default(),
            text_color: Color::WHITE,
            pos: NotiPosition::default(),
            show_time: 5.,
            background_color: BACKGROUND_COLOR.into(),
            width: Val::Percent(20.),
            height: Val::Percent(20.),
        }
    }
}

impl NotiBoxEvent {
    pub fn from_message(msg: String) -> Self {
        NotiBoxEvent { msg, ..default() }
    }
}

#[derive(Component, Default)]
#[require(Interaction)]
struct NotiBox {
    states: Vec<(AnimationState, Timer)>,
}

fn listen_event(mut commands: Commands, mut event: EventReader<NotiBoxEvent>) {
    for noti in event.read() {
        let states = if noti.show_time > 0. {
            vec![
                (
                    AnimationState::Start,
                    Timer::from_seconds(DEFAULT_ANIMATION_DURATION, TimerMode::Once),
                ),
                (
                    AnimationState::Middle,
                    Timer::from_seconds(noti.show_time, TimerMode::Once),
                ),
                (
                    AnimationState::End,
                    Timer::from_seconds(DEFAULT_ANIMATION_DURATION, TimerMode::Once),
                ),
            ]
        } else {
            Vec::new()
        };

        let mut border_color: BorderColor = noti.background_color.0.into();
        border_color.0.set_alpha(0.4);
        let mut background_color = noti.background_color.0;
        background_color.set_alpha(0.0);
        let mut text_color = noti.text_color;
        text_color.set_alpha(0.0);

        commands.spawn((
            NotiBox { states },
            pos_to_style(&noti.pos),
            BackgroundColor::from(background_color),
            border_color,
            Text::from(noti.msg.clone()),
            noti.font.clone(),
            TextColor::from(text_color),
        ));
    }
}

fn listen_click(mut commands: Commands, query: Query<(&Interaction, Entity), (Changed<Interaction>, With<NotiBox>)>) {
    for (i, e) in query.iter() {
        if *i == Interaction::Pressed {
            commands.entity(e).despawn_recursive();
        }
    }
}

fn countdown(
    mut commands: Commands,
    mut query: Query<(Entity, &mut NotiBox, &mut BackgroundColor, &mut TextColor)>,
    time: Res<Time>,
) {
    for (e, mut noti_box, mut bg_color, mut text_color) in query.iter_mut() {
        for (state, ref mut timer) in noti_box.states.iter_mut() {
            if timer.finished() {
                continue;
            }
            timer.tick(time.delta());
            match state {
                AnimationState::Start => {
                    let alpha = timer.elapsed_secs() / timer.duration().as_secs_f32();
                    bg_color.0.set_alpha(alpha);
                    text_color.0.set_alpha(alpha);
                }
                AnimationState::Middle => {
                    bg_color.0.set_alpha(1.);
                    text_color.0.set_alpha(1.);
                }
                AnimationState::End => {
                    let alpha = timer.remaining_secs() / timer.duration().as_secs_f32();
                    bg_color.0.set_alpha(alpha);
                    text_color.0.set_alpha(alpha);

                    if timer.just_finished() {
                        commands.entity(e).despawn_recursive();
                    }
                }
            }
            break;
        }
    }
}

fn pos_to_style(pos: &NotiPosition) -> Node {
    let mut ret = Node {
        width: Val::Percent(20.),
        height: Val::Percent(20.),
        margin: UiRect::all(Val::Px(5.)),
        justify_content: JustifyContent::Center,
        align_content: AlignContent::Center,
        align_items: AlignItems::Center,
        justify_items: JustifyItems::Center,
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
