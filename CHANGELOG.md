# Changelog

## [0.1.8] - 2022-08-07
### Added
- Add damage to the enemy when the player interacts with them.
- Add an ability to change settings. For now, it's only
music
  - Ability to turn on / off music
  - Ability to increase / decrease volume
  - Ability to go back to the main menu

### Changed
- Move `MainMenuUI` and `DeadMenuUI` into `UI` plugin and add only this plugin in `main.rs`. We may add
additional UI plugins later and it will not affect main file.

## [0.1.7] - 2022-07-24
| Out-of bounce dead | Enemy damage |
| ------------------ | ------------ |
| ![0.1.7_1](./docs/dungeon_rogue_0_1_7_1.gif) | ![0.1.7_2](./docs/dungeon_rogue_0_1_7_2.gif) |

### Added
- Add background game music when the player is playing the game. And paused it when the player is in menu.
- Add player health and add HuD (Heads-up Display)
- Add `Health` for Player and all enemies (currently all enemies has the same amount of health but it will be 
  changed in the future).
- Update `Health` component when the player clicked `H` keyboard (just for testing). There is no relation to the 
  game. Just to check how to update `Health` component and HuD width. 
- Show dead menu when the player is dead. Forbid to resume the game if the player is dead
- Add out-of bounce area when the player should die because it's a restricted area.

### Changed
- Add more complex game state. Add additional game state for `Game` and `Menu` states. Now `Menu` has `Main` 
  and`Dead` sub states.

## [0.1.6] - 2022-07-12
![0.1.6](./docs/dungeon_rogue_0_1_6.gif)

### Added
- Add enemy sprites and make them look like a monsters
- Add `patrol` for the monster to be able to walk on the map
- Add enemy animation if they are on move (the same logic as we did for player)

### Changed
- Do not affect tutorials if there is any interactions except the Player. Because previously any kind of entity may 
  affect tutorials and even the monsters removes tutorials after collision interactions.
- Change enemy sprite rotation when the monster goes on left or right

## [0.1.5] - 2022-06-21

### Added
- Add movement tutorial. When we show a UI on the top-left screen. And we're leading the user of how to move and climb

### Faced bug
User couldn't jump on new levels. But ground detection
works fine for current levels. Maybe the problem inside
how the levels has been created or how ground detection
checks new loaded levels.

I'll try to play with it and fix it in the next release.

## [0.1.4] - 2022-06-10

### Added
- Add an ability to use main menu. When the user open the menu they may interact with `Play` and `Exit` buttons to play or exit the game respectively.

### Removed
- Remove an ability to jump in the air. Now the player may jump only once until it faced the surface.

---

## [0.1.3] - 2022-06-06
![0.1.3](./docs/dungeon_rogue_0_1_3.gif)

### Added
- Add Main Menu to pause and resume the game. For now it may only pause and resume the game. But every button just resume the game and now it's more fake.
- Add `iyes_loopless` crate to manupulate with the game state. For now player and enemies spawns only when user enter the game in the first time. And when the user in the menu all systems which are related with player or enemy are not running anymore. It should help to manupulate with systems extendability because the logic has been written for the whole plugin but not for the each system.

---

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

---

## [0.1.1]
### Fixed
- [debug] Update debug layer when dimensions are changed

---

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
