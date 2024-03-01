use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::setup_physics::WALL_LENGTH;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum BallsMovingState {
    Moving,
    #[default]
    NotMoving,
}

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BallsMovingState>()
            .add_systems(Startup, spawn_balls)
            .add_systems(Update, (check_scored_ball, check_ball_moving));
    }
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Side {
    Stripes,
    Solids,
    #[default]
    Neither,
}

#[derive(Debug, Default, Eq, PartialEq)]
pub enum Superpower {
    ExtraPower,
    SecondLife,
    RemoveBall,
    #[default]
    None,
}

impl Superpower {
    fn random() -> Self {
        let result = fastrand::u8(0..3);
        match result % 3 {
            0 => Superpower::ExtraPower,
            1 => Superpower::SecondLife,
            2 => Superpower::RemoveBall,
            _ => Superpower::None,
        }
    }
}

impl Side {
    fn random() -> Self {
        let result = fastrand::u8(0..2);
        match result {
            0 => Side::Stripes,
            1 => Side::Solids,
            _ => Side::Neither,
        }
    }
}

#[derive(Component, Default)]
pub struct Ball {
    pub side: Side,
    pub superpower: Superpower,
}

pub const BALL_SIZE: f32 = 1.8;

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Sphere::new(BALL_SIZE));
    let material = materials.add(StandardMaterial {
        base_color: Color::RED,
        ..default()
    });

    // Create the balls
    for i in 0..5 {
        // 5 rows
        for ii in (0..=i).rev() {
            // 15 balls
            commands
                .spawn(PbrBundle {
                    mesh: mesh.clone(),
                    material: material.clone(),
                    ..default()
                })
                .insert(ball(
                    Ball {
                        side: Side::random(),
                        superpower: Superpower::random(),
                    },
                    Vec2::new(
                        2.0 * BALL_SIZE * i as f32 - BALL_SIZE * ii as f32 - 4.0 * BALL_SIZE,
                        2.0 * BALL_SIZE * ii as f32 - WALL_LENGTH,
                    ),
                ));
        }
    }
    // Cue ball
    commands
        .spawn(PbrBundle {
            mesh: mesh.clone(),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..default()
            }),
            ..default()
        })
        .insert(ball(Ball::default(), Vec2::new(0.0, 2.0 * BALL_SIZE)))
        .insert(ColliderDebugColor(Color::WHITE));
}

fn check_scored_ball(mut commands: Commands, ball_query: Query<(Entity, &Transform, &Ball)>) {
    for (entity, transform, ball) in ball_query.iter() {
        if transform.translation.y < 0.0 {
            println!(
                "Scored {:?} with superpower {:?}!",
                ball.side, ball.superpower
            );
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn check_ball_moving(
    ball_query: Query<&Sleeping, With<Ball>>,
    mut ball_moving_state: ResMut<NextState<BallsMovingState>>,
) {
    for sleeping in ball_query.iter() {
        if !sleeping.sleeping {
            ball_moving_state.set(BallsMovingState::Moving);
            return;
        }
    }
    ball_moving_state.set(BallsMovingState::NotMoving);
}

fn ball(
    ball: Ball,
    pos: Vec2,
) -> (
    Ball,
    RigidBody,
    Collider,
    Restitution,
    Damping,
    TransformBundle,
    Friction,
    Velocity,
    Sleeping,
    Ccd,
    GravityScale,
) {
    (
        ball,
        RigidBody::Dynamic,
        Collider::ball(BALL_SIZE),
        Restitution::coefficient(0.7),
        Damping {
            linear_damping: 0.5,
            angular_damping: 0.5,
        },
        TransformBundle::from(Transform::from_xyz(pos.x, BALL_SIZE / 2.0, pos.y)),
        Friction::coefficient(1.0),
        Velocity::default(),
        Sleeping {
            linear_threshold: 5.0,
            ..default()
        },
        Ccd::enabled(),
        GravityScale(10.0),
    )
}
