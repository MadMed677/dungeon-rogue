use std::collections::HashSet;

use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;

/// Medium animation duration should
///  be from `0.08` to `0.12` seconds
#[derive(Component)]
pub struct MediumAnimation {
    pub timer: Timer,
}

/// Lange animation duration should
///  be from `0.13` to `0.17` seconds
#[derive(Component)]
pub struct LangeAnimation {
    pub timer: Timer,
}

/// Fast animation duration should
///  be `0.05`
#[derive(Component)]
pub struct FastAnimation {
    pub timer: Timer,
}

/// X-treme fast animation duration should
///  be from `0.03` to `0.04`
#[derive(Component)]
pub struct XFastAnimation {
    pub timer: Timer,
}

#[derive(Component, Default, Inspectable)]
/// Describes that entity on move or not
pub struct OnMove(pub bool);

#[derive(Component, Inspectable, Eq, PartialEq, Clone, Debug)]
pub enum MovementDirection {
    Left,
    Right,
}

#[derive(Component, Copy, Clone, Debug, Default)]
/// Describes that this element
///  might be used for `Climber` entities
pub struct Climbable;

#[derive(Component, Clone, Debug, Default)]
/// Describes that this entity
///  may interact with `Climbable` elements
pub struct Climber {
    /// Describes that climber faced intersection with
    ///  `Climbable` element and it's ready to climb
    /// Contains a list of all intersaction elements
    ///  which the Climber has a contact with
    pub intersaction_elements: HashSet<Entity>,

    // Describes that climber is in climbing process
    pub climbing: bool,
}

#[derive(Component, Clone, Debug, Default, Inspectable)]
pub struct Health {
    /// Describes current health
    pub current: i32,

    /// Describes maximum health
    pub max: i32,
}

/// If `true` than the entity in attack state
/// Otherwise - no
#[derive(Debug, Component, Inspectable)]
pub struct Attacks(pub bool);

/// Describes that this entity might have receive `Attacks`
#[derive(Debug, Component, Inspectable)]
pub struct Attackable;

#[derive(Component, Inspectable, Debug)]
pub struct Speed(pub f32);
