// This example shows how to animate UI images.

use bevy::prelude::*;
use bevy_spritesheet_animation::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(SpritesheetAnimationPlugin)
        .add_systems(Startup, create_ui)
        .run();
}

fn create_ui(
    mut commands: Commands,
    assets: Res<AssetServer>,
    mut animations: ResMut<Assets<Animation>>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    commands.spawn(Camera2d);

    // Create some animations

    let image = assets.load("hearts.png");

    let spritesheet = Spritesheet::new(&image, 5, 3);

    let animations = (0..3).map(|i| {
        let animation = spritesheet
            .create_animation()
            .add_row(i)
            .set_duration(AnimationDuration::PerRepetition(1000))
            .set_direction(AnimationDirection::PingPong)
            .build();

        animations.add(animation)
    });

    // Create a few UI images using those animations

    commands
        .spawn(Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Row,
            ..default()
        })
        .with_children(|parent| {
            animations.for_each(|animation| {
                parent
                    .spawn(Node {
                        flex_grow: 1.0,
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    })
                    .with_children(|parent| {
                        let image_node = spritesheet
                            .with_size_hint(80, 48)
                            .image_node(&mut atlas_layouts);

                        parent.spawn((
                            image_node,
                            SpritesheetAnimation::new(animation),
                            UiTransform::from_scale(Vec2::splat(10.0)),
                        ));
                    });
            });
        });
}
