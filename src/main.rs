use bevy::prelude::*;
use bevy::{asset::AssetServerSettings, prelude::Component};
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::{prelude::*, shapes::Circle};
use drawing::DrawingPlugin;
use page1::Page1Plugin;
use page2::Page2Plugin;
use page3::Page3Plugin;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use variables::debug::DebugPlugin;
use variables::variable::Variable;
use variables::VariablePlugin;

mod drawing;
mod page1;
mod page2;
mod page3;
mod page4;
mod variables;

const GLOBAL: usize = 0;

#[derive(Debug, Clone, Eq, PartialEq, Hash, EnumIter, Copy, Component)]
pub(crate) enum Page {
    Simple,
    Combination,
    Game,
    Unnamed,
}

#[derive(Component)]
pub(crate) struct EquationText {
    variables: Vec<Entity>,
    template: &'static str,
}

#[derive(Component)]
pub struct Time;

fn main() {
    App::new()
        .insert_resource(AssetServerSettings {
            watch_for_changes: true,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_plugin(EguiPlugin)
        .add_plugin(VariablePlugin)
        .add_plugin(DrawingPlugin { num_pages: 4 })
        .add_plugin(DebugPlugin {
            variables: false,
            bindings: false,
        })
        .add_state(Page::Simple)
        .add_plugin(Page1Plugin)
        .add_plugin(Page2Plugin)
        .add_plugin(Page3Plugin)
        .add_system(time_update)
        .add_system(page_system)
        .add_system(page_enter)
        .run();
}

fn page_system(mut page: ResMut<State<Page>>, input: Res<Input<KeyCode>>) {
    let mut movement = 0;
    if input.just_pressed(KeyCode::D) {
        movement += 1;
    }
    if input.just_pressed(KeyCode::A) {
        movement -= 1;
    }
    if movement != 0 {
        let pages: Vec<Page> = Page::iter().collect();
        let mut current_index = pages.iter().position(|w| *w == *page.current()).unwrap() as isize;
        current_index = (current_index + movement)
            .max(0)
            .min((pages.len() - 1) as isize);
        page.set(*pages.get(current_index as usize).unwrap());
    }
}

fn page_enter(mut page_query: Query<(&Page, &mut Visibility)>, current_page: Res<State<Page>>) {
    if current_page.is_changed() {
        for (page, mut visibility) in page_query.iter_mut() {
            visibility.is_visible = page == current_page.current();
        }
    }
}

fn time_update(mut time_query: Query<&mut Variable, With<Time>>) {
    let delta = 0.02;
    for mut var in time_query.iter_mut() {
        let old_value = (*var).value();
        var.set_value(old_value + delta);
    }
}
