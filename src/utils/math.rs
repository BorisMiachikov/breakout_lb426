use bevy::prelude::*;

/// Проверка пересечения двух AABB (Axis-Aligned Bounding Box)
pub fn aabb_collision(
    a_pos: Vec3,
    a_size: Vec2,
    b_pos: Vec3,
    b_size: Vec2,
) -> bool {
    (a_pos.x - b_pos.x).abs() < (a_size.x + b_size.x) / 2.0
        && (a_pos.y - b_pos.y).abs() < (a_size.y + b_size.y) / 2.0
}