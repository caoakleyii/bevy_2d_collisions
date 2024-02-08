use std::collections::HashMap;

use bevy::{
    ecs::{bundle::Bundle, component::Component, entity::Entity},
    math::Vec2,
};

#[derive(Bundle, Default, Debug)]
pub struct CollisionBundle {
    pub collision_box: CollisionBox,

    pub collision_group: CollisionGroup,

    pub collisions: Collisions,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq)]
pub struct CollisionBox {
    pub size: Vec2,

    pub disabled: bool,
}

#[derive(Component, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CollisionGroup {
    pub layer: u32,
    pub mask: u32,
}

impl CollisionGroup {
    pub fn can_see(&self, other: &CollisionGroup) -> bool {
        self.mask & other.layer != 0
    }
}

#[derive(Component, Default, Debug)]
pub struct Collisions {
    pub map: HashMap<Entity, Collision>,
}

#[derive(Debug, Component, Clone, Copy)]
pub struct Collision {
    pub entity: Entity,
}

impl Collision {
    pub fn new(entity: Entity) -> Self {
        Self { entity }
    }
}
