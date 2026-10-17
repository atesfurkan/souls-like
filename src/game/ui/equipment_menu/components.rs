use bevy::prelude::*;

// Hangi yuvanın seçili olduğunu tutan State (Kaynak)
#[derive(Resource, Default)]
pub struct ActiveSelectionSlot(pub Option<EquipmentSlotType>);

// Sağdaki eşya listesi panelini bulmak için marker
#[derive(Component)]
pub struct InventoryListPanel;

// Listedeki bir eşyaya tıklandığını anlamak için marker'lar
#[derive(Component)]
pub struct InventoryWeaponButton(pub crate::game::weapon::components::WeaponType);

#[derive(Component)]
pub struct InventoryShieldButton(pub crate::game::shield::data::ShieldType);

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum MenuState {
    #[default]
    Closed,
    EquipmentOpen,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EquipmentSlotType {
    RightHand,
    LeftHand,
}

#[derive(Component)]
pub struct EquipmentMenuRoot;

#[derive(Component)]
pub struct EquipmentSlotText(pub EquipmentSlotType);

// YENİ: Tıklanabilir butonları ayırt etmek için
#[derive(Component)]
pub struct EquipmentSlotButton(pub EquipmentSlotType);