// This example shows how to create 3D sprites.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::{prelude::*, sprite::Anchor};
use bevy_spritesheet_animation::prelude::*;
use rand::{Rng, seq::IndexedRandom as _};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin,
        ))
        .add_systems(Startup, spawn_sprites)
        .add_systems(Update, (update_on_keypress, orbit, draw_gizmos))
        .run();
}

fn spawn_sprites(
    mut commands: Commands,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    // 3D sprites require a 3D camera

    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 1000.0, 4000.0).looking_at(Vec3::ZERO, Dir3::Y),
    ));

    // Create an animation as usual

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.row(3));

    let clip_id = library.register_clip(clip);

    let animation = Animation::from_clip(clip_id);

    let animation_id = library.register_animation(animation);

    // Create an image and a texture atlas like you would for any Bevy sprite

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
        ..default()
    };

    // Spawn 3D sprites

    // A few 3D sprites orbiting around the center with various parameters

    let sprites = [
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_flip(true, false),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_flip(false, true),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_flip(true, true),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::BOTTOM_LEFT),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::BOTTOM_CENTER),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::BOTTOM_RIGHT),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::CENTER_LEFT),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::CENTER),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::CENTER_RIGHT),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::TOP_LEFT),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::TOP_CENTER),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone()).with_anchor(Anchor::TOP_RIGHT),
        Sprite3d::from_atlas_image(image.clone(), atlas.clone())
            .with_custom_size(Vec2::new(100.0, 400.0)),
    ];

    let sprite_count = sprites.len();

    for (i, sprite) in sprites.into_iter().enumerate() {
        commands.spawn((
            sprite,
            SpritesheetAnimation::from_id(animation_id),
            Orbit {
                start_angle: i as f32 * std::f32::consts::TAU / sprite_count as f32,
            },
        ));
    }

    // A non-animated 3D sprite in the center

    commands.spawn(
        Sprite3d::from_atlas_image(image.clone(), atlas.clone())
            .with_color(Color::linear_rgb(1.0, 0.0, 0.0)),
    );

    // Help text

    commands.spawn((Text(
        "C: random colors\nX: flip on X\nY: flip on Y\nA: random anchors\nS: random sizes\nR: reset".to_owned()),
        TextFont::from_font_size(30.0)
    ));
}

fn update_on_keypress(keyboard: Res<ButtonInput<KeyCode>>, mut sprites: Query<&mut Sprite3d>) {
    let mut rng = rand::rng();

    for mut sprite in &mut sprites {
        // Random color

        if keyboard.just_pressed(KeyCode::KeyC) {
            sprite.color = Color::linear_rgb(rng.random(), rng.random(), rng.random());
        }

        // Flip

        if keyboard.just_pressed(KeyCode::KeyX) {
            sprite.flip_x = !sprite.flip_x;
        }

        if keyboard.just_pressed(KeyCode::KeyY) {
            sprite.flip_y = !sprite.flip_y;
        }

        // Random anchors

        if keyboard.just_pressed(KeyCode::KeyA) {
            static ANCHORS: [Anchor; 9] = [
                Anchor::BOTTOM_LEFT,
                Anchor::BOTTOM_CENTER,
                Anchor::BOTTOM_RIGHT,
                Anchor::CENTER_LEFT,
                Anchor::CENTER,
                Anchor::CENTER_RIGHT,
                Anchor::TOP_LEFT,
                Anchor::TOP_CENTER,
                Anchor::TOP_RIGHT,
            ];

            sprite.anchor = *ANCHORS.choose(&mut rng).unwrap();
        }

        // Random size

        if keyboard.just_pressed(KeyCode::KeyS) {
            sprite.custom_size = Some(Vec2::new(
                rng.random_range(100.0..1000.0),
                rng.random_range(100.0..1000.0),
            ));
        }

        // Reset

        if keyboard.just_pressed(KeyCode::KeyR) {
            sprite.color = Color::WHITE;
            sprite.flip_x = false;
            sprite.flip_y = false;
            sprite.custom_size = None;
            sprite.anchor = Anchor::default();
        }
    }
}

#[derive(Component)]
struct Orbit {
    start_angle: f32,
}

fn orbit(time: Res<Time>, mut query: Query<(&Orbit, &mut Transform)>) {
    for (orbit, mut transform) in &mut query {
        transform.translation.x = (orbit.start_angle + time.elapsed_secs()).cos() * 1500.0;
        transform.translation.z = (orbit.start_angle + time.elapsed_secs()).sin() * 1500.0;
    }
}

fn draw_gizmos(mut gizmos: Gizmos, sprites: Query<&Transform, With<Sprite3d>>) {
    for &transform in &sprites {
        gizmos.axes(transform, 100.0);
    }
}
