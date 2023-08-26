use bevy::prelude::*;
use bevy_touch_camera::TouchCameraPlugin;

fn main() {
   App::new()
       .add_plugins((
            DefaultPlugins,
            TouchCameraPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut cmds: Commands) {
    cmds.spawn(Camera2dBundle::default());
}