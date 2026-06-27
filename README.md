# flow

A third-person 3D platforming adventure about four strangers pulled toward an abandoned physics facility by a signal they feel rather than hear.

Near-future Earth, 2055. Low-poly Synty aesthetic. Lazarus-inspired saturated color palette. No health bar. No combat. Parkour movement with a flow meter that gates wall climbing. A short story about something ending, and who you want beside you when it does.

## Run

```sh
cargo run
```

## Design docs

- [Narrative](docs/narrative.md) — premise, characters, act structure, ending
- [Characters](docs/characters.md) — Kai, Sela, Daven, Pell + their rivals
- [Mechanics](docs/mechanics.md) — movement, flow meter, disruption system
- [Art Style](docs/art-style.md) — color language, visual effects, asset pipeline

## Structure

```
src/
  main.rs          — app entry point
  state.rs         — GameState enum
  loading/         — loading screen
  menu/            — main menu
  settings/        — settings resource + persistence
  pause/           — pause menu
  save.rs          — save/load (ron)
  cutscene/        — cutscene sequence player
  game/
    player.rs      — Kai entity + input
    camera.rs      — spring-arm follow camera
    movement.rs    — run, jump, dash
    wall_movement/ — wall run, wall jump, wall climb
    flow.rs        — flow meter (Low/Mid/Full tiers)
    disruption.rs  — hit system, shaken state
    world.rs       — level loading, name conventions
    dialogue.rs    — dialogue queue + speaker colors
    wall_effect.rs — 8s saturation post-process event
    wisp.rs        — spacetime wisp hazard
tools/
  blender_flow_exporter/   — Blender addon: auto-export glTF on save
assets/
  levels/          — glTF level files (authored in Blender)
docs/              — design documents
```

## Blender workflow

Install `tools/blender_flow_exporter/` as a Blender addon. Enable **FOLD Auto-Exporter** in the N-panel (View3D sidebar → FOLD tab). Set export path to `../assets/levels/`, enable auto-export, and Blender will export to glTF whenever you save the `.blend` file. Bevy hot-reloads the scene automatically.

Level objects follow a naming convention:
- `Collider_*` → gets a physics collider
- `Trigger_*` → becomes a trigger zone; custom properties exported as glTF extras
