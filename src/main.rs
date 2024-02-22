use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

mod setup_physics;
use setup_physics::setup_physics;

mod balls;
use balls::BallPlugin;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            BallPlugin,
            LookTransformPlugin,
            FpsCameraPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
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
