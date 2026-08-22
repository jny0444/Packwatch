use bevy::prelude::*;

use crate::{items::wallet::Wallet, screens::GameState};

#[derive(Component)]
pub struct WalletHud;

pub fn spawn_wallet_hud(mut commands: Commands) {
    commands.spawn((
        WalletHud,
        DespawnOnExit(GameState::Playing),
        Node {
            position_type: PositionType::Absolute,
            top: px(24),
            left: px(24),
            ..default()
        },
        Text::new("$0"),
        TextFont {
            font_size: FontSize::Px(22.0),
            ..default()
        },
        TextColor(Color::WHITE),
    ));
}

pub fn update_wallet_hud(wallet: Res<Wallet>, mut label: Query<&mut Text, With<WalletHud>>) {
    let Ok(mut text) = label.single_mut() else {
        return;
    };
    **text = format!("${}", wallet.balance);
}
