use bevy::ecs::{entity::Entity, event::Event};

#[derive(Event, Debug)]
pub struct CollisionBegin {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

#[derive(Event, Debug)]
pub struct CollisionEnd {
    pub entity_a: Entity,
    pub entity_b: Entity,
}
