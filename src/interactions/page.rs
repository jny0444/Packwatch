use bevy::{prelude::*, window::CursorOptions};

use crate::interactions::components::{InspectionPage, InspectionTitle, OpenInspection};

pub fn spawn_page(mut commands: Commands) {
    commands.spawn((
        InspectionPage,
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        Visibility::Hidden,
        children![(
            Node {
                width: px(360),
                padding: UiRect::all(px(24)),
                ..default()
            },
            BackgroundColor(Color::srgb(0.12, 0.12, 0.14)),
            children![(
                InspectionTitle,
                Text::new(""),
                TextFont {
                    font_size: FontSize::Px(28.0),
                    ..default()
                },
                TextColor(Color::WHITE),
            )],
        )],
    ));
}

pub fn close_page(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut open: ResMut<OpenInspection>,
    mut page: Query<&mut Visibility, With<InspectionPage>>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if open.0.is_none() {
        return;
    }
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    open.0 = None;

    if let Ok(mut visibility) = page.single_mut() {
        *visibility = Visibility::Hidden;
    }

    cursor_options.grab_mode = bevy::window::CursorGrabMode::Locked;
    cursor_options.visible = false;
}
