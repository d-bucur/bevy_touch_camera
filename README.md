![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)
[![0.11](https://img.shields.io/badge/Bevy-0.11-blue)](https://crates.io/crates/bevy/0.11.2)
[![Crate](https://img.shields.io/crates/v/bevy_touch_camera.svg)](https://crates.io/crates/bevy_touch_camera)
[![Doc](https://docs.rs/bevy_touch_camera/badge.svg)](https://docs.rs/bevy_touch_camera)
[![CI](https://github.com/d-bucur/bevy_touch_camera/actions/workflows/ci.yaml/badge.svg)](https://github.com/d-bucur/bevy_touch_camera/actions/workflows/ci.yaml)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-release-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)

## 🎥 Bevy Touch Camera
A plugin for touch based camera movement. Supports one finger drag to pan and two finger pinch to zoom.

![](https://github.com/d-bucur/demos/raw/master/touch_camera.webp)

## 📄 Usage
Add `TouchCameraPlugin` to your app.
```rust
App::new().add_plugins((
    DefaultPlugins,
    TouchCameraPlugin::default()
)).run();
```

You can configure some of the parameters of the plugin. Changing configuration at runtime is currently not supported. [^1]
```rust
let config = TouchCameraConfig {
    drag_sensitivity: 2.,
    touch_time_min: 0.2,
    ..Default::default()
};
app.add_plugins((TouchCameraPlugin {config}))
```

The plugin will try to attach itself to a camera. This can be done in either one of the following ways:
1) Create a single `Camera` component before the `PostUpdate` schedule. The plugin will attach itself automatically to it.
2) Manually attach a `TouchCameraTag` component to the camera entity you want to be handled by the plugin. Useful if you have multiple active cameras or if method 1) is not possible.

## Limitations
- Plugin will always try to update the projection and the transform of the Camera. A separate mode is planned where updates are only written to a component so that the user can read them and mix them with other custom logic before applying to the camera.
- Only viewport based panning and scaling is supported currently. That means that your finger won't stay in exactly the same position of the world view like in common implementations.

## 🔗 Bevy compatibility
| bevy_touch_camera | bevy |
|-------------------|------|
| 0.1               | 0.11 |

## 🪪 License
Either one:
- [Apache 2](LICENSE-APACHE)
- [MIT](LICENSE-MIT)


[^1]: TODO maybe it is, need to test it and update example