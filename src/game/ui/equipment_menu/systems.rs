use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};
use super::components::*;
use crate::game::player::components::{Player, PlayerInventory};
use crate::game::weapon::components::{Weapon, WeaponType};
use crate::game::shield::components::Shield;
use crate::game::shield::data::ShieldType;
use crate::game::weapon::events::WeaponSwapRequestEvent;
use crate::game::shield::events::ShieldSwapRequestEvent;

pub fn toggle_menu_state(
    keyboard: Res<ButtonInput<KeyCode>>,
    state: Res<State<MenuState>>,
    mut next_state: ResMut<NextState<MenuState>>,
    mut active_slot: ResMut<ActiveSelectionSlot>,
) {
    if keyboard.just_pressed(KeyCode::KeyI) {
        match state.get() {
            MenuState::Closed => next_state.set(MenuState::EquipmentOpen),
            MenuState::EquipmentOpen => {
                next_state.set(MenuState::Closed);
                active_slot.0 = None;
            }
        }
    }
}

pub fn unlock_cursor_on_menu_open(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = q_windows.get_single_mut() {
        window.cursor.visible = true;
        window.cursor.grab_mode = CursorGrabMode::None;
    }
}

pub fn lock_cursor_on_menu_close(mut q_windows: Query<&mut Window, With<PrimaryWindow>>) {
    if let Ok(mut window) = q_windows.get_single_mut() {
        window.cursor.visible = false;
        window.cursor.grab_mode = CursorGrabMode::Locked;
    }
}

pub fn build_equipment_menu(mut commands: Commands) {
    let panel_color = Color::srgba(0.1, 0.1, 0.1, 0.95);
    let slot_bg_color = Color::srgb(0.2, 0.2, 0.2);
    let text_style = TextStyle { font_size: 20.0, color: Color::WHITE, ..default() };
    let header_style = TextStyle { font_size: 16.0, color: Color::srgb(0.7, 0.7, 0.7), ..default() };

    commands.spawn((
        NodeBundle {
            style: Style { width: Val::Percent(100.0), height: Val::Percent(100.0), justify_content: JustifyContent::Center, align_items: AlignItems::Center, ..default() },
            background_color: Color::srgba(0.0, 0.0, 0.0, 0.7).into(),
            ..default()
        },
        EquipmentMenuRoot,
    )).with_children(|overlay| {
        overlay.spawn(NodeBundle {
            style: Style { width: Val::Px(800.0), height: Val::Px(500.0), flex_direction: FlexDirection::Row, ..default() },
            background_color: panel_color.into(),
            ..default()
        }).with_children(|main_container| {
            // SOL PANEL
            main_container.spawn(NodeBundle {
                style: Style { width: Val::Percent(50.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, padding: UiRect::all(Val::Px(20.0)), ..default() },
                ..default()
            }).with_children(|left_panel| {
                left_panel.spawn(TextBundle::from_section("EQUIPMENT", TextStyle { font_size: 28.0, color: Color::WHITE, ..default() }));
                
                let mut spawn_slot = |parent: &mut ChildBuilder, title: &str, slot_type: EquipmentSlotType| {
                    parent.spawn((
                        ButtonBundle {
                            style: Style { width: Val::Percent(100.0), height: Val::Px(80.0), flex_direction: FlexDirection::Column, justify_content: JustifyContent::Center, align_items: AlignItems::Center, margin: UiRect::bottom(Val::Px(10.0)), border: UiRect::all(Val::Px(2.0)), ..default() },
                            background_color: slot_bg_color.into(),
                            border_color: Color::srgb(0.3, 0.3, 0.3).into(),
                            ..default()
                        },
                        EquipmentSlotButton(slot_type),
                    )).with_children(|slot| {
                        slot.spawn(TextBundle::from_section(title, header_style.clone()));
                        slot.spawn((TextBundle::from_section("Empty", text_style.clone()), EquipmentSlotText(slot_type)));
                    });
                };
                spawn_slot(left_panel, "Right Hand", EquipmentSlotType::RightHand);
                spawn_slot(left_panel, "Left Hand", EquipmentSlotType::LeftHand);
            });
            // SAĞ PANEL
            main_container.spawn((
                NodeBundle { style: Style { width: Val::Percent(50.0), height: Val::Percent(100.0), flex_direction: FlexDirection::Column, padding: UiRect::all(Val::Px(20.0)), margin: UiRect::left(Val::Px(10.0)), ..default() }, ..default() },
                InventoryListPanel,
            ));
        });
    });
}

pub fn destroy_equipment_menu(mut commands: Commands, query: Query<Entity, With<EquipmentMenuRoot>>) {
    for entity in query.iter() { commands.entity(entity).despawn_recursive(); }
}

pub fn update_equipment_texts(
    player_query: Query<(Entity, Option<&Weapon>), With<Player>>,
    shield_query: Query<(Entity, &Shield)>,
    parent_query: Query<&Parent>,
    mut text_query: Query<(&mut Text, &EquipmentSlotText)>,
) {
    let Ok((player_entity, weapon_opt)) = player_query.get_single() else { return };
    let mut found_shield_name = "None".to_string();

    for (shield_entity, shield) in shield_query.iter() {
        let mut current_entity = shield_entity;
        let mut is_owned = false;
        if current_entity == player_entity { is_owned = true; }
        while let Ok(parent) = parent_query.get(current_entity) {
            if parent.get() == player_entity { is_owned = true; break; }
            current_entity = parent.get();
        }
        if is_owned { found_shield_name = shield.name.clone(); break; }
    }

    for (mut text, slot_type) in text_query.iter_mut() {
        match slot_type.0 {
            EquipmentSlotType::RightHand => text.sections[0].value = weapon_opt.map_or("None".into(), |w| w.name.clone()),
            EquipmentSlotType::LeftHand => text.sections[0].value = found_shield_name.clone(),
        }
    }
}

pub fn handle_menu_interactions(
    mut commands: Commands,
    mut interaction_query: Query<(&Interaction, Option<&EquipmentSlotButton>, Option<&InventoryWeaponButton>, Option<&InventoryShieldButton>, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut active_slot: ResMut<ActiveSelectionSlot>,
    player_query: Query<(Entity, &PlayerInventory), With<Player>>,
    mut w_swap: EventWriter<WeaponSwapRequestEvent>,
    mut s_swap: EventWriter<ShieldSwapRequestEvent>,
    list_panel_query: Query<Entity, With<InventoryListPanel>>,
) {
    let Ok((player_entity, inventory)) = player_query.get_single() else { return };
    let Ok(list_entity) = list_panel_query.get_single() else { return };

    for (interaction, slot_btn, w_btn, s_btn, mut color) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            *color = Color::srgb(0.4, 0.4, 0.4).into();
            if let Some(slot) = slot_btn {
                active_slot.0 = Some(slot.0);
                commands.entity(list_entity).despawn_descendants();
                commands.entity(list_entity).with_children(|panel| {
                    match slot.0 {
                        EquipmentSlotType::RightHand => {
                            for w in &inventory.weapons {
                                panel.spawn((ButtonBundle { style: Style { width: Val::Percent(100.0), height: Val::Px(50.0), margin: UiRect::bottom(Val::Px(5.0)), ..default() }, background_color: Color::srgb(0.2, 0.2, 0.2).into(), ..default() }, InventoryWeaponButton(*w)))
                                    .with_children(|b| { b.spawn(TextBundle::from_section(format!("{:?}", w), TextStyle { font_size: 18.0, color: Color::WHITE, ..default() })); });
                            }
                        }
                        EquipmentSlotType::LeftHand => {
                            for s in &inventory.shields {
                                panel.spawn((ButtonBundle { style: Style { width: Val::Percent(100.0), height: Val::Px(50.0), margin: UiRect::bottom(Val::Px(5.0)), ..default() }, background_color: Color::srgb(0.2, 0.2, 0.2).into(), ..default() }, InventoryShieldButton(*s)))
                                    .with_children(|b| { b.spawn(TextBundle::from_section(format!("{:?}", s), TextStyle { font_size: 18.0, color: Color::WHITE, ..default() })); });
                            }
                        }
                    }
                });
            }
            if let Some(w) = w_btn { w_swap.send(WeaponSwapRequestEvent { entity: player_entity, target_weapon: w.0 }); }
            if let Some(s) = s_btn { s_swap.send(ShieldSwapRequestEvent { entity: player_entity, target_shield: s.0 }); }
        }
    }
}