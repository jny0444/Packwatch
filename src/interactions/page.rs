use bevy::{prelude::*, window::CursorOptions};

use crate::camera::set_cursor_locked;
use crate::interactions::components::{InspectionPage, InspectionTitle, OpenInspection};
use crate::items::ShopPage;
use crate::items::bag::InventoryPage;
use crate::screens::GameState;

pub fn spawn_page(mut commands: Commands) {
    commands.spawn((
        InspectionPage,
        DespawnOnExit(GameState::Playing),
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
    mut page: Query<&mut Visibility, (With<InspectionPage>, Without<ShopPage>, Without<InventoryPage>)>,
    mut shop: Query<&mut Visibility, (With<ShopPage>, Without<InspectionPage>, Without<InventoryPage>)>,
    mut bag: Query<&mut Visibility, (With<InventoryPage>, Without<InspectionPage>, Without<ShopPage>)>,
    mut cursor_options: Single<&mut CursorOptions>,
) {
    if !open.is_open() {
        return;
    }
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    open.close();

    if let Ok(mut visibility) = page.single_mut() {
        *visibility = Visibility::Hidden;
    }
    if let Ok(mut visibility) = shop.single_mut() {
        *visibility = Visibility::Hidden;
    }
    if let Ok(mut visibility) = bag.single_mut() {
        *visibility = Visibility::Hidden;
    }

    set_cursor_locked(&mut cursor_options, true);
}
