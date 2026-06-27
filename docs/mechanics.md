# Mechanics — flow

## Playable Character

**Kai only.** Third-person camera, behind-and-above with spring-arm follow.

---

## Core Inputs

Four buttons plus the movement stick. No complex sequences.

| Input | Action |
|-------|--------|
| Stick | Move / strafe |
| Jump | Jump; tap again midair for double jump |
| Dash | Short burst forward; one use per airborne sequence, resets on landing |
| Grab | Ledge grab, vault over low obstacles, wall climb (at full flow) |

All depth comes from the flow meter and spatial reading — not input complexity.

---

## Flow Meter

Builds while Kai is continuously in motion: running, jumping, dashing, wall-running.
Drains when he stops, idles, or takes a disruption hit.

### Three Tiers

**Low flow** (0–40%)
- Normal movement
- Dash has a short cooldown (~0.8s)
- Wall run lasts ~1.5s before Kai falls off

**Mid flow** (40–80%)
- Dash cooldown removed
- Wall run extends to ~3s
- Vaults snap faster — less animation lock

**Full flow** (80–100%)
- **Wall climb unlocked**: hold Grab while running at a vertical surface to scale it
- Wall climb lasts ~2s before Kai must jump off or grab a ledge
- Dash slightly longer distance

Full flow is the vertical key. Routes requiring wall climb are only accessible when Kai has been moving continuously. Stopping resets the option.

---

## Parkour Moves

### Vault
Approach a low obstacle at running speed and press Grab — Kai vaults automatically.
No timing window required. Rewards momentum; vaulting while stationary is slow.

### Ledge Grab
Near a ledge edge, tap Jump to grab the lip and pull up.
Can also grab by running off a ledge and pressing Grab within a short window.

### Wall Run
Run toward a wall at an angle and press Jump — Kai runs horizontally along it.
Duration scales with flow tier. Jump off the wall to redirect or gain height.

### Wall Climb (full flow only)
From a wall run or while running at a vertical surface, hold Grab.
Kai scales the wall vertically for ~2s. Jump off to reach a high ledge or launch to another surface.
Losing flow mid-climb causes Kai to slide and fall.

### Dash Timing
Dashing just before a jump noticeably extends the arc. Not a hidden combo — just physics that
rewards reading your own momentum. Landing from a dash-boosted jump and immediately dashing again
preserves speed (no deceleration on landing while in motion).

---

## Disruption System (no health bar)

Getting caught by agency personnel or hit by a spacetime wisp adds a disruption state:
- Screen edges bloom with color; geometry slightly warps around Kai
- Flow meter resets to zero instantly
- Three disruptions within a short window (~15s) triggers **shaken state**:
  - Movement sluggish for ~20s
  - Camera wobbles slightly
  - World geometry briefly more unstable visually
  - No game over; shaken state fades naturally

There is no death and no health bar. Danger reads as the world becoming geometrically unstable
around Kai — consistent with the game's visual language.

---

## Enemies / Hazards

**Agency personnel**: Pursue Kai in restricted areas. Getting caught escorts him back to the last
open checkpoint. Not violent — they're doing their job. Behavior escalates in Act 3.

**Spacetime wisps**: Environmental hazards near anomaly sites. They don't chase but drift
unpredictably. Moving through one triggers a disruption. Visually beautiful — luminous geometry
fragments. Avoiding them is about reading their drift patterns, not reaction speed.

**Final confrontation (Act 3)**: The agency director blocks access to the sealed wing. Not a
boss fight — a dialogue confrontation. The party can't leave until Daven reveals what he knows
and the director decides whether to stand aside.

---

## Level Design Principles

Every level has two routes:
- **Ground path**: always accessible, no flow requirement
- **High path**: requires sustained flow to reach via wall climb; faster, shorter, better view

The Fracture level (sealed wing, Act 3) breaks this — geometry shifts dynamically, making which
path is "high" and which is "low" change as you move through. Wall climb becomes essential because
the floor is sometimes the ceiling.

Vertical level design goals:
- Open vertical space rewarding flow maintenance
- Platforms at varying heights reachable only at mid/full flow
- Wall-climbable surfaces visually distinct (slightly different material/color treatment)
- Flow loss should feel like a natural consequence of stopping, not a punishment

---

## Camera

Spring-arm follow, behind-and-above Kai. Pulls slightly ahead in the direction of movement to
show what's coming. Zooms in slightly during dialogue. During wall effect moments (narrative
punctuation), camera briefly locks and tilts — then releases.
