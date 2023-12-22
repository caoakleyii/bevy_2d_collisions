use std::collections::HashMap;

use bevy::{
    app::{App, Plugin, PreUpdate},
    ecs::{
        bundle::Bundle,
        component::Component,
        entity::Entity,
        event::{Event, EventWriter},
        system::Query,
    },
    math::Vec2,
    sprite::collide_aabb::{collide, Collision},
    transform::components::Transform,
};

#[derive(Bundle, Default, Debug)]
pub struct CollisionBundle {
    pub collision_box: CollisionBox,

    pub collision_group: CollisionGroup,
}

#[derive(Component, Default, Debug)]
pub struct CollisionBox {
    pub size: Vec2,

    pub disabled: bool,
}

#[derive(Component, Default, Debug)]
pub struct CollisionGroup {
    pub layer: u32,
    pub mask: u32,
}

impl CollisionGroup {
    pub fn can_see(&self, other: &CollisionGroup) -> bool {
        (self.mask & other.layer) != 0
    }
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub entity_a: Entity,
    pub entity_b: Entity,
}

pub struct CollisionsPlugin;

impl Plugin for CollisionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
        app.add_systems(PreUpdate, Self::update);
    }
}

impl CollisionsPlugin {
    fn update(
        mut ew_collision_events: EventWriter<CollisionEvent>,
        collision_boxes: Query<(Entity, &Transform, &CollisionBox, &CollisionGroup)>,
    ) {
        let mut checked_entities: HashMap<(Entity, Entity), bool> = HashMap::new();

        for (entity, transform, collision_box, group) in collision_boxes.iter() {
            for (other_entity, other_transform, other_box, other_group) in collision_boxes.iter() {
                if entity == other_entity {
                    continue;
                }

                if checked_entities.contains_key(&(entity, other_entity)) {
                    continue;
                }

                checked_entities.insert((entity, other_entity), true);
                checked_entities.insert((other_entity, entity), true);

                if collision_box.disabled || other_box.disabled {
                    continue;
                }

                if !group.can_see(other_group) || !other_group.can_see(group) {
                    continue;
                }

                let collision =
                    Self::check_collision(transform, collision_box, other_transform, other_box);
                if collision.is_some() {
                    ew_collision_events.send(CollisionEvent {
                        entity_a: entity,
                        entity_b: other_entity,
                    });
                    println!("Collision detected!");
                }
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
