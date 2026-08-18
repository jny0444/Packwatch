use bevy::prelude::*;

use crate::interactions::components::OpenInspection;
use crate::screens::GameState;

#[derive(Component)]
pub struct Crosshair;

pub fn spawn_crosshair(mut commands: Commands) {
    commands.spawn((
        Crosshair,
        DespawnOnExit(GameState::Playing),
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            (
                Node {
                    position_type: PositionType::Absolute,
                    width: px(14),
                    height: px(2),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            ),
            (
                Node {
                    position_type: PositionType::Absolute,
                    width: px(2),
                    height: px(14),
                    ..default()
                },
                BackgroundColor(Color::WHITE),
            )
        ],
    ));
}

pub fn update_crosshair(
    open: Res<OpenInspection>,
    mut crosshair: Query<&mut Visibility, With<Crosshair>>,
) {
    let Ok(mut visibility) = crosshair.single_mut() else {
        return;
    };

    *visibility = if open.is_open() {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };
}
