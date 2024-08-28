# Changelog

## 0.4.0 - 2024-??-??

This update simplifies the high-level API of the library.

A few structs have been renamed for clarity and consistency.

More importantly, creating animations is more straightforward.
You can now directly instantiate clips and animations, configure them and register them with the library, which gives them unique IDs.
The animation stages that acted like "clip instances" have been removed.
To create a variant of a clip, just clone and reconfigure it before registering the variant.

### Added

- Add with_xxx() methods to Clip and Animation to make it easier to set their parameters

### Changed

- Clip and Animation can be instantiated directly and must then be registered with the library
- Rename AnimationClip to Clip
- Rename SpritesheetLibrary to AnimationLibrary
- Rename "cycles" to "repetitions"
- Rename AnimationRepeat::Cycles to AnimationRepeat::Times
- Rename AnimationDuration::PerCycle to AnimationDuration::PerRepetition
- Rename structs related to 3D sprites with lowercase "3d" to better match Bevy

### Removed

- Remove AnimationStage

## 0.3.0 - 2024-08-26

### Added

- Add support for 3D sprites

## 0.2.0 - 2024-07-06

### Added

- Support Bevy 0.14.0
- Add a reset() method on the SpritesheetAnimation component

### Fixed

- Switch new_clip/new_animation closures to FnMut to allow mutations

## 0.1.0 - 2024-04-10

### Fixed

- Fix some direction combinations emitting events on the wrong frames

## 0.1.0-beta.1 - 2024-04-08

### Fixed

- Fix AnimationLibrary::name_clip/animation/marker returning an error when naming an item with a name it already has
- Fix MarketHit events reporting an incorrect stage index when an animation has empty clips
- Fix Easing::Out with the Cubic and Quartic modes generating incorrect values

## 0.1.0-beta.0 - 2024-04-07

Initial release
