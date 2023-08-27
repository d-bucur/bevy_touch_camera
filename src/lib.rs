//! A plugin for touch based camera movement. Supports one finger drag to pan and two finger pinch to zoom.
//!
//! ## How to use
//! Add [`TouchCameraPlugin`] to your app.
//! ```ignore
//! App::new().add_plugins((
//!     DefaultPlugins,
//!     TouchCameraPlugin::default()
//! )).run();
//! ```
//!
//! You can configure some of the parameters of the plugin by passing a [`TouchCameraConfig`] object.
//!
//! Changing configuration at runtime is currently not supported. [^1]
//! ```ignore
//! let config = TouchCameraConfig {
//!     drag_sensitivity: 2.,
//!     touch_time_min: 0.2,
//!     ..Default::default()
//! };
//! app.add_plugins((TouchCameraPlugin {config}))
//! ```
//!
//! The plugin will try to attach itself to a camera. This can be done in either one of the following ways:
//! 1) Create a single `Camera` component before the `PostUpdate` schedule. The plugin will attach itself automatically to it.
//! 2) Manually attach a `TouchCameraTag` component to the camera entity you want to be handled by the plugin.Useful if you have multiple active cameras or if method 1) is not possible.
//!
//! [^1]: TODO maybe it is, need to test it and update example
mod plugin;

pub use plugin::*;
