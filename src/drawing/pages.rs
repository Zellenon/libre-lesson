use bevy::prelude::*;

const WINDOW_WIDTH: f32 = 1280.0;

#[derive(Component)]
pub struct Page {
    pub current_page: isize,
    pub num_pages: usize,
}

#[derive(Component)]
pub(crate) struct PageCamera;

#[derive(Debug, Default, Component)]
pub(crate) struct CameraLocation(f32);

pub(crate) fn camera_setup(mut commands: Commands) {
    commands
        .spawn_bundle(Camera2dBundle::default())
        .insert(PageCamera)
        .insert(CameraLocation::default());
}

pub(crate) fn camera_controls(
    mut camera_query: Query<&mut CameraLocation, With<PageCamera>>,
    mut page: ResMut<Page>,
    input: Res<Input<KeyCode>>,
) {
    let mut movement = 0;
    if input.just_pressed(KeyCode::D) {
        movement += 1;
    }
    if input.just_pressed(KeyCode::A) {
        movement -= 1;
    }
    if movement != 0 {
        page.current_page += movement;
        page.current_page = page.current_page.max(1).min(page.num_pages as isize);

        for mut target in camera_query.iter_mut() {
            target.0 = page.current_page as f32 * WINDOW_WIDTH;
        }
    }
}

pub(crate) fn move_camera(
    mut query: Query<(&mut Transform, &CameraLocation), With<PageCamera>>,
    time: Res<Time>,
) {
    for (mut camera_transform, target) in query.iter_mut() {
        let camera_x = &mut camera_transform.translation.x;

        let remaining_distance = target.0 - *camera_x;
        let displacement = 5.0 * time.delta_seconds() * remaining_distance;
        *camera_x = if remaining_distance.abs() < 0.1 {
            target.0
        } else {
            *camera_x + displacement
        };
    }
}
