use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub const WALL_LENGTH: f32 = 35.5;
const WALL_WIDTH: f32 = 68.0;
pub fn setup_physics(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(30.5, 0.1, 64.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)))
        .insert(Friction::coefficient(1.0));

    // Create the walls
    const WALL_HEIGHT: f32 = 50.0; // Like really tall, man
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
}
