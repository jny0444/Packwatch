use bevy::prelude::*;

use crate::screens::GameState;

pub struct StartPlugin;

#[derive(Component)]
struct StartHint;

impl Plugin for StartPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Start), spawn_start)
            .add_systems(
                Update,
                (start_game, pulse_start_hint).run_if(in_state(GameState::Start)),
            );
    }
}

fn spawn_start(mut commands: Commands) {
    commands.spawn((
        DespawnOnExit(GameState::Start),
        GlobalZIndex(1000),
        Node {
            width: percent(100),
            height: percent(100),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            row_gap: px(16),
            ..default()
        },
        BackgroundColor(Color::srgb(0.04, 0.04, 0.05)),
        children![
            (
                Text::new("PACKWATCH"),
                TextFont {
                    font_size: FontSize::Px(56.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            ),
            (
                StartHint,
                Text::new("Click to start"),
                TextFont {
                    font_size: FontSize::Px(20.0),
                    ..default()
                },
                TextColor(Color::srgb(0.65, 0.65, 0.68)),
            ),
        ],
    ));
}

fn start_game(
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if mouse_button.just_pressed(MouseButton::Left) || keyboard.get_just_pressed().next().is_some()
    {
        next_state.set(GameState::Playing);
    }
}

fn pulse_start_hint(time: Res<Time>, mut hints: Query<&mut TextColor, With<StartHint>>) {
    let t = (time.elapsed_secs() * 2.4).sin() * 0.5 + 0.5;
    let lightness = 0.2 + 0.82 * t;
    for mut color in &mut hints {
        *color = TextColor(Color::srgb(lightness, lightness, lightness + 0.03));
    }
}
