[![Crates.io](https://img.shields.io/crates/v/bevy_spritesheet_animation)](https://crates.io/crates/bevy_spritesheet_animation)
[![Docs](https://docs.rs/bevy_spritesheet_animation/badge.svg)](https://docs.rs/bevy_spritesheet_animation)
[![Build](https://github.com/merwaaan/bevy_spritesheet_animation/actions/workflows/build.yml/badge.svg)](https://github.com/merwaaan/bevy_spritesheet_animation/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

bevy_spritesheet_animation is a [Bevy](https://bevyengine.org/) plugin for animating sprites that are backed by spritesheets.

![An animated character walking from the left to the right and shooting their gun](https://github.com/merwaaan/bevy_spritesheet_animation/raw/main/example.gif)

# Features

- A single Bevy [component](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/component/struct.SpritesheetAnimation.html) to add to your entities to play animations.
- Tunable parameters: [duration](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDuration.html), [repetitions](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationRepeat.html), [direction](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDirection.html), [easing](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/easing/enum.Easing.html).
- [Composable animations](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/struct.Animation.html) from multiple clips.
- [Events](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/events/enum.AnimationEvent.html) to react to animations ending or reaching specific points.
- A [convenient API](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/spritesheet/struct.Spritesheet.html) to select frames in spritesheets.

# Quick start

1. Add the [SpritesheetAnimationPlugin](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/plugin/struct.SpritesheetAnimationPlugin.html) to your app
2. Use the [SpritesheetLibrary](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/library/struct.SpritesheetLibrary.html) resource to create new clips and animations
3. Add a [SpritesheetAnimation](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/component/struct.SpritesheetAnimation.html) component to your entity

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Add the plugin to enable animations.
        // This makes the SpritesheetLibrary resource available to your systems.
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut library: ResMut<SpritesheetLibrary>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    assets: Res<AssetServer>,
) {
    // Create an animation

    let clip_id = library.new_clip(|clip| {
        // You can configure this clip here (duration, number of repetitions, etc...)

        // This clip will use all the frames in row 3 of the spritesheet
        clip.push_frame_indices(Spritesheet::new(8, 8).row(3));
    });

    let animation_id = library.new_animation(|animation| {
        // You can configure this animation here (duration, number of repetitions, etc...)

        animation.add_stage(clip_id.into());

        // This is a simple animation with a single clip but we can create more sophisticated
        // animations with multiple clips, each one having different parameters.
        //
        // See the `composition` example for more details.
    });

    // Spawn a sprite using Bevy's built-in SpriteSheetBundle

    let texture = assets.load("character.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        Vec2::new(96.0, 96.0),
        8,
        8,
        None,
        None,
    ));

    commands.spawn((
        SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout,
                ..default()
            },
            ..default()
        },
        // Add a SpritesheetAnimation component that references our newly created animation
        SpritesheetAnimation::from_id(animation_id),
    ));

    commands.spawn(Camera2dBundle::default());
}
```

# Overview

## Animation clips

An animation clip is a reusable sequence of frames.

It is the most basic building block for creating animations.

Use the [SpritesheetLibrary](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/library/struct.SpritesheetLibrary.html) resource to create and configure a new clip.

The clip can then be referenced in any number of animations.

```rust
fn setup(mut commands: Commands, mut library: ResMut<SpritesheetLibrary>) {

    // Create a clip that uses some frames from a spritesheet

    let clip_id = library.new_clip(|clip| {
        clip
            .push_frame_indices(Spritesheet::new(8, 8).column(2))
            .set_default_duration(AnimationDuration::PerCycle(1500))
            .set_default_repeat(5);
    });

    // Add this clip to an animation

    let animation_id = library.new_animation(|animation| {
        animation.add_stage(clip_id.into());
    });

    // ... Assign the animation to an entity with the SpritesheetAnimation component ...
}
```

## Animations

In its simplest form, an animation is composed of a single clip that loops endlessly.

However, you're free to compose more sophisticated animations by chaining multiple clips and by tuning the animation parameters.

Use the SpritesheetLibrary resource to create a new animation.

The animation can then be referenced in any number of SpritesheetAnimation component.

```rust
fn setup(mut commands: Commands, mut library: ResMut<SpritesheetLibrary>) {

    // ...

    let animation_id = library.new_animation(|animation| {
        let mut stage1 = AnimationStage::from_clip(some_clip_id);
        stage1
            .set_repeat(5)
            .set_easing(Easing::InOut(EasingVariety::Quadratic));

        let mut stage2 = AnimationStage::from_clip(another_clip_id);
        stage2
            .set_duration(AnimationDuration::PerFrame(120))
            .set_direction(Animation::Direction::Backwards);

        animation
            .add_stage(stage1)
            .add_stage(stage2)
            .set_duration(AnimationDuration::PerCycle(1000))
            .set_direction(Animation::Direction::PingPong);
    });

    // ... Assign the animation to an entity with the SpritesheetAnimation component ...
}
```

## Think of clips and animations as assets!

Clips and animations should be created once.
You can then assign them to many entities.

### ❌ BAD

You should not create the same clip/animation for each entity that plays it.

```rust
fn spawn_enemies(mut commands: Commands, mut library: ResMut<SpritesheetLibrary>) {

    // Creating identical animations gives more work to the plugin and degrades performance!

    for _ in 0..100 {
        let clip_id = library.new_clip(|clip| { /* ... */ });

        let animation_id = library.new_animation(|animation| { /* ... */ });

        commands.spawn((
            SpriteSheetBundle { /* .... */ },
            SpritesheetAnimation::from_id(animation_id),
        ));
    }
}
```

### 👍 GOOD

Instead, create clips/animations once and then reference them when needed.

For instance, you can create all your animations in a setup system, give them unique names and then assign them to entities at a later stage.

```rust
fn create_animation(mut library: ResMut<SpritesheetLibrary>) {

    let clip_id = library.new_clip(|clip| { /* ... */ });

    let animation_id = library.new_animation(|animation| { /* ... */ });

    // Here, we name the animation to make it easy to retrieve it in other systems.
    //
    // Alternatively, you may prefer to store the animation ID yourself.
    // For instance, in a Bevy Resource that contains the IDs of all your clips/animations.
    // Something like:
    //
    // #[derive(Resource)]
    // struct GameAnimations {
    //     enemy_running: Option<AnimationId>,
    //     enemy_firing: Option<AnimationId>,
    //     ... and so on ...
    // }

    library.name_animation(animation_id, "enemy running");
}

fn spawn_enemies(mut commands: Commands, library: Res<SpritesheetLibrary>) {

    // Retrieve our animation and assign it to many entities

    if let Some(animation_id) = libray.animation_with_name("enemy running") {
        for _ in 0..100 {
            commands.spawn((
                SpriteSheetBundle { /* .... */ },
                SpritesheetAnimation::from_id(animation_id),
            ));
        }
    }
}
```

# More examples

For more examples, browse the [examples/](examples) directory.

| Example                                | Description                                                              |
| -------------------------------------- | ------------------------------------------------------------------------ |
| [basic](examples/basic.rs)             | Minimal example showing how to create an animated sprite                 |
| [composition](examples/composition.rs) | Advanced example showing how to create an animation with multiple stages |
| [parameters](examples/parameters.rs)   | Shows the effect of each parameter                                       |
| [character](examples/character.rs)     | Shows how to create a controllable character with multiple animations    |
| [events](examples/events.rs)           | Shows how to react to animations reaching points of interest with events |
| [stress](examples/stress.rs)           | A stress test with thousands of animated sprites                         |

# Compatibility

| bevy | bevy_spritesheet_animation |
| ---- | -------------------------- |
| 0.13 | 0.1.0                      |

# Credits

- The character spritesheet used for the examples is CC0 from [thekingphoenix](https://opengameart.org/content/pixel-character-with-gun)
