# Changelog

## [0.1.3] - 2022-06-06
![0.1.3](./docs/dungeon_rogue_0_1_3.gif)

### Added
- Add Main Menu to pause and resume the game. For now it may only pause and resume the game. But every button just resume the game and now it's more fake.
- Add `iyes_loopless` crate to manupulate with the game state. For now player and enemies spawns only when user enter the game in the first time. And when the user in the menu all systems which are related with player or enemy are not running anymore. It should help to manupulate with systems extendability because the logic has been written for the whole plugin but not for the each system.


## [0.1.2] - 2022-06-02
![0.1.2](./docs/dungeon_rogue_0_1_2.gif)

### Added
- [debug] Debug layer now might be run via command `cargo run --features bevy/dynamic --features debug` which enables debug layer on the top of current ones and visualize all the collisions on the screen.
- Added an enemies on the screen based on Ldtk map position. Currently they are visualized just as boxes.

### Changed
- Updated `bevy_rapier2d` to [0.14](https://github.com/dimforge/bevy_rapier/pull/181) when the collider and rigid-body positions are read from the `GlobalTransform` instead of `Transform`.
After that all collisions with the floor, ladders, and enemies works correctly. 
I've also created an issue into the [bevy_ecs_ldtk](https://github.com/Trouv/bevy_ecs_ldtk/issues/89) when described the problem and @Trouv helped me with the explanation.
Hopefully, now everything works fine.

### Removed
- `sync_global_coords_with_local` system because `rapier` now works with `GlobalTransform`.

## [0.1.1]
### Fixed
- [debug] Update debug layer when dimensions are changed

## [0.1.0]
![0.1.0](./docs/dungeon_rogue_0_1_0.gif)

### Initial version which provides:
- Spawn the player and the map with collisions
- You may change player sprite by pressing `Q` (now only 2 options: Pumpkin and Dragon)
- You may travel through different levels in the same map
- Movement animation
- Debug layer to visualize Player, Ladder and Wall collisions

### What is not working:
#### Ladders isn't working
Ladders is not interactive when in the Ldtk settings set
```rust
LevelSpawnBehavior::UseWorldTranslation {
    load_level_neighbors: true,
}
```
by this config we load all nearby levels and change the position for the
levels from local to global.

#### Collisions with walls are made by kludges
I've created a `sync_global_coords_with_local` to syncronize global coords
and local coords after the collision has been set.
I don't know why it happened by I think the same problem occurs for the
Ladders