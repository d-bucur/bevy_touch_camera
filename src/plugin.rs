use bevy::{
    input::touch,
    prelude::{
        error, info, App, Camera, Commands, Component, Entity, OrthographicProjection, Plugin,
        PostStartup, Query, Res, ResMut, Resource, Touches, Transform, Update, Vec2, Vec3, With,
    },
    time::Time,
};

/// A plugin that will update camera movement based on `Touch` gestures that Bevy provides
#[derive(Default)]
pub struct TouchCameraPlugin {
    pub config: TouchCameraConfig,
}

impl Plugin for TouchCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(TouchTracker::default())
            .insert_resource(self.config.clone())
            .add_systems(PostStartup, setup)
            .add_systems(Update, touch_pan_zoom);
    }
}

/// Contains the configuration parameters for the plugin.
/// A copy of this will be attached as a `Resource` to the `App`.
#[derive(Resource, Clone)]
pub struct TouchCameraConfig {
    /// How far the camera will move relative to the touch drag distance. Higher is faster
    pub drag_sensitivity: f32,
    /// How much the camera will zoom relative to the pinch distance using two fingers. Higher means faster.
    /// At the moment the default is very low at 0.005 but this might change in the future
    pub zoom_sensitivity: f32,
    /// Minimum time before starting to pan in seconds. Useful to avoid panning on short taps
    pub touch_time_min: f32,
    /// Tolerance for pinch fingers moving in opposite directions (-1. .. 1.).
    /// Higher values make it more tolerant.
    /// Very low values not recommended as it would be overly sensitive
    pub opposites_tolerance: f32,
}

impl Default for TouchCameraConfig {
    fn default() -> Self {
        Self {
            drag_sensitivity: 1.,
            zoom_sensitivity: 0.005,
            touch_time_min: 0.01,
            opposites_tolerance: 0.,
        }
    }
}

/// This is the tag that the plugin will scan for and update its `Camera` component.
/// You can either attach it manually to your camera, or the plugin will try to attach it
/// to the default camera in the `PostStartup` schedule
#[derive(Component)]
pub struct TouchCameraTag;

#[derive(PartialEq, Default)]
enum GestureType {
    #[default]
    None,
    Pan,
    Pinch,
    PinchCancelled,
}

#[derive(Resource, Default)]
struct TouchTracker {
    pub camera_start_pos: Vec3,
    pub time_start_touch: f32,
    pub gesture_type: GestureType,

    // Keeps track of position on last frame.
    // This is different from Touch.last_position as that only updates when there has been a movement
    pub last_touch_a: Option<Vec2>,
    pub last_touch_b: Option<Vec2>,
}

fn setup(
    tag_query: Query<&TouchCameraTag, With<Camera>>,
    camera_query: Query<Entity, With<Camera>>,
    mut commands: Commands,
) {
    if !tag_query.is_empty() {
        info!("TouchCameraPlugin initialized: found a tag attached to a camera");
        return;
    }
    if camera_query.is_empty() {
        error!("TouchCameraPlugin found no camera to use. Please attach the TouchCameraTag to a camera manually or create a camera before the PostUpdate schedule");
        return;
    }
    match camera_query.get_single() {
        Ok(camera) => {
            info!("TouchCameraPlugin initialized: using main camera");
            commands.entity(camera).insert(TouchCameraTag);
        },
        Err(_) => error!("TouchCameraPlugin found multiple cameras. Not sure which to use. Please attach the TouchCameraTag to a camera manually"),
    }
}

fn touch_pan_zoom(
    touches_res: Res<Touches>,
    mut camera_q: Query<(&mut Transform, &mut OrthographicProjection), With<TouchCameraTag>>,
    mut tracker: ResMut<TouchTracker>,
    config: Res<TouchCameraConfig>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut projection)) = camera_q.get_single_mut() else {
        return;
    };

    let touches: Vec<&touch::Touch> = touches_res.iter().collect();

    if touches.is_empty() {
        tracker.gesture_type = GestureType::None;
        tracker.last_touch_a = None;
        tracker.last_touch_b = None;
        return;
    }

    if touches_res.any_just_released() {
        tracker.gesture_type = GestureType::PinchCancelled;
        tracker.last_touch_a = None;
        tracker.last_touch_b = None;
    }

    if touches.len() == 2 {
        tracker.gesture_type = GestureType::Pinch;
        // complicated way to reset previous position to prevent some bugs. Should simplify
        let last_a = if tracker.last_touch_b.is_none() {
            touches[0].position()
        } else {
            tracker.last_touch_a.unwrap_or(touches[0].position())
        };
        let last_b = if tracker.last_touch_b.is_none() {
            touches[1].position()
        } else {
            tracker.last_touch_b.unwrap_or(touches[1].position())
        };

        let delta_a = touches[0].position() - last_a;
        let delta_b = touches[1].position() - last_b;
        let delta_total = (delta_a + delta_b).length();
        let dot_delta = delta_a.dot(delta_b);
        if dot_delta > config.opposites_tolerance {
            return;
        }

        let distance_current = touches[0].position() - touches[1].position();
        let distance_prev = touches[0].previous_position() - touches[1].previous_position();
        let pinch_direction = distance_prev.length() - distance_current.length();
        projection.scale +=
            pinch_direction.signum() * delta_total * config.zoom_sensitivity * projection.scale;

        tracker.last_touch_a = Some(touches[0].position());
        tracker.last_touch_b = Some(touches[1].position());
    } else if touches.len() == 1
        && matches!(tracker.gesture_type, GestureType::None | GestureType::Pan)
    {
        if tracker.gesture_type == GestureType::None {
            tracker.camera_start_pos = transform.translation;
            tracker.time_start_touch = time.elapsed_seconds();
        }
        tracker.gesture_type = GestureType::Pan;
        let time_since_start = time.elapsed_seconds() - tracker.time_start_touch;
        if time_since_start < config.touch_time_min {
            return;
        }
        let distance = Vec3::new(touches[0].distance().x, -touches[0].distance().y, 0.);
        transform.translation =
            tracker.camera_start_pos - config.drag_sensitivity * distance * projection.scale;
        tracker.last_touch_a = Some(touches[0].position());
        tracker.last_touch_b = None;
    }
}
