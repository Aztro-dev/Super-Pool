use bevy::prelude::*;

use crate::stick::ShotPower;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, update_ui);
    }
}

#[derive(Component)]
struct UiComponent;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(5.0),
                height: Val::Percent(0.0),
                left: Val::Percent(90.0), // 30% - 40% - 30%
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                display: Display::Flex,
                ..default()
            },
            background_color: BackgroundColor(Color::GREEN),
            border_color: BorderColor(Color::BLACK),
            ..default()
        })
        .insert(UiComponent);
}

fn update_ui(
    mut shot_power: ResMut<ShotPower>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut ui_query: Query<&mut Style, With<UiComponent>>,
) {
    if keyboard.pressed(KeyCode::ArrowUp) {
        shot_power.0 += 0.01;
    }
    if keyboard.pressed(KeyCode::ArrowDown) {
        shot_power.0 -= 0.01;
    }

    shot_power.0 = shot_power.0.clamp(0.0, 1.0);

    if let Some(mut style) = ui_query.get_single_mut().ok() {
        style.height = Val::Percent(shot_power.0 * 80.0);
    }
}
