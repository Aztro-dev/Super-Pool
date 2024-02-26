use crate::setup_physics::WALL_LENGTH;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::balls::{Ball, BallsMovingState, Side};

use core::f32::consts::PI;

pub struct StickPlugin;

impl Plugin for StickPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_stick).add_systems(
            Update,
            (move_stick, shoot_stick).run_if(in_state(BallsMovingState::NotMoving)),
        );
    }
}

#[derive(Component)]
struct Stick;

const STICK_LENGTH: f32 = 30.0;

fn spawn_stick(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/pool-stick-diff.glb#Scene0"),
            transform: Transform {
                translation: Vec3::new(0.0, 10.0, 3.0 * WALL_LENGTH / 2.0),
                rotation: Quat::from_rotation_y(PI / 2.0),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::KinematicPositionBased)
        .insert(AdditionalMassProperties::Mass(100.0))
        .insert(Collider::cuboid(STICK_LENGTH, 1.0, 1.0))
        .insert(Stick);
}

fn move_stick(
    mut stick_query: Query<&mut Transform, With<Stick>>,
    ball_query: Query<(&Ball, &Transform), Without<Stick>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if let Some(mut stick_transform) = stick_query.get_single_mut().ok() {
        // Could be optimized by only looking for the CueBall, but whatevs
        for (ball, ball_transform) in ball_query.iter() {
            if ball.side != Side::Neither {
                continue;
            }
            if keyboard.pressed(KeyCode::KeyR) {
                stick_transform.look_at(ball_transform.translation, Vec3::Y);
                stick_transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_rotation_y(time.delta_seconds() * 2.0),
                );
                stick_transform.rotate_local_y(PI / 2.0);
            } else if keyboard.pressed(KeyCode::KeyT) {
                stick_transform.look_at(ball_transform.translation, Vec3::Y);
                stick_transform.rotate_around(
                    Vec3::ZERO,
                    Quat::from_rotation_y(-time.delta_seconds() * 2.0),
                );
                stick_transform.rotate_local_y(PI / 2.0);
            }
            break;
        }
    }
}

fn shoot_stick(
    mut stick_query: Query<&mut Transform, With<Stick>>,
    ball_query: Query<(&Ball, &Transform), Without<Stick>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::Space) {
        if let Some(mut stick_transform) = stick_query.get_single_mut().ok() {
            // Could be optimized by only looking for the CueBall, but whatevs
            for (ball, ball_transform) in ball_query.iter() {
                if ball.side != Side::Neither {
                    continue;
                }
                stick_transform.translation = ball_transform.translation;
                break;
            }
        }
    }
}
