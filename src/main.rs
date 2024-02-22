use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LookTransformPlugin)
        .add_plugins(FpsCameraPlugin::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup_graphics)
        .add_systems(Startup, setup_physics)
        .run();
}

fn setup_graphics(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn(Camera3dBundle {
            ..Default::default()
        })
        .insert(FpsCameraBundle::new(
            FpsCameraController {
                translate_sensitivity: 50.0,
                ..default()
            },
            Vec3::new(0.0, 100.0, 0.0),
            Vec3::ZERO,
            Vec3::Y,
        ));

    commands.spawn(SceneBundle {
        scene: asset_server.load("models/luxury-pool-table.glb#Scene0"),
        transform: Transform {
            translation: Vec3::new(0.0, -44.5, 0.0),
            scale: Vec3::splat(50.0),
            ..default()
        },
        ..default()
    });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 500.0,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::WHITE,
            illuminance: 10_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0.0, 500.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

const BALL_SIZE: f32 = 2.0;

fn setup_physics(mut commands: Commands) {
    const WALL_LENGTH: f32 = 35.5;
    const WALL_WIDTH: f32 = 68.0;
    commands
        .spawn(Collider::cuboid(30.5, 0.1, 64.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(Friction::coefficient(10.0));
    // Create the walls
    const WALL_HEIGHT: f32 = 5.0;
    commands
        .spawn(Collider::cuboid(WALL_LENGTH, WALL_HEIGHT, 0.01))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT / 2.0,
            -WALL_WIDTH,
        )));
    commands
        .spawn(Collider::cuboid(WALL_LENGTH, WALL_HEIGHT, 0.01))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT / 2.0,
            WALL_WIDTH,
        )));

    commands
        .spawn(Collider::cuboid(0.01, WALL_HEIGHT, WALL_WIDTH))
        .insert(TransformBundle::from(Transform::from_xyz(
            WALL_LENGTH,
            WALL_HEIGHT / 2.0,
            0.0,
        )));
    commands
        .spawn(Collider::cuboid(0.01, WALL_HEIGHT, WALL_WIDTH))
        .insert(TransformBundle::from(Transform::from_xyz(
            -WALL_LENGTH,
            WALL_HEIGHT / 2.0,
            0.0,
        )));

    commands
        .spawn(Collider::cuboid(0.01, WALL_HEIGHT, WALL_WIDTH / 2.5))
        .insert(TransformBundle::from(Transform::from_xyz(
            WALL_LENGTH - 5.0,
            WALL_HEIGHT / 2.0,
            WALL_WIDTH / 2.5 + 5.0,
        )));

    commands
        .spawn(Collider::cuboid(0.01, WALL_HEIGHT, WALL_WIDTH / 2.5))
        .insert(TransformBundle::from(Transform::from_xyz(
            -(WALL_LENGTH - 5.0),
            WALL_HEIGHT / 2.0,
            WALL_WIDTH / 2.5 + 5.0,
        )));

    commands
        .spawn(Collider::cuboid(0.01, WALL_HEIGHT, WALL_WIDTH / 2.5))
        .insert(TransformBundle::from(Transform::from_xyz(
            WALL_LENGTH - 5.0,
            WALL_HEIGHT / 2.0,
            -(WALL_WIDTH / 2.5 + 5.0),
        )));

    commands
        .spawn(Collider::cuboid(0.01, WALL_HEIGHT, WALL_WIDTH / 2.5))
        .insert(TransformBundle::from(Transform::from_xyz(
            -(WALL_LENGTH - 5.0),
            WALL_HEIGHT / 2.0,
            -(WALL_WIDTH / 2.5 + 5.0),
        )));

    commands
        .spawn(Collider::cuboid(WALL_LENGTH * 0.75, WALL_HEIGHT, 0.01))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT / 2.0,
            -(WALL_WIDTH - 5.0),
        )));

    commands
        .spawn(Collider::cuboid(WALL_LENGTH * 0.75, WALL_HEIGHT, 0.01))
        .insert(TransformBundle::from(Transform::from_xyz(
            0.0,
            WALL_HEIGHT / 2.0,
            WALL_WIDTH - 5.0,
        )));

    /* Create the bouncing ball. */
    commands
        .spawn(RigidBody::Dynamic)
        .insert(Collider::ball(BALL_SIZE))
        .insert(Restitution::coefficient(0.7))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)))
        .insert(Friction::coefficient(10.0))
        .insert(Velocity {
            linvel: Vec3::new(10.0, 0.0, 10.0),
            ..default()
        })
        .insert(Sleeping::disabled());
}
