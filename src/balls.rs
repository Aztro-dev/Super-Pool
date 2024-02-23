use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_balls)
            .add_systems(Update, check_scored_ball);
    }
}

enum Side {
    Stripes,
    Solids,
}

enum Superpower {
    ExtraPower,
    SecondLife,
    RemoveBall,
}

#[derive(Component)]
struct Ball {
    side: Option<Side>,
    superpower: Option<Superpower>,
}

const BALL_SIZE: f32 = 1.8;

fn spawn_balls(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Create the balls
    for i in 0..15 {
        // 15 balls
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(Sphere::new(BALL_SIZE)),
                material: materials.add(StandardMaterial {
                    base_color: Color::RED,
                    ..default()
                }),
                ..default()
            })
            .insert(Ball {
                side: Some(Side::Solids),
                superpower: None,
            })
            .insert(RigidBody::Dynamic)
            .insert(Collider::ball(BALL_SIZE))
            .insert(Restitution::coefficient(0.7))
            .insert(Damping {
                linear_damping: 0.5,
                angular_damping: 0.5,
            })
            .insert(TransformBundle::from(Transform::from_xyz(
                i as f32 * BALL_SIZE,
                BALL_SIZE / 2.0,
                0.0,
            )))
            .insert(Friction::coefficient(1.0))
            .insert(Velocity {
                linvel: Vec3::new(100.0, 0.0, 100.0),
                ..default()
            })
            .insert(Sleeping::disabled());
    }
}

fn check_scored_ball(mut commands: Commands, ball_query: Query<(Entity, &Transform), With<Ball>>) {
    for (entity, transform) in ball_query.iter() {
        if transform.translation.y < 0.0 {
            println!("Scored!");
            commands.entity(entity).despawn_recursive();
        }
    }
}
