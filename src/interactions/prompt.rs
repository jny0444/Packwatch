use bevy::prelude::*;

use crate::interactions::components::{FocusedInteractable, InteractPrompt, OpenInspection};
use crate::screens::GameState;

pub fn spawn_prompt(mut commands: Commands) {
    commands.spawn((
        InteractPrompt,
        DespawnOnExit(GameState::Playing),
        Node {
            position_type: PositionType::Absolute,
            width: percent(100),
            bottom: px(64),
            justify_content: JustifyContent::Center,
            ..default()
        },
        Visibility::Hidden,
        children![(
            Text::new("E to interact"),
            TextFont {
                font_size: FontSize::Px(20.0),
                ..default()
            },
            TextColor(Color::WHITE),
        )],
    ));
}

pub fn update_prompt(
    open: Res<OpenInspection>,
    focused: Res<FocusedInteractable>,
    mut prompt: Query<&mut Visibility, With<InteractPrompt>>,
) {
    let Ok(mut visibility) = prompt.single_mut() else {
        return;
    };

    *visibility = if !open.is_open() && focused.entity().is_some() {
        Visibility::Visible
    } else {
        Visibility::Hidden
    };
}
