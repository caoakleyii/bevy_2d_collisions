use std::collections::HashMap;

use bevy::ecs::system::Resource;

use crate::CollisionMapKey;

#[derive(Default, Debug, Resource)]
pub struct CollisionMap {
    pub map: HashMap<CollisionMapKey, bool>,
}
