// This example illustrates how to create a simple animated sprite.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::{prelude::*, sprite::Anchor};
use bevy_spritesheet_animation::{
    components::sprite3d::{Sprite3D, Sprite3DBuilder},
    prelude::*,
};
use rand::{seq::SliceRandom, Rng};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, (setup, spawn_3d_sprite))
        .add_systems(Update, (update_on_keypress, orbit, draw_axes))
        .run();
}

fn setup(mut commands: Commands, mut library: ResMut<SpritesheetLibrary>) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 4000.0),
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
        UVec2::new(96, 96),
        8,
        8,
        None,
        None,
    ));

    // Retrieve our animation from the library

    let animation_id = library.animation_with_name("walk");

    // Spawn 3D sprites

    if let Some(animation_id) = animation_id {
        let sprite_builders = [
            Sprite3DBuilder::from_image(texture.clone()),
            Sprite3DBuilder::from_image(texture.clone())
                .with_color(Color::linear_rgb(1.0, 0.0, 0.0)),
            Sprite3DBuilder::from_image(texture.clone()).with_flip(true, false),
            Sprite3DBuilder::from_image(texture.clone()).with_flip(false, true),
            Sprite3DBuilder::from_image(texture.clone()).with_flip(true, true),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::BottomLeft),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::BottomCenter),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::BottomRight),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::CenterLeft),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::Center),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::CenterRight),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::TopLeft),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::TopCenter),
            Sprite3DBuilder::from_image(texture.clone()).with_anchor(Anchor::TopRight),
            Sprite3DBuilder::from_image(texture.clone()).with_custom_size(Vec2::new(100.0, 400.0)),
            // TODO rect
        ];

        for (i, builder) in sprite_builders.iter().enumerate() {
            commands.spawn((
                builder
                    .clone()
                    .with_atlas(atlas_layout_handle.clone())
                    .build(),
                SpritesheetAnimation::from_id(animation_id),
                Orbit {
                    start_angle: i as f32 * std::f32::consts::TAU / sprite_builders.len() as f32,
                },
            ));
        }
    }
}

fn update_on_keypress(keyboard: Res<ButtonInput<KeyCode>>, mut sprites: Query<&mut Sprite3D>) {
    let mut rng = rand::thread_rng();

    for mut sprite in &mut sprites {
        // Random color

        if keyboard.just_pressed(KeyCode::KeyC) {
            sprite.color = Color::linear_rgb(rng.gen(), rng.gen(), rng.gen());
        }

        // Flip

        if keyboard.just_pressed(KeyCode::KeyX) {
            sprite.flip_x = !sprite.flip_x;
        }

        if keyboard.just_pressed(KeyCode::KeyY) {
            sprite.flip_y = !sprite.flip_y;
        }

        // Random anchors

        if keyboard.just_pressed(KeyCode::KeyN) {
            static ANCHORS: [Anchor; 9] = [
                Anchor::BottomLeft,
                Anchor::BottomCenter,
                Anchor::BottomRight,
                Anchor::CenterLeft,
                Anchor::Center,
                Anchor::CenterRight,
                Anchor::TopLeft,
                Anchor::TopCenter,
                Anchor::TopRight,
            ];

            sprite.anchor = ANCHORS.choose(&mut rng).unwrap().clone();
        }

        // Random size

        if keyboard.just_pressed(KeyCode::KeyS) {
            sprite.custom_size = Some(Vec2::new(
                rng.gen_range(100.0..1000.0),
                rng.gen_range(100.0..1000.0),
            ));
        }

        // TODO Random rect

        // Reset

        if keyboard.just_pressed(KeyCode::KeyR) {
            sprite.color = Color::WHITE;
            sprite.flip_x = false;
            sprite.flip_y = false;
            sprite.custom_size = None;
            // TODO rect
            sprite.anchor = Anchor::default();
        }
    }
}

fn draw_axes(mut gizmos: Gizmos, sprites: Query<&Transform, With<Sprite3D>>) {
    for &transform in &sprites {
        gizmos.axes(transform, 100.0);
    }
}

#[derive(Component)]
struct Orbit {
    start_angle: f32,
}

fn orbit(time: Res<Time>, mut query: Query<(&Orbit, &mut Transform)>) {
    for (orbit, mut transform) in &mut query {
        transform.translation.x = (orbit.start_angle + time.elapsed_seconds()).cos() * 1500.0;
        transform.translation.z = (orbit.start_angle + time.elapsed_seconds()).sin() * 1500.0;
    }
}
