// This example illustrates how to create a more sophisticated animation with multiple stages.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut library: ResMut<SpritesheetLibrary>,
    assets: Res<AssetServer>,
) {
    commands.spawn(Camera2dBundle::default());

    // Load assets for the sprite

    let texture = assets.load("character.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(96.0, 96.0),
        8,
        8,
        None,
        None,
    ));

    // Compose an animation from multiple clips
    //
    // - idle 3 times
    // - run 5 times
    // - shoot once
    //
    // The whole animation will repeat 10 times

    let idle_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).horizontal_strip(0, 0, 5))
            .set_default_duration(AnimationDuration::PerCycle(700))
            .set_default_repeat(3);
    });

    let run_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).row(3))
            .set_default_duration(AnimationDuration::PerCycle(600))
            .set_default_repeat(5);
    });

    let shoot_clip_id = library.new_clip(|clip| {
        clip.push_frame_indices(Spritesheet::new(8, 8).horizontal_strip(0, 5, 5))
            .set_default_duration(AnimationDuration::PerCycle(600))
            .set_default_repeat(1);
    });

    let anim_id = library.new_animation(|anim| {
        anim.add_stage(idle_clip_id.into())
            .add_stage(run_clip_id.into())
            .add_stage(shoot_clip_id.into())
            // Let's repeat it 10 times and then stop
            .set_repeat(AnimationRepeat::Cycles(10));
    });

    // Spawn a sprite that uses the animation

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout,
                ..default()
            },
            ..default()
        },
        SpritesheetAnimation::from_id(anim_id),
    ));
}
