use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum UpgradeType {
    Magnet,
    BallMultiplier,
    AddBall,
    SuperBall,
    ExtraLife,
    SpaceInvader,
}

pub struct Upgrades {
    pub falling_upgrades: Vec<UpgradeCoin>,
    pub active_upgrades: Vec<UpgradeType>,
    pub magnet_active: bool,
    pub space_invader_active: bool,
}

impl Upgrades {
    pub fn new() -> Self {
        Self {
            falling_upgrades: Vec::new(),
            active_upgrades: Vec::new(),
            magnet_active: false,
            space_invader_active: false,
        }
    }

    pub fn spawn_upgrade(&mut self, originating_block: Rect) {
        self.falling_upgrades
            .push(UpgradeCoin::new(None, originating_block));
    }

    pub fn update(&mut self, player_rect: Rect) {
        for upgrade in &mut self.falling_upgrades {
            if self.magnet_active {
                let center = player_rect.point() + player_rect.size() * 0.5f32;
                let to_center = center - upgrade.rect.point();
                let to_center = to_center.normalize();
                upgrade.rect.x += to_center.x * 2f32;
                upgrade.rect.y += to_center.y * 2f32;
            } else {
                upgrade.rect.y += 1f32;
            }
        }
        if let Some(upgrade) = self
            .falling_upgrades
            .iter()
            .find(|upgrade| upgrade.rect.intersect(player_rect).is_some())
        {
            self.active_upgrades.push(upgrade.upgrade_type);
        }
        self.falling_upgrades.retain(|upgrade| {
            upgrade.rect.y < screen_height() && upgrade.rect.intersect(player_rect).is_none()
        });
    }

    pub fn reset(&mut self) {
        self.falling_upgrades.clear();
        self.active_upgrades.clear();
        self.magnet_active = false;
        self.space_invader_active = false;
    }

    pub fn draw(&mut self) {
        for upgrade in &mut self.falling_upgrades {
            let color: Color = match upgrade.upgrade_type {
                UpgradeType::Magnet => PINK,
                UpgradeType::BallMultiplier => SKYBLUE,
                UpgradeType::AddBall => PURPLE,
                UpgradeType::SuperBall => VIOLET,
                UpgradeType::ExtraLife => GOLD,
                UpgradeType::SpaceInvader => BLACK,
            };

            draw_rectangle(
                upgrade.rect.x,
                upgrade.rect.y,
                upgrade.rect.w,
                upgrade.rect.h,
                color,
            );
        }
    }
}
pub struct UpgradeCoin {
    upgrade_type: UpgradeType,
    pub rect: Rect,
}

impl UpgradeCoin {
    pub fn new(upgrade_type: Option<UpgradeType>, originating_block: Rect) -> Self {
        let upgrade_type = upgrade_type.unwrap_or_else(|| match rand::gen_range(0, 6) {
            0 => UpgradeType::Magnet,
            1 => UpgradeType::BallMultiplier,
            2 => UpgradeType::AddBall,
            3 => UpgradeType::SuperBall,
            4 => UpgradeType::ExtraLife,
            _ => UpgradeType::SpaceInvader,
        });

        Self {
            upgrade_type,
            rect: Rect::new(
                originating_block.x,
                originating_block.y,
                originating_block.w,
                originating_block.h,
            ),
        }
    }
}
