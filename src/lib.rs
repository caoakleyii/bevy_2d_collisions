use bevy::{
    app::{App, Plugin, PreUpdate},
    ecs::{
        entity::Entity,
        event::{EventReader, EventWriter},
        system::{Query, ResMut},
    },
    sprite::collide_aabb::{collide, Collision},
    transform::components::Transform,
};
use components::{CollisionBox, CollisionGroup, Collisions};
use events::{CollisionBegin, CollisionEnd, CollisionEvent};
use resources::CollisionMap;
use std::collections::HashMap;
use std::hash::{Hash as HashTrait, Hasher};
pub mod components;
pub mod events;
pub mod resources;

#[derive(Clone, Copy, Debug)]
pub struct CollisionMapKey {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

impl CollisionMapKey {
    pub fn new(entity_a: Entity, entity_b: Entity) -> Self {
        Self { entity_a, entity_b }
    }
}

impl Eq for CollisionMapKey {}

impl PartialEq for CollisionMapKey {
    fn eq(&self, other: &Self) -> bool {
        (self.entity_a == other.entity_a && self.entity_b == other.entity_b)
            || (self.entity_a == other.entity_b && self.entity_b == other.entity_a)
    }
}

impl HashTrait for CollisionMapKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if self.entity_a < self.entity_b {
            self.entity_a.hash(state);
            self.entity_b.hash(state);
        } else {
            self.entity_b.hash(state);
            self.entity_a.hash(state);
        }
    }
}

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionBegin>();
        app.add_event::<CollisionEvent>();
        app.add_event::<CollisionEnd>();

        app.add_systems(
            PreUpdate,
            (Self::update, Self::collision_begin, Self::collision_end),
        );

        app.insert_resource(CollisionMap::default());
    }
}

impl CollisionsPlugin {
    fn update(
        mut ew_collision_events: EventWriter<CollisionEvent>,
        mut ew_collision_begin_events: EventWriter<CollisionBegin>,
        mut eq_collision_end_events: EventWriter<CollisionEnd>,
        mut collision_map: ResMut<CollisionMap>,
        collision_boxes: Query<(Entity, &Transform, &CollisionBox, &CollisionGroup)>,
    ) {
        let mut checked_entities: HashMap<CollisionMapKey, bool> = HashMap::new();
        for (entity, transform, collision_box, group) in &collision_boxes {
            for (other_entity, other_transform, other_box, other_group) in &collision_boxes {
                if entity == other_entity {
                    continue;
                }

                let key = CollisionMapKey::new(entity, other_entity);
                if checked_entities.contains_key(&key) {
                    continue;
                } else {
                    checked_entities.insert(key, true);
                }

                if collision_box.disabled || other_box.disabled {
                    // TODO: Bug here, if one of the boxes is disabled, the end collision will not be detected
                    continue;
                }

                if !group.can_see(other_group) || !other_group.can_see(group) {
                    continue;
                }

                let collision =
                    Self::check_collision(transform, collision_box, other_transform, other_box);
                if collision.is_some() {
                    if collision_map.map.contains_key(&key) {
                        ew_collision_events.send(CollisionEvent {
                            entity_a: entity,
                            entity_b: other_entity,
                        });
                    } else {
                        ew_collision_begin_events.send(CollisionBegin {
                            entity_a: entity,
                            entity_b: other_entity,
                        });

                        collision_map.map.insert(key, true);
                    }
                } else {
                    if collision_map.map.contains_key(&key) {
                        eq_collision_end_events.send(CollisionEnd {
                            entity_a: entity,
                            entity_b: other_entity,
                        });
                        collision_map.map.remove(&key);
                    }
                }
            }
        }
    }

    fn collision_begin(
        mut er_collision_begin_events: EventReader<CollisionBegin>,
        mut query: Query<&mut Collisions>,
    ) {
        for event in er_collision_begin_events.read() {
            if let Ok(mut collisions) = query.get_mut(event.entity_a) {
                collisions
                    .map
                    .insert(event.entity_b, components::Collision::new(event.entity_b));
            }

            if let Ok(mut collisions) = query.get_mut(event.entity_b) {
                collisions
                    .map
                    .insert(event.entity_a, components::Collision::new(event.entity_a));
            }
        }
    }

    fn collision_end(
        mut er_collision_end_events: EventReader<CollisionEnd>,
        mut query: Query<&mut Collisions>,
    ) {
        for event in er_collision_end_events.read() {
            if let Ok(mut collisions) = query.get_mut(event.entity_a) {
                collisions.map.remove(&event.entity_b);
            }

            if let Ok(mut collisions) = query.get_mut(event.entity_b) {
                collisions.map.remove(&event.entity_a);
            }
        }
    }

    fn check_collision(
        a_transform: &Transform,
        a_box: &CollisionBox,
        b_transform: &Transform,
        b_box: &CollisionBox,
    ) -> Option<Collision> {
        collide(
            a_transform.translation,
            a_box.size,
            b_transform.translation,
            b_box.size,
        )
    }
}
