use bevy::prelude::*;
use super::components::{Stamina, StaminaBarUI};
use crate::game::player::components::Player; // UI'ın sadece oyuncunun staminasını okuması için

pub fn stamina_regen(time: Res<Time>, mut query: Query<&mut Stamina>) {
    for mut stamina in query.iter_mut() {
        stamina.regen_timer.tick(time.delta());
        
        if stamina.regen_timer.finished() && stamina.current < stamina.max {
            stamina.current += stamina.regen_rate * time.delta_seconds();
            if stamina.current > stamina.max {
                stamina.current = stamina.max;
            }
        }
    }
}

pub fn spawn_stamina_ui(mut commands: Commands) {
    // Arka plan (Siyah çerçeve)
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        },
        background_color: Color::srgb(0.1, 0.1, 0.1).into(),
        ..default()
    }).with_children(|parent| {
        // Ön plan (Yeşil Stamina Barı)
        parent.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                background_color: Color::srgb(0.2, 0.8, 0.2).into(),
                ..default()
            },
            StaminaBarUI,
        ));
    });
}

pub fn update_stamina_ui(
    stamina_query: Query<&Stamina, With<Player>>,
    mut ui_query: Query<&mut Style, With<StaminaBarUI>>,
) {
    let Ok(stamina) = stamina_query.get_single() else { return };
    let Ok(mut style) = ui_query.get_single_mut() else { return };

    // Staminayı yüzdeye çevir ve UI barının genişliğini ayarla
    let percentage = (stamina.current / stamina.max) * 100.0;
    style.width = Val::Percent(percentage);
}
