use bevy::{
    ecs::{entity::Entity, event::Event},
    sprite::collide_aabb::Collision,
};

#[derive(Event, Debug)]
pub struct CollisionBegin {
    pub entity: Entity,
    pub detected: Entity,
    pub location: Collision,
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity: Entity,
    pub detected: Entity,
    pub location: Collision,
}

#[derive(Event, Debug)]
pub struct CollisionEnd {
    pub entity: Entity,
    pub left: Entity,
}
