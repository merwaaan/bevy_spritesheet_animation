// This example illustrates how to create a simple animated sprite.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::{components::sprite3d::Sprite3DBuilder, prelude::*};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, (setup, spawn_3d_sprite))
        .add_systems(Update, orbit)
        .run();
}

fn setup(mut commands: Commands, mut library: ResMut<SpritesheetLibrary>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    });

    // Create an animation

    let clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).row(3));
    });

    let animation_id = library.new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    library.name_animation(animation_id, "walk").unwrap();
}

fn spawn_3d_sprite(
    mut commands: Commands,
    library: Res<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    // Create an image and an atlas layout like you would for any Bevy sprite

    let texture = assets.load("character.png");

    let atlas_layout_handle = atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(96.0, 96.0),
        8,
        8,
        None,
        None,
    ));

    // Retrieve our animation from the library

    let animation_id = library.animation_with_name("walk");

    //

    // if let Some(id) = animation_id {
    //     commands.spawn((
    //         Sprite3DBuilder::from_image(texture.clone())
    //             .with_atlas(atlas_layout_handle)
    //             .build(),
    //         SpritesheetAnimation::from_id(id),
    //         Orbit { start_angle: 1.0 },
    //     ));
    // }

    commands.spawn((
        Sprite3DBuilder::from_image(texture.clone()).build(),
        Orbit { start_angle: 0.0 },
    ));
}

#[derive(Component)]
struct Orbit {
    start_angle: f32,
}

fn orbit(time: Res<Time>, mut query: Query<(&Orbit, &mut Transform)>) {
    for (orbit, mut transform) in &mut query {
        transform.translation.x = (orbit.start_angle + time.elapsed_seconds()).cos() * 300.0;
        transform.translation.z = (orbit.start_angle + time.elapsed_seconds()).sin() * 300.0;
    }
}
