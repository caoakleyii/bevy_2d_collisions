use bevy::{prelude::*, window::PrimaryWindow};
use bevy_2d_collisions::{
    components::{CollisionBox, CollisionBundle, CollisionGroup},
    events::CollisionBegin,
    CollisionsPlugin,
};

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, CollisionsPlugin));

    app.add_systems(Startup, (spawn_player, spawn_enemy));

    app.add_systems(
        Update,
        (collision_events, move_inputs, shoot_inputs, physics),
    );

    app.run();
}

#[derive(Component, Default, Debug)]
struct Player;

#[derive(Component, Default, Debug)]
struct Enemy;

#[derive(Component, Default, Debug)]
struct Velocity(Vec2);

fn spawn_player(asset_server: Res<AssetServer>, mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scale = 0.5;
    commands.spawn(camera_bundle);

    let texture_handle = asset_server.load("player.png");
    commands
        .spawn(SpriteBundle {
            texture: texture_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            ..Default::default()
        })
        .insert(Velocity(Vec2::new(0.0, 0.0)))
        .insert(CollisionBundle {
            collision_box: CollisionBox {
                size: Vec2::new(32.0, 32.0),
                ..Default::default()
            },
            collision_group: CollisionGroup { layer: 0, mask: 2 },
            ..Default::default()
        })
        .insert(Player);
}

fn spawn_enemy(asset_server: Res<AssetServer>, mut commands: Commands) {
    let texture_handle = asset_server.load("enemy.png");

    for i in -5..5 {
        for j in -5..5 {
            commands
                .spawn(SpriteBundle {
                    texture: texture_handle.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        i as f32 * 32.0,
                        j as f32 * 32.0,
                        0.0,
                    )),
                    ..Default::default()
                })
                .insert(Velocity(Vec2::new(0.0, 0.0)))
                .insert(CollisionBundle {
                    collision_box: CollisionBox {
                        size: Vec2::new(32.0, 32.0),
                        ..Default::default()
                    },
                    collision_group: CollisionGroup { layer: 1, mask: 2 },
                    ..Default::default()
                })
                .insert(Enemy);
        }
    }
}

fn collision_events(mut events: EventReader<CollisionBegin>, mut command: Commands) {
    for event in events.read() {
        println!("{:?}", event);
        command.entity(event.entity_a).despawn();
        command.entity(event.entity_b).despawn();
    }
}

fn move_inputs(keyboard_input: Res<Input<KeyCode>>, mut query: Query<&mut Velocity, With<Player>>) {
    for mut velocity in query.iter_mut() {
        let mut fx = 0.0;
        let mut fy = 0.0;

        if keyboard_input.pressed(KeyCode::A) {
            fx -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::D) {
            fx += 1.0;
        }
        if keyboard_input.pressed(KeyCode::W) {
            fy += 1.0;
        }
        if keyboard_input.pressed(KeyCode::S) {
            fy -= 1.0;
        }

        let force = Vec2::new(fx, fy).normalize_or_zero();

        velocity.0 = force * 100.0;
    }
}

fn shoot_inputs(
    asset_server: Res<AssetServer>,
    mouse_input: Res<Input<MouseButton>>,
    mut command: Commands,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
    q_player: Query<&Transform, With<Player>>,
) {
    if !mouse_input.just_pressed(MouseButton::Left) {
        return;
    }

    // get the camera info and transform
    if let Ok((camera, camera_transform)) = q_camera.get_single() {
        // There is only one primary window, so we can similarly get it from the query:
        let window = q_window.single();

        // check if the cursor is inside the window and get its position
        // then, ask bevy to convert into world coordinates, and truncate to discard Z
        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            let player_transform = q_player.get_single().unwrap();

            let to = world_position;
            let from = player_transform.translation;
            let y = to.y - from.y;
            let x = to.x - from.x;
            let angle = y.atan2(x);

            let texture_handle = asset_server.load("bullet.png");

            command
                .spawn(SpriteBundle {
                    texture: texture_handle,
                    transform: Transform::from_translation(from),
                    ..Default::default()
                })
                .insert(Velocity(Vec2::new(
                    angle.cos() * 500.0,
                    angle.sin() * 500.0,
                )))
                .insert(CollisionBundle {
                    collision_box: CollisionBox {
                        size: Vec2::new(8.0, 8.0),
                        ..Default::default()
                    },
                    collision_group: CollisionGroup { layer: 2, mask: 1 },
                    ..Default::default()
                });
        }
    }
}

fn physics(dt: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, vel) in &mut query {
        transform.translation.x += vel.0.x * dt.delta_seconds();
        transform.translation.y += vel.0.y * dt.delta_seconds();
    }
}
