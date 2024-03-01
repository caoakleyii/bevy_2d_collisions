use bevy::ecs::{entity::Entity, event::Event};

use crate::CollisionDirection;

#[derive(Event, Debug)]
pub struct CollisionBegin {
    pub entity: Entity,
    pub detected: Entity,
    pub location: CollisionDirection,
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub detected: Entity,
    pub location: CollisionDirection,
}

#[derive(Event, Debug)]
pub struct CollisionEnd {
    pub entity: Entity,
    pub left: Entity,
}
