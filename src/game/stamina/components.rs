use bevy::prelude::*;

#[derive(Component)]
pub struct Stamina {
    pub current: f32,
    pub max: f32,
    pub regen_rate: f32,
    pub sprint_cost: f32,
    pub jump_cost: f32,
    pub regen_timer: Timer,
}

impl Default for Stamina {
    fn default() -> Self {
        Self {
            current: 100.0,
            max: 100.0,
            regen_rate: 20.0,
            sprint_cost: 25.0, // Saniyede harcanan stamina
            jump_cost: 10.0, // Saniyede harcanan stamina
            // 1.5 saniyelik bekleme süresi, sürekli baştan kurulacak
            regen_timer: Timer::from_seconds(1.5, TimerMode::Once),
        }
    }
}

impl Stamina {
    // Hem anlık (attack) hem sürekli (sprint) tüketim için ortak metot
    pub fn try_drain(&mut self, amount: f32) -> bool {
        if self.current > 0.0 {
            self.current -= amount;
            if self.current < 0.0 {
                self.current = 0.0;
            }
            self.regen_timer.reset(); // Harcama yapıldığı an yenilenme sayacını başa sar
            true
        } else {
            false
        }
    }
}

#[derive(Component)]
pub struct StaminaBarUI;
