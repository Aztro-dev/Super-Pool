use crate::{balls::BALL_SIZE, setup_physics::WALL_LENGTH};
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
    let mut transform = Transform {
        translation: Vec3::new(0.0, 10.0, 3.0 * WALL_LENGTH / 2.0),
        ..default()
    }
    .looking_at(Vec3::new(0.0, 0.0, 2.0 * BALL_SIZE), Vec3::Y); // Cue Ball pos
    transform.rotate_local_y(PI / 2.0);

    commands
        .spawn(SceneBundle {
            scene: asset_server.load("models/pool-stick-diff.glb#Scene0"),
            transform,
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
            stick_transform.translation.y = 10.0;
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
            return;
        }
    }
}

const SHOOT_SPEED: f32 = 1.0;

fn shoot_stick(
    mut stick_query: Query<&mut Transform, With<Stick>>,
    ball_query: Query<(&Ball, &Transform), Without<Stick>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    if keyboard.pressed(KeyCode::Space) {
        if let Some(mut stick_transform) = stick_query.get_single_mut().ok() {
            // Could be optimized by only looking for the CueBall, but whatevs
            for (ball, ball_transform) in ball_query.iter() {
                if ball.side != Side::Neither {
                    continue;
                }
                if ball_transform
                    .translation
                    .distance(stick_transform.translation)
                    - STICK_LENGTH
                    < 1.0
                {
                    stick_transform.translation.y = 10.0;
                    stick_transform.translation.z = ball_transform.translation.z;
                    stick_transform.look_at(ball_transform.translation, Vec3::Y); // Cue Ball pos
                    stick_transform.rotate_local_y(PI / 2.0);
                }
                let direction = stick_transform.right();
                stick_transform.translation += direction * SHOOT_SPEED * time.delta_seconds().exp();
                return;
            }
        }
    }
}
