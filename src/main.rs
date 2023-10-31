use bevy::{prelude::*, window::WindowMode};

#[derive(Bundle)]
struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    velocity: Velocity,
    p: Player
}

#[derive(Resource)]
struct PlayerSprite;

#[derive(Resource, Default, Debug)]
struct ScreenEdgeCoordinates {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct Player;

#[derive(Component, Clone, Copy)]
struct Velocity {
    v: Vec3
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
                    primary_window: Some(Window {
                    title: "its a game or something".into(),
                    resolution: (800., 600.).into(),
                    mode: WindowMode::Windowed,
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),)
        .add_systems(Startup, setup)
        .add_systems(Update, update_screen_coords)
        .add_systems(FixedUpdate, move_velocity_entities)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    let player_texture = asset_server.load("img/test.png");
    let player_rot_rad = (359.0 * rand::random::<f32>()).to_radians();

    let player_speed = 5.0;

    commands.spawn(PlayerBundle {
        sprite_bundle: SpriteBundle {
            transform: Transform {
                translation: Vec3::new(50.0, 50.0, 0.0),
                rotation: Quat::from_rotation_z(player_rot_rad),
                ..default()
            },
            texture: player_texture,
            ..default()},
        p: Player {},
        velocity: Velocity { v: Vec3 {
            x: player_rot_rad.cos() * player_speed,
            y: player_rot_rad.sin() * player_speed,
            z: 0.0 }}
    });

    commands.init_resource::<ScreenEdgeCoordinates>();
}

fn update_screen_coords(
    mut coords: ResMut<ScreenEdgeCoordinates>,
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut gizmos: Gizmos) {
    let (camera, camera_transform) = q_camera.single();
    let cam_rect = camera.logical_viewport_rect();
    match cam_rect {
        Some(r) => {
            let min = camera.viewport_to_world_2d(camera_transform, r.min).unwrap();
            let max = camera.viewport_to_world_2d(camera_transform, r.max).unwrap();
            coords.left = min.x;
            coords.right = max.x;
            coords.top = min.y;
            coords.bottom = max.y},
        None => {
            coords.left = 0.0;
            coords.right = 50.0;
            coords.top = 50.0;
            coords.bottom = 0.0},
    }

    // gizmos.rect_2d(Vec2::new(coords.right, coords.bottom), 0.0, Vec2::new((coords.left - coords.right).abs(), (coords.top - coords.bottom).abs()), Color::PINK);
    gizmos.circle_2d(Vec2::new(coords.left, coords.top), 5.0, Color::PINK);
    gizmos.circle_2d(Vec2::new(coords.left, coords.bottom), 5.0, Color::PINK);
    gizmos.circle_2d(Vec2::new(coords.right, coords.top), 5.0, Color::PINK);
    gizmos.circle_2d(Vec2::new(coords.right, coords.bottom), 5.0, Color::PINK);
}

fn move_velocity_entities(
    edges: Res<ScreenEdgeCoordinates>,
    mut q_ent: Query<(&mut Transform, &mut Velocity), With<Velocity>>,
    mut gizmos: Gizmos) {
    for (mut tr, mut vl) in q_ent.iter_mut() {
        let mut new_vel = vl.v;

        let new_x_pos = tr.translation.x + vl.v.x;
        let new_y_pos = tr.translation.y + vl.v.y;

        if new_x_pos < edges.left || new_x_pos > edges.right {
            new_vel.x *= -1.0;
        }

        if new_y_pos < edges.bottom || new_y_pos > edges.top {
            new_vel.y *= -1.0;
        }

        let new_pos = Vec3 {
            x: tr.translation.x + new_vel.x,
            y: tr.translation.y + new_vel.y,
            z: tr.translation.z + new_vel.z,
        };

        tr.translation = new_pos;
        vl.v = new_vel;
        tr.rotation = Quat::from_rotation_z(new_vel.y.atan2(new_vel.x))
    }
}
