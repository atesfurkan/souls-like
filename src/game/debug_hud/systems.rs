use bevy::prelude::*;
use super::components::DebugHudText;

// İlgili modüllerin Component'leri (Projenin yapısına göre yollar değişebilir)
use crate::game::health::components::Health; 
use crate::game::stamina::components::Stamina;
use crate::game::weapon::components::Weapon;
use crate::game::combat::components::EquippedWeapon; // Kontratta belirtilen marker
use crate::game::player::components::Player;

pub fn spawn_debug_hud(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Developer HUD Initializing...",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0), // Ekranın sağ üst köşesine sabitle
            ..default()
        }),
        DebugHudText,
    ));
}

pub fn update_debug_hud(
    mut hud_query: Query<&mut Text, With<DebugHudText>>,
    // Kontrata uygun olarak sadece Player root entity'sine bakıyoruz
    player_query: Query<(&Health, &Stamina, Option<&Weapon>), With<Player>>,
) {
    let Ok(mut text) = hud_query.get_single_mut() else { return };

    match player_query.get_single() {
        Ok((health, stamina, weapon_opt)) => {
            
            // Yeni kontrata göre Weapon struct'ının içinde 'name: String' alanı kesin olarak var.
            let weapon_name = match weapon_opt {
                Some(weapon) => weapon.name.clone(), 
                None => "None".to_string(),
            };

            text.sections[0].value = format!(
                "--- DEBUG HUD ---\nHealth: {:.1} / {:.1}\nStamina: {:.1} / {:.1}\nWeapon: {}",
                health.current, health.max,
                stamina.current, stamina.max,
                weapon_name
            );
        }
        Err(_) => {
            text.sections[0].value = "--- DEBUG HUD ---\nPLAYER DEAD (Entity Despawned)".to_string();
        }
    }
}