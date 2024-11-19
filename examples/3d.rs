// This example shows how to create 3D sprites.

#[path = "./common/mod.rs"]
pub mod common;

use bevy::{prelude::*, sprite::Anchor};
use bevy_spritesheet_animation::prelude::*;
use rand::{seq::SliceRandom, Rng};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            SpritesheetAnimationPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_on_keypress, orbit, draw_gizmos))
        .run();
}

fn setup(
    mut commands: Commands,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    // 3D sprites require a 3D camera

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 1000.0, 4000.0).looking_at(Vec3::ZERO, Dir3::Y),
        ..default()
    });

    // Create an animation as usual

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.row(3));

    let clip_id = library.register_clip(clip);

    let animation = Animation::from_clip(clip_id);

    let animation_id = library.register_animation(animation);

    // Create an image and an atlas layout like you would for any Bevy sprite

    let texture = assets.load("character.png");

    let atlas_layout = atlas_layouts.add(spritesheet.atlas_layout(96, 96));

    // Spawn 3D sprites

    // A few 3D sprites orbiting around the center with various parameters

    let sprite_builders = [
        Sprite3dBuilder::from_image(texture.clone()),
        Sprite3dBuilder::from_image(texture.clone()).with_flip(true, false),
        Sprite3dBuilder::from_image(texture.clone()).with_flip(false, true),
        Sprite3dBuilder::from_image(texture.clone()).with_flip(true, true),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::BottomLeft),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::BottomCenter),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::BottomRight),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::CenterLeft),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::Center),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::CenterRight),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::TopLeft),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::TopCenter),
        Sprite3dBuilder::from_image(texture.clone()).with_anchor(Anchor::TopRight),
        Sprite3dBuilder::from_image(texture.clone()).with_custom_size(Vec2::new(100.0, 400.0)),
    ];

    for (i, builder) in sprite_builders.iter().enumerate() {
        commands.spawn((
            builder.clone().with_atlas(atlas_layout.clone()).build(),
            SpritesheetAnimation::from_id(animation_id),
            Orbit {
                start_angle: i as f32 * std::f32::consts::TAU / sprite_builders.len() as f32,
            },
        ));
    }

    // A non-animated 3D sprite in the center

    commands.spawn(
        Sprite3dBuilder::from_image(texture.clone())
            .with_atlas(atlas_layout.clone())
            .with_color(Color::linear_rgb(1.0, 0.0, 0.0))
            .build(),
    );

    // Help text

    commands.spawn(TextBundle::from_section(
        "C: random colors\nX: flip on X\nY: flip on Y\nA: random anchors\nS: random sizes\nR: reset",
        TextStyle {
            font_size: 30.0,
            ..default()
        },
    ));
}

fn update_on_keypress(keyboard: Res<ButtonInput<KeyCode>>, mut sprites: Query<&mut Sprite3d>) {
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

        if keyboard.just_pressed(KeyCode::KeyA) {
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
        transform.translation.x = (orbit.start_angle + time.elapsed_seconds()).cos() * 1500.0;
        transform.translation.z = (orbit.start_angle + time.elapsed_seconds()).sin() * 1500.0;
    }
}

fn draw_gizmos(mut gizmos: Gizmos, sprites: Query<&Transform, With<Sprite3d>>) {
    for &transform in &sprites {
        gizmos.axes(transform, 100.0);
    }
}
