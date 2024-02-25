use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use core::f32::consts::PI;

use crate::balls::BallsMovingState;

pub struct StickPlugin;

impl Plugin for StickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stick).add_systems(
            Update,
            move_stick.run_if(in_state(BallsMovingState::Moving)),
        );
    }
}

fn spawn_stick(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/pool-stick-diff.glb#Scene0"),
            transform: Transform {
                rotation: Quat::from_rotation_z(PI / 2.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(10.0, 10.0, 1.0));
}

fn move_stick() {
    println!("Bruh\nBruh2");
}
