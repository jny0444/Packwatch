use bevy::prelude::*;

pub fn spawn_crosshair(mut commands: Commands) {
    commands.spawn((
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
