[![Crates.io](https://img.shields.io/crates/v/bevy_spritesheet_animation)](https://crates.io/crates/bevy_spritesheet_animation)
[![Docs](https://docs.rs/bevy_spritesheet_animation/badge.svg)](https://docs.rs/bevy_spritesheet_animation)
[![Build](https://github.com/merwaaan/bevy_spritesheet_animation/actions/workflows/ci.yaml/badge.svg)](https://github.com/merwaaan/bevy_spritesheet_animation/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

bevy_spritesheet_animation is a [Bevy](https://bevyengine.org/) plugin for easily animating **2D** and **3D** sprites.

![An animated character walking from the left to the right and shooting their gun](https://github.com/merwaaan/bevy_spritesheet_animation/raw/main/assets/example.gif)

> [!TIP]
> This crate supports the latest [Bevy 0.17](https://bevyengine.org/news/bevy-0-17/). Please check the [compatibility table](#compatibility) to see which versions of this crate work with older Bevy versions.

> [!NOTE]
> This crate is under active development. Please regularly check the [CHANGELOG](CHANGELOG.md) for recent changes.

# Features

- Animate [2D sprites](#quick-start), [3D sprites](#3d-sprites), [UI images](#ui-images) and [cursors](#cursors)! 🎉
- [Easily build](crate::prelude::AnimationBuilder) animations from [spritesheets](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/spritesheet/struct.Spritesheet.html) with custom parameters like [duration](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDuration.html), [repetitions](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationRepeat.html), [direction](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDirection.html) and [easing](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/easing/enum.Easing.html).
- Trigger [events](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/events/enum.AnimationEvent.html) when animations end or reach specific points.

# Quick start

1. Add the [SpritesheetAnimationPlugin](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/plugin/struct.SpritesheetAnimationPlugin.html) to your app
2. Build animations with the [Spritesheet API](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/spritesheet/struct.Spritesheet.html)
3. Add [SpritesheetAnimation](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/components/spritesheet_animation/struct.SpritesheetAnimation.html) components to your entities

```rust
use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        // Add the plugin to enable animations
        //
        // This configures the app to play animations for entities with a SpritesheetAnimation component.
        // This also makes the Assets<Animation> resource available to your systems.
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, create_animated_sprite)
        .run();
}

fn create_animated_sprite(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Create an animation from a row of an 8x8 spritesheet
    //
    // This is a simple animation made of a single clip but we can create more sophisticated animations with multiple clips, each one having different parameters.
    //
    // See the `composition` example for more details.

    let image = assets.load("character.png");

    let spritesheet = Spritesheet::new(&image, 8, 8);

    let animation = spritesheet
        .create_animation()
        .add_row(3)
        .set_duration(AnimationDuration::PerFrame(100))
        .build();

    // Register the animation as an asset

    let animation_handle = animations.add(animation);

    // Create a regular Bevy sprite
    //
    // Here we use the spritesheet to automatically generate the animation-ready Bevy sprite.
    // This is optional and you may prefer to build the sprite manually.

    let sprite = spritesheet
        .with_size_hint(768, 768)
        .sprite(&mut atlas_layouts);

    // Spawn the sprite with a SpritesheetAnimation component that references our animation

    commands.spawn((
        // This is a regular Bevy sprite
        sprite,
        // This is the component that animates the sprite
        SpritesheetAnimation::new(animation_handle),
    ));
}

```

# Overview

## Animations

In its simplest form, an [Animation](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/struct.Animation.html) is a sequence of cells extracted from a spritesheet image.

Use a [Spritesheet](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/spritesheet/struct.Spritesheet.html) to create an [Animation](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/struct.Animation.html).

For each animation, you can control its [duration](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDuration.html), [repetitions](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationRepeat.html), [direction](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/animation/enum.AnimationDirection.html) and [easing](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/easing/enum.Easing.html).

```rust
// Here, we extract two animations from an 8x8 spritesheet image

let image = assets.load("character.png");

let spritesheet = Spritesheet::new(&image, 8, 8)

// Let's create a looping animation from the first row

let run_animation = spritesheet
    .create_animation()
    .add_row(0)
    .set_duration(AnimationDuration::PerRepetition(1500))
    .set_repetitions(AnimationRepeat::Loop)
    .build();

// Let's create another animation from another row, with different playback parameters

let shoot_animation = spritesheet
    .create_animation()
    .add_row(3)
    .set_duration(AnimationDuration::PerFrame(120))
    .set_easing(Easing::In(EasingVariety::Quadratic))
    .build();
```

## Clips

To create more sophisticated animations, you can leverage a lower-level construct: [clips](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/clip/struct.Clip.html).

Think of clips as sub-animations that are chained together and have their own parameters (duration, repetitions, etc...).

```rust
let animation = Spritesheet::new(&image, 8, 8)
    .create_animation()

    // An animation starts with an implicit clip that you can edit right away
    //
    // Parameters set with set_clip_xxx() only apply to the current clip
    .add_row(2)
    .set_clip_duration(AnimationDuration::PerRepetition(2500))

    // Call start_clip() to add another clip
    //
    // Here the second clip uses the same frames but will play faster and repeat a few times
    .start_clip()
    .add_row(2)
    .set_clip_duration(AnimationDuration::PerRepetition(1000))
    .set_clip_repetitions(5)

    // Parameters set with set_xxx() apply to the whole animation
    .set_direction(AnimationDirection::PingPong)

    .build();
```

## Animations are assets

Just like Bevy's images, materials or meshes, this crate's animations are assets.

After creating an animation, register it in `Assets<Animation>`.

This will give you a `Handle<Animation>` that you can assign to `SpritesheetAnimation` components.

```rust
fn create_animated_sprite(
    mut commands: Commands,
    mut animations: ResMut<Assets<Animation>>,
) {
    // ... omitted: create an animation

    let animation_handle = animations.add(animation);

    commands.spawn((
        // ... omitted: your entity's other components

        SpritesheetAnimation::new(animation_handle),
    ));
}
```

> [!WARNING]
> An animation should be created only once and then assigned to as many sprites as you need.
> Creating identical animations gives more work to the plugin and may degrade performance.

## Interaction with Bevy's built-in components

To animate an entity, you just have to give it:

- a regular Sprite component (provided by Bevy)
- a [SpritesheetAnimation](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/components/spritesheet_animation/struct.SpritesheetAnimation.html) component (provided by this crate)

This crate provides a few helpers to make the creation of such entities more concise.

In the simplest case, you can use `sprite()` to get an animation-ready sprite with the correct image and texture atlas:

```rust
commands.spawn((
    spritesheet.with_size_hint(600, 400).sprite(&mut atlas_layouts),
    SpritesheetAnimation::new(animation_handle),
));
```

If you need more control, for instance to set some sprite attributes like its color, you might prefer to construct the sprite yourself.

In that case, you can spell out the entity creation and use `Spritesheet::atlas()` to retrieve the texture atlas that matches your spritesheet.

```rust
commands.spawn((
    Sprite {
        image: your_image,
        texture_atlas: spritesheet.atlas(&mut atlas_layouts),
        color: LinearRGBA::RED,
        ..default()
    },
    SpritesheetAnimation::new(animation_handle),
));
```

## Accessing your animations across systems

If you need to access your animations from different systems, store their handles in custom resources.

```rust
#[derive(Resource)]
struct MyAnimations {
    idle: Handle<Animation>,
    shoot: Handle<Animation>,
}

fn spawn_animated_character(
    mut commands: Commands,
    mut animations: ResMut<Assets<Animation>>
) {
    // ... omitted: create the animations

    let idle_animation_handle = animations.add(idle_animation);
    let shoot_animation_handle = animations.add(shoot_animation);

    // Spawn the character

    commands.spawn((
        sprite,
        SpritesheetAnimation::new(idle_animation_handle.clone()))
    );

    // Store the animations as a resource

    commands.insert_resource(MyAnimations {
        idle: idle_animation_handle,
        shoot: shoot_animation_handle,
    });
}

fn switch_animation(
    keyboard: Res<ButtonInput<KeyCode>>,
    character: Single<&mut SpritesheetAnimation>,
    my_animations: Res<MyAnimations>,
) {
    let mut animation = character.into_inner();

    if keyboard.pressed(KeyCode::KeyA) {
        animation.switch(my_animations.shoot.clone());
    }
    else if keyboard.pressed(KeyCode::KeyB) {
        animation.switch(my_animations.idle.clone());
    }
}
```

This method is showcased in the [character](examples/character.rs) and [events](examples/events.rs) examples.

## 3D sprites

![A dozen of 3D sprites moving in 3D space](https://github.com/merwaaan/bevy_spritesheet_animation/raw/main/assets/example_3d.gif)

This crate also makes it easy to integrate 3D sprites into your games, which is not supported by Bevy out of the box.

Animating a 3D sprite is the same as animating 2D sprites: simply spawn a [Sprite3d](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/components/sprite3d/struct.Sprite3d.html) instead of Bevy's built-in Sprite and attach a [SpritesheetAnimation](https://docs.rs/bevy_spritesheet_animation/latest/bevy_spritesheet_animation/components/spritesheet_animation/struct.SpritesheetAnimation.html) component to the entity.

Like for 2D sprites, Spritesheet provides a helper that creates an animation-ready 3D sprite:

```rust
let sprite3d = spritesheet
    .with_size_hint(600, 400)
    .sprite3d(&mut atlas_layouts)
    .with_color(LinearRgba::RED)
    .with_flip(false, true);

commands.spawn((
    sprite3d,
    SpritesheetAnimation::new(animation_handle),
));
```

> [!TIP]
> Static 3D sprites can also be spawned by omitting the `SpritesheetAnimation` component.

## UI images

![Three animated hearts filling with colors to represent health](https://github.com/merwaaan/bevy_spritesheet_animation/raw/main/assets/example_ui.gif)

This crate also animates UI images with the same workflow.

Please check out the [complete example](examples/ui.rs).

```rust
fn create_animated_ui_image(
    mut commands: Commands,
    assets: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    // ... omitted: create a spritesheet and an animation

    let image_node = spritesheet
        .with_size_hint(300, 300)
        .expect("the image is not loaded")
        .image_node(&mut atlas_layouts);

    commands.spawn((
        image_node,
        SpritesheetAnimation::new(animation),
    ));
}
```

## Cursors

![An animated cursor clicking randomly](https://github.com/merwaaan/bevy_spritesheet_animation/raw/main/assets/example_cursor.gif)

This crate also animates cursors with the same workflow.

Please check out the [complete example](examples/cursor.rs).

```rust
fn create_animated_cursor(
    window: Single<Entity, With<Window>>,
    mut commands: Commands,
    assets: Res<AssetServer>,
    images: Res<Assets<Image>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>
) {
    // ... omitted: create a spritesheet and an animation

    let cursor_icon = spritesheet
        .with_size_hint(500, 100)
        .expect("the image is not loaded")
        .cursor_icon(&mut atlas_layouts);

    commands.entity(*window).insert((
        cursor_icon,
        SpritesheetAnimation::new(animation),
    ));
}
```

# More examples

For more examples, browse the [examples/](examples) directory.

| Example                                | Description                                                           |
| -------------------------------------- | --------------------------------------------------------------------- |
| [basic](examples/basic.rs)             | Shows how to create a simple animated sprite                          |
| [composition](examples/composition.rs) | Shows how to create an animation with multiple clips                  |
| [progress](examples/progress.rs)       | Shows how to query/control the progress of an animation               |
| [parameters](examples/parameters.rs)   | Shows the effect of each animation parameter                          |
| [character](examples/character.rs)     | Shows how to create a controllable character with multiple animations |
| [events](examples/events.rs)           | Shows how to handle animation events                                  |
| [3d](examples/3d.rs)                   | Shows how to create 3D sprites                                        |
| [cursor](examples/cursor.rs)           | Shows how to create an animated cursor                                |
| [ui](examples/ui.rs)                   | Shows how to create an animated UI image                              |
| [headless](examples/headless.rs)       | Shows how to run animations in a headless Bevy app without rendering  |
| [stress](examples/stress.rs)           | Stress test with thousands of animated sprites (either 2D or 3D)      |

# Compatibility

| bevy | bevy_spritesheet_animation |
| ---- | -------------------------- |
| 0.17 | 4.0.0                      |
| 0.16 | 3.0.0                      |
| 0.15 | 2.0.0                      |
| 0.14 | 0.2.0                      |
| 0.13 | 0.1.0                      |

# Credits

- Assets used in the examples:
  - Character spritesheet by [thekingphoenix](https://opengameart.org/content/pixel-character-with-gun)
  - Cursor spritesheet by [crusenho](https://crusenho.itch.io/complete-ui-book-styles-pack)
  - Hearts spritesheet by [ELV Games](https://elvgames.itch.io/free-inventory-asset-pack)
