use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_prototype_lyon::{prelude::*, shapes::Polygon};
use chess::Chess;

mod chess;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb_u8(0xEE, 0xDF, 0xDE)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                // fill the entire browser window
                fit_canvas_to_parent: true,
                ..default()
            },
            ..default()
        }))
        .add_plugin(ShapePlugin)
        .add_startup_system(setup)
        .add_plugin(Chess { number: 3 })
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}
