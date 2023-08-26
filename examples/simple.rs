use bevy::prelude::*;
use bevy_touch_camera::{TouchCameraPlugin, TouchCameraConfig};

fn main() {
   App::new()
       .add_plugins((
            DefaultPlugins,
            TouchCameraPlugin {
                config: TouchCameraConfig {
                    zoom_sensitivity: 0.01,
                    ..Default::default()
                }
            },
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());

    // Rectangle
    cmds.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
}