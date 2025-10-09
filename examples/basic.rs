// This example shows how to create a simple animated sprite.

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
