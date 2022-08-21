use bevy_rapier2d::prelude::CollisionGroups;

pub const ATTACK_COLLISION_GROUP: CollisionGroups = CollisionGroups::new(0b1101, 0b0100);
