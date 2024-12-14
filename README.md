[![Crates.io](https://img.shields.io/crates/v/bevy_spritesheet_animation)](https://crates.io/crates/bevy_spritesheet_animation)
[![Docs](https://docs.rs/bevy_spritesheet_animation/badge.svg)](https://docs.rs/bevy_spritesheet_animation)
[![Build](https://github.com/merwaaan/bevy_spritesheet_animation/actions/workflows/build.yml/badge.svg)](https://github.com/merwaaan/bevy_spritesheet_animation/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

bevy_spritesheet_animation is a [Bevy](https://bevyengine.org/) plugin for easily animating 2D and 3D sprites.

![An animated character walking from the left to the right and shooting their gun](https://github.com/merwaaan/bevy_spritesheet_animation/raw/main/assets/example.gif)

# Features

- Animate 2D and [3D sprites](#3d-sprites)! 🎉
- A single Bevy [component](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/components/spritesheet_animation/struct.SpritesheetAnimation.html) to add to your entities to play animations.
- Tunable parameters: [duration](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDuration.html), [repetitions](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationRepeat.html), [direction](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDirection.html), [easing](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/easing/enum.Easing.html).
- [Composable animations](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/struct.Animation.html) from multiple clips.
- [Events](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/events/enum.AnimationEvent.html) to react to animations ending or reaching specific points.
- A [convenient API](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/spritesheet/struct.Spritesheet.html) to select frames in spritesheets.

> [!TIP]
> This crate is under active development. Please regularly check the [CHANGELOG](CHANGELOG.md) for recent changes.

# Quick start

1. Add the [SpritesheetAnimationPlugin](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/plugin/struct.SpritesheetAnimationPlugin.html) to your app
2. Use the [AnimationLibrary](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/library/struct.AnimationLibrary.html) resource to create new clips and animations
3. Add [SpritesheetAnimation](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/components/spritesheet_animation/struct.SpritesheetAnimation.html) components to your entities

```rust
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin to enable animations.
        // This makes the AnimationLibrary resource available to your systems.
        .add_plugins(SpritesheetAnimationPlugin::default())
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut library: ResMut<AnimationLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    // Create a clip

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.row(3))
        .with_duration(AnimationDuration::PerFrame(150));

    let clip_id = library.register_clip(clip);

    // Add this clip to an animation

    let animation = Animation::from_clip(clip_id)
        .with_repetitions(AnimationRepetition::Times(3));

    let animation_id = library.register_animation(animation);

    // This is a simple animation with a single clip but we can create more sophisticated
    // animations with multiple clips, each one having different parameters.
    //
    // See the `composition` example for more details.

    // Spawn a Bevy built-in Sprite

    let image = assets.load("character.png");

    let atlas = TextureAtlas {
        layout: atlas_layouts.add(spritesheet.atlas_layout(96, 96)),
        ..default()
    };

    commands.spawn((
        Sprite::from_atlas_image(image, atlas),

        // Add a SpritesheetAnimation component that references our newly created animation
        SpritesheetAnimation::from_id(animation_id),
    ));

    commands.spawn(Camera2d);
}
```

# Overview

## Clips

A clip is a sequence of frames.

It is the most basic building block for creating animations.
Simple animations may contain a single clip while more complex animations may contain a sequence of clips.

Parameters like [duration](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDuration.html), [repetitions](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationRepeat.html), [direction](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDirection.html) and [easing](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/easing/enum.Easing.html) can be specified.

Use the [AnimationLibrary](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/library/struct.AnimationLibrary.html) resource to register a new clip.

The clip can then be referenced in any number of animations.

```rust
fn create_animation(mut commands: Commands, mut library: ResMut<AnimationLibrary>) {

    // Create a clip that uses some frames from a spritesheet

    let spritesheet = Spritesheet::new(8, 8);

    let clip = Clip::from_frames(spritesheet.column(2))
        .with_duration(AnimationDuration::PerRepetition(1500))
        .with_repetitions(5);

    let clip_id = library.register_clip(clip);

    // Add this clip to an animation

    let animation = Animation::from_clip(clip_id);

    // ...
}
```

## Animations

In its simplest form, an animation is composed of a single clip that loops endlessly.

However, you may compose more sophisticated animations by chaining multiple clips and tuning their parameters separately.

Use the [AnimationLibrary](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/library/struct.AnimationLibrary.html) resource to register a new animation.

The animation can then be referenced in any number of SpritesheetAnimation component.

```rust
fn create_animation(mut commands: Commands, mut library: ResMut<AnimationLibrary>) {

    // ... omitted: create and register a few clips

    let animation = Animation::from_clips([
            running_clip_id,
            shooting_clip_id,
            running_clip_id
        ])
        .with_duration(AnimationDuration::PerRepetition(3000))
        .with_direction(Animation::Direction::Backwards);

    let animation_id = library.register_animation(animation);

    // Assign the animation to an entity with the SpritesheetAnimation component

    // ... omitted: load the sprite's image and create an atlas

    commands.spawn((
        Sprite::from_atlas_image(image, atlas),
        SpritesheetAnimation::from_id(animation_id),
    ));
}
```

## Think of clips and animations as assets!

Clips and animations should be created once.
You can then assign them to many entities.

### ❌ BAD

Do not create the same clip/animation for each entity that plays it.

```rust
fn spawn_characters(mut commands: Commands, mut library: ResMut<AnimationLibrary>) {

    // Creating identical animations gives more work to the plugin and degrades performance!

    for _ in 0..100 {
        let clip = Clip::from_frames([1, 2, 3]);
        let clip_id = library.register_clip(clip);

        let animation = Animation::from_clip(clip_id);
        let animation_id = library.register_animation();

        commands.spawn((
            Sprite::from_atlas_image(/* .... */),
            SpritesheetAnimation::from_id(animation_id),
        ));
    }
}
```

### 👍 GOOD

Instead, create clips/animations once and then reference them when needed.

For instance, you can create all your animations in a setup system, give them unique names and then assign them to entities, immediately or at a later stage.

```rust
fn spawn_characters(mut library: ResMut<AnimationLibrary>) {
    let clip = Clip::from_frames([1, 2, 3]);
    let clip_id = library.register_clip(clip);

    let animation = Animation::from_clip(clip_id);
    let animation_id = library.register_animation();

    // Here, we name the animation to make it easy to retrieve it in other systems.
    //
    // Alternatively, you may prefer to store the animation ID yourself.
    // For instance, in a Bevy Resource that contains the IDs of all your clips/animations.
    //
    // Something like:
    //
    // #[derive(Resource)]
    // struct GameAnimations {
    //     enemy_running: AnimationId,
    //     enemy_firing: AnimationId,
    //     ... and so on ...
    // }

    library.name_animation(animation_id, "enemy running");
}

fn spawn_enemies(mut commands: Commands, library: Res<AnimationLibrary>) {

    // Retrieve our animation and assign it to many entities

    if let Some(animation_id) = libray.animation_with_name("enemy running") {
        for _ in 0..100 {
            commands.spawn((
                Sprite::from_atlas_image(/* .... */),
                SpritesheetAnimation::from_id(animation_id),
            ));
        }
    }
}
```

## 3D sprites

![A dozen of 3D sprites moving in 3D space](https://github.com/merwaaan/bevy_spritesheet_animation/raw/main/assets/example3d.gif)

This crate also makes it easy to integrate 3D sprites into your games, which is not supported by Bevy out of the box.

Animating a 3D sprite is the same as animating 2D sprites: simply spawn a [Sprite3d](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/components/sprite3d/struct.Sprite3d.html) instead of Bevy's built-in Sprite and attach a [SpritesheetAnimation](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/components/spritesheet_animation/struct.SpritesheetAnimation.html) component to the entity.

```rust
fn spawn_character(mut commands: Commands, mut library: ResMut<AnimationLibrary>) {

    // ...

    let animation_id = library.register_animation(animation);

    commands.spawn((
        Sprite3dBuilder::from_image(texture.clone())
            .with_atlas(atlas_layout)
            .with_anchor(Anchor::BottomRight)
            .build(),
        SpritesheetAnimation::from_id(animation_id)
    ));
}
```

# More examples

For more examples, browse the [examples/](examples) directory.

| Example                                | Description                                                              |
| -------------------------------------- | ------------------------------------------------------------------------ |
| [basic](examples/basic.rs)             | Shows how to create an animated sprite                                   |
| [3d](examples/3d.rs)                   | Shows how to create 3D sprites                                           |
| [progress](examples/progress.rs)       | Shows how to control an animation                                        |
| [composition](examples/composition.rs) | Shows how to create an animation with multiple clips                     |
| [parameters](examples/parameters.rs)   | Shows the effect of each animation parameter                             |
| [character](examples/character.rs)     | Shows how to create a controllable character with multiple animations    |
| [events](examples/events.rs)           | Shows how to react to animations reaching points of interest with events |
| [headless](examples/headless.rs)       | Shows how to run animations in a headless Bevy app without rendering     |
| [stress](examples/stress.rs)           | Stress test with thousands of animated sprites (either 2D or 3D)         |

# Compatibility

| bevy | bevy_spritesheet_animation |
| ---- | -------------------------- |
| 0.15 | 2.0.0                      |
| 0.14 | 0.2.0                      |
| 0.13 | 0.1.0                      |

# Credits

- The character spritesheet used for the examples is CC0 from [thekingphoenix](https://opengameart.org/content/pixel-character-with-gun)
