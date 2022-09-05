use bevy::prelude::*;

use crate::variables::binding::update_bindings;

use self::boundcircle::{update_bound_circles, BoundCircle};
use self::boundline::{update_bound_lines, BoundLine};
use self::boundpoint::BoundPoint;
use self::boundtracker::{update_bound_trackers, BoundTracker};

pub mod boundcircle;
pub mod boundline;
pub mod boundpoint;
pub mod boundtracker;

pub struct DrawingPlugin {
    pub num_pages: usize,
}

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        // app.insert_resource(Page {
        //     current_page: 1,
        //     num_pages: self.num_pages,
        // });
        app.add_startup_system(camera_setup);
        // app.add_system(camera_controls);
        // app.add_system(move_camera);

        app.add_system_set(
            SystemSet::new()
                .label("drawing")
                .after("variable_recalculation")
                .with_system(update_bindings::<BoundLine>.label("bind_lines"))
                .with_system(update_bound_lines.after("bind_lines"))
                .with_system(update_bindings::<BoundPoint>.label("bind_points"))
                .with_system(update_bindings::<BoundCircle>.label("bind_circles"))
                .with_system(
                    update_bound_circles
                        .after("bind_points")
                        .after("bind_circles"),
                )
                .with_system(update_bindings::<BoundTracker>.label("bind_trackers"))
                .with_system(update_bound_trackers.after("bind_trackers")),
        );
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
