use macroquad::prelude::*;

enum UpgradeType {
    Magnet(Texture2D),
    BallMultiplier(Texture2D),
    AddBall(Texture2D),
    SuperBall(Texture2D),
}

struct Upgrade {
    upgrade_type: UpgradeType,
    rect: Rect,
}