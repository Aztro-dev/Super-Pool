use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::balls::{Ball, BallsMovingState, Side};

pub struct StickPlugin;

impl Plugin for StickPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShotPower>()
            .add_plugins(DefaultRaycastingPlugin)
            .add_systems(Startup, setup)
            .add_systems(
                Update,
                (
                    shoot_stick.run_if(in_state(BallsMovingState::NotMoving)),
                    show_current_shot_position,
                ),
            );
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Sphere::new(0.5).mesh().ico(7).unwrap()),
            material: materials.add(StandardMaterial {
                base_color: Color::GREEN,
                ..default()
            }),
            visibility: Visibility::Hidden,
            ..default()
        })
        .insert(ShotIndicator);
}

#[derive(Resource)]
pub struct ShotPower(pub f32);

impl Default for ShotPower {
    fn default() -> Self {
        Self(1.0)
    }
}

const SHOOTING_SPEED: f32 = 5_000.0;
const MIN_SHOOTING_SPEED: f32 = 200.0;

#[derive(Component)]
struct ShotIndicator;

fn show_current_shot_position(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    ball_query: Query<(Entity, &Ball), With<Ball>>,
    mut shot_query: Query<(&mut Transform, &mut Visibility), With<ShotIndicator>>,
    ball_moving_state: Res<State<BallsMovingState>>,
) {
    let (mut transform, mut visibility) = shot_query.get_single_mut().unwrap();
    if ball_moving_state.eq(&BallsMovingState::Moving) {
        *visibility = Visibility::Hidden;
        return;
    }
    if let Some(cursor_ray) = **cursor_ray {
        for (entity, ball) in ball_query.iter() {
            if ball.side != Side::Neither {
                continue;
            }
            let data = raycast.cast_ray(
                cursor_ray,
                &RaycastSettings {
                    filter: &|e| e.index() == entity.index(),
                    ..default()
                },
            );
            if data.is_empty() {
                *visibility = Visibility::Hidden;
                return;
            }
            *visibility = Visibility::Visible;
            let intersection_data = data[0].clone().1;
            transform.translation = intersection_data.position();
            let direction = -intersection_data.normal();
            transform.rotation =
                Quat::from_euler(EulerRot::XYZ, direction.x, direction.y, direction.z);
            return;
        }
    }
}

fn shoot_stick(
    mut commands: Commands,
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    ball_query: Query<(Entity, &Ball, &Transform), With<Ball>>,
    mouse: Res<ButtonInput<MouseButton>>,
    shot_power: Res<ShotPower>,
) {
    if !mouse.pressed(MouseButton::Left) {
        return;
    }
    if let Some(cursor_ray) = **cursor_ray {
        for (entity, ball, ball_transform) in ball_query.iter() {
            if ball.side != Side::Neither {
                continue;
            }
            let data = raycast.cast_ray(
                cursor_ray,
                &RaycastSettings {
                    filter: &|e| e.index() == entity.index(),
                    ..default()
                },
            );
            if data.is_empty() {
                return;
            }
            let intersection_data = &data[0].1;
            let impulse_direction =
                -(shot_power.0 * SHOOTING_SPEED + MIN_SHOOTING_SPEED) * intersection_data.normal();
            commands.entity(entity).insert(ExternalImpulse::at_point(
                impulse_direction * Vec3::new(1.0, 0.0, 1.0), // Disable y
                intersection_data.position(),
                ball_transform.translation,
            ));
            return;
        }
    }
}
