# AGENTS.md — flow

## Project

Rust/Bevy 0.16 third-person platformer. Working title: **flow**.

## Quick orientation

- Design intent → `docs/`
- Game code → `src/`
- Blender export tool → `tools/blender_flow_exporter/`
- Level assets → `assets/levels/` (glTF/GLB, authored in Blender)

## Key design decisions

- **No health bar.** Disruption system: 3 hits in 15s = shaken state (20s, halved speed).
- **Flow meter gates movement.** Wall climb only at full flow (80–100%). Builds while moving, drains on idle.
- **Geometry effects are rare.** 5 wall-effect moments total (screen saturation + inward breathing), one Fracture level in Act 3.
- **Synty assets via asset server** at `srv:/media/data/other/asset-server/cooked/`. Packs: Scifi Space (primary), Shopping Plaza, Western, Fantasy Kingdom, Ancient Egypt, Prototype.
- **Blender is the level editor.** Blender → glTF → Bevy hot-reload. No in-game editor.

## Name conventions (glTF)

Objects in Blender levels follow these prefixes:
- `Collider_*` → Avian physics collider added at load
- `Trigger_*` → sensor zone; glTF extras become `TriggerData`

## Running tests

```sh
cargo test
```

45 tests, all passing as of initial scaffold.

## Docs index

| Doc | Contents |
|-----|----------|
| [narrative.md](docs/narrative.md) | Full story: premise, characters, act structure, loss, ending |
| [characters.md](docs/characters.md) | Kai, Sela, Daven, Pell + 4 rivals |
| [mechanics.md](docs/mechanics.md) | Input map, flow meter tiers, disruption, level design principles |
| [art-style.md](docs/art-style.md) | Color palette per character, wall effect, Fracture level, asset pipeline |
