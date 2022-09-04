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

pub struct DrawingPlugin;

impl Plugin for DrawingPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(drawing_setup);

        app.add_system(update_bindings::<BoundLine>.before(update_bound_lines));
        app.add_system(update_bound_lines);
        app.add_system(update_bindings::<BoundPoint>.before(update_bound_circles));
        app.add_system(update_bindings::<BoundCircle>.before(update_bound_circles));
        app.add_system(update_bound_circles);
        app.add_system(update_bindings::<BoundTracker>.before(update_bound_trackers));
        app.add_system(update_bound_trackers);
    }
}

fn drawing_setup(mut commands: Commands) {
    commands.spawn_bundle(Camera2dBundle::default());
}
