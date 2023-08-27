use bevy::prelude::*;
use bevy_touch_camera::{TouchCameraPlugin, TouchCameraTag};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::DARK_GRAY))
        .add_plugins((DefaultPlugins, TouchCameraPlugin::default()))
        .add_systems(Startup, setup)
        .add_systems(Update, touch_indicators)
        .run();
}

#[derive(Component)]
struct TouchIndicatorTag;

fn setup(mut cmds: Commands, asset_server: Res<AssetServer>) {
    cmds.spawn((Camera2dBundle::default(), TouchCameraTag));

    // touch indicators
    for _ in 0..2 {
        // UI box
        cmds.spawn((
            NodeBundle {
                style: Style {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::WHITE.into(),
                // background_color: Color::rgba(1.0, 0.5, 0.5, 0.7).into(),
                ..default()
            },
            UiImage::new(asset_server.load("pointer-hand-svgrepo-com.png")), // TODO add to toml ignore
            TouchIndicatorTag,
        ));
    }

    // scene objects
    cmds.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::PURPLE,
            custom_size: Some(Vec2::new(100.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });
}

fn touch_indicators(
    mut indicators_q: Query<(&mut Style), With<TouchIndicatorTag>>,
    touches_res: Res<Touches>,
) {
    for mut style in indicators_q.iter_mut() {
        style.display = Display::None;
    }

    for (touch, mut style) in touches_res.iter().zip(indicators_q.iter_mut()) {
        style.display = Display::Flex;
        style.left = Val::Px(touch.position().x);
        style.top = Val::Px(touch.position().y);
    }
}
