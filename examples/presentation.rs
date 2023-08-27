use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy::sprite::Mesh2dHandle;
use bevy_touch_camera::{TouchCameraPlugin, TouchCameraTag};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            TouchCameraPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, touch_indicators)
        .run();
}

fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    cmds.spawn((Camera2dBundle::default(), TouchCameraTag));

    let mesh = meshes.add(shape::Circle::new(50.).into()).into();
    let material = materials.add(ColorMaterial::from(Color::PURPLE));
    for i in 0..1 {
        // Circle
        cmds.spawn(MaterialMesh2dBundle {
            mesh: mesh.clone(),
            material: material.clone(),
            // transform: Transform::from_translation(Vec3::new(-150., 0., 0.)),
            ..default()
        });
    }

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

fn touch_indicators(mut cmds: Commands, ) {

}
