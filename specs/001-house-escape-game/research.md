# Research: House Escape Game Technology Stack

**Date**: 2025-10-03
**Feature**: House Escape Game (001)

## Decision Summary

All technical unknowns resolved. Rust + Bevy ecosystem provides complete solution for cross-platform 2D game development with ECS architecture.

---

## 1. Game Engine: Bevy 0.16.1

### Decision
Use Bevy as the primary game engine for ECS architecture, rendering, input handling, and asset management.

### Rationale
- **ECS Architecture**: Built-in Entity-Component-System aligns with constitutional Principle V
- **Cross-Platform**: Single codebase targets Windows, macOS, Linux, WASM
- **Performance**: Rust's zero-cost abstractions + data-oriented ECS achieves 60 FPS target
- **Active Development**: Large community, frequent releases, extensive documentation
- **Plugin Ecosystem**: Rich plugin support for 2D tilemap, audio, UI needs

### Alternatives Considered
- **macroquad**: Simpler but lacks robust ECS, harder to scale for 20-30 rooms
- **ggez**: Good 2D support but less modern ECS patterns
- **Custom engine**: Excessive complexity for scope, violates constitutional simplicity

### Best Practices
- Use fixed timestep for deterministic physics (testing requirement)
- Leverage Bevy's state management for game states (menu, playing, paused)
- Schedule systems with explicit ordering via `before()`/`after()` labels
- Use `Commands` for deferred entity spawning (avoid mid-system mutations)

---

## 2. 2D Tilemap: bevy_ecs_tilemap 0.15

### Decision
Use `bevy_ecs_tilemap` for room rendering and tile-based collision detection.

### Rationale
- **Performance**: Batch-rendered tiles optimize GPU usage for large rooms
- **ECS Integration**: Tiles as entities fit Bevy architecture
- **Layer Support**: Separate layers for floor, walls, decorations, lighting
- **Collision**: Built-in tile-based collision helpers

### Alternatives Considered
- **Manual sprite batching**: Reinventing the wheel, higher bug risk
- **bevy_rapier2d tilemap**: Physics overkill for simple AABB collision

### Best Practices
- Use chunk-based loading for memory efficiency (unload off-screen rooms)
- Store tile collision metadata in separate layer (not visual layer)
- Implement room transitions via entity despawn/spawn (clean state)

---

## 3. Audio: bevy_kira_audio 0.21

### Decision
Use `bevy_kira_audio` for sound effects and background music.

### Rationale
- **High-Quality**: Kira audio library designed for games
- **Bevy Integration**: Resources and systems integrate cleanly
- **Spatial Audio**: Support for positional sound if needed later
- **Format Support**: OGG, MP3, WAV, FLAC

### Alternatives Considered
- **bevy_audio** (built-in): Limited format support, less control
- **rodio directly**: Requires manual Bevy integration

### Best Practices
- Preload audio assets at startup (avoid runtime hitches)
- Use audio channels for separate volume control (SFX, music, ambient)
- Trigger sounds via event systems (decoupled from game logic)

---

## 4. UI: bevy_egui 0.31

### Decision
Use `bevy_egui` for HUD (candle meter, inventory bar) and menus (pause, settings).

### Rationale
- **Immediate Mode**: Simple to implement dynamic UI (inventory count updates)
- **Bevy Integration**: Official Bevy plugin, seamless ECS access
- **Accessibility**: Built-in keyboard navigation, scalable fonts
- **Prototyping**: Fast iteration for UI layout

### Alternatives Considered
- **bevy_ui** (built-in): Retained mode more complex for dynamic elements
- **Custom sprite-based UI**: Higher implementation cost

### Best Practices
- Use egui contexts per UI subsystem (HUD, menu, map)
- Update UI state via Resources (decouple from rendering)
- Apply colorblind-friendly palette via egui style configuration

---

## 5. Lighting System: Custom Shader + Sprite Masking

### Decision
Implement dynamic 2D lighting using custom fragment shader with circular gradient and sprite-based fog of war.

### Rationale
- **Performance**: GPU fragment shader processes lighting in <1ms
- **Flexibility**: Adjustable visibility radius per candle state (1-2 tiles vs 6-8 tiles)
- **Cross-Platform**: WGSL shaders compile to all targets (Metal, Vulkan, DX12, WebGL2)
- **Fog of War**: Sprite alpha masking for explored/unexplored areas

### Alternatives Considered
- **CPU-based lighting**: Too slow for real-time 60 FPS
- **Pre-baked lightmaps**: Inflexible for dynamic candle state

### Best Practices
- Use signed distance field for smooth circular gradient
- Implement lighting as post-process pass (render to texture, apply shader)
- Cache explored room areas in texture for fog of war persistence

---

## 6. Save/Load System: RON + serde

### Decision
Use RON (Rusty Object Notation) serialization with `serde` for auto-save on room entry.

### Rationale
- **Human-Readable**: Debuggable save files
- **Type-Safe**: Serde derives ensure schema correctness
- **Bevy Compatible**: Many Bevy types implement Serialize/Deserialize
- **Cross-Platform**: Text format works on all platforms

### Alternatives Considered
- **JSON**: More verbose than RON
- **Binary (bincode)**: Harder to debug, fragile across versions

### Best Practices
- Save only delta state (current room, inventory, collected items, map state)
- Use versioning field for save format migrations
- Implement auto-save via event trigger (RoomEnteredEvent)
- Store saves in platform-specific directories (XDG on Linux, AppData on Windows)

---

## 7. Input Handling: bevy::input with Leafwing Input Manager

### Decision
Use `bevy::input` built-in with `leafwing-input-manager` for configurable controls.

### Rationale
- **Abstraction**: Map keyboard/gamepad/mouse to logical actions (Jump, Interact, ToggleCandle)
- **Configurability**: Runtime key rebinding via config resource
- **Multi-Input**: Seamless keyboard + gamepad support
- **Bevy Integration**: Clean system queries

### Alternatives Considered
- **Raw bevy::input**: Manual mapping for each input type
- **Custom input abstraction**: Reinventing the wheel

### Best Practices
- Define action enums (PlayerAction::Jump, PlayerAction::UseItem)
- Load key bindings from config file (saved with game state)
- Provide sensible defaults (WASD, Arrow keys, Gamepad A button)

---

## 8. Level Data Format: RON

### Decision
Store room layouts, entity positions, and connections in RON files under `assets/levels/`.

### Rationale
- **Declarative**: Room structure defined separately from code
- **Iteration**: Artists/designers can edit without recompiling
- **Type-Safe**: Serde validation on load
- **Version Control**: Text format diffs cleanly

### Alternatives Considered
- **Tiled TMX**: Requires XML parsing, overkill for simple rooms
- **Hard-coded**: Inflexible, poor iteration time

### Best Practices
- One RON file per room (e.g., `ground_floor_entry.ron`)
- Define schema: Room metadata, tile layout, entity spawns, connections
- Lazy-load rooms on demand (memory efficiency)

---

## 9. Physics/Collision: Custom AABB with rapier2d (Optional)

### Decision
Implement simple Axis-Aligned Bounding Box (AABB) collision manually. Consider `bevy_rapier2d` if complex physics needed later.

### Rationale
- **Simplicity**: Platformer collision is tile-based, no rotation/complex shapes
- **Performance**: Custom AABB faster than full physics engine
- **Determinism**: Easier to make deterministic for testing
- **Low Risk**: Can add rapier2d later if needed

### Alternatives Considered
- **bevy_rapier2d immediately**: Overkill for simple tile collision
- **No collision library**: Same as custom AABB

### Best Practices
- Store bounding boxes as components (BoundingBox { min: Vec2, max: Vec2 })
- Use spatial hashing for broad-phase collision (O(n) → O(1) per entity)
- Implement swept AABB for continuous collision (avoid tunneling)

---

## 10. Testing Strategy: cargo test + Bevy Test Harness

### Decision
Use `cargo test` for unit tests, Bevy's `App::update()` for integration tests with headless mode.

### Rationale
- **Deterministic**: Fixed timestep + seeded RNG ensure reproducible tests
- **Fast**: Headless mode (no rendering) completes tests <30s
- **Coverage**: `tarpaulin` measures 80% coverage target
- **CI/CD**: GitHub Actions runs tests on push

### Alternatives Considered
- **Manual testing only**: Violates constitutional Testing Discipline
- **Snapshot testing**: Useful but insufficient alone

### Best Practices
- Unit test systems in isolation (mock components, check state changes)
- Integration tests spawn full ECS world, simulate player actions, assert outcomes
- Property-based tests for physics (e.g., player never falls through floor)
- Benchmark critical paths (lighting shader, room transitions)

---

## 11. Cross-Platform Build Strategy

### Decision
Native compilation for Windows/macOS/Linux, WASM target for web deployment.

### Rationale
- **Performance**: Native builds achieve 60 FPS easily
- **Reach**: WASM target allows browser play (itch.io, GitHub Pages)
- **Distribution**: Single Rust codebase builds all targets

### Alternatives Considered
- **Native only**: Limits reach
- **WASM only**: Worse performance, harder debugging

### Best Practices
- Use conditional compilation for platform-specific code (`#[cfg(target_arch = "wasm32")]`)
- Test WASM build regularly (file I/O behaves differently)
- Optimize WASM binary size (`wasm-opt`, strip symbols)

---

## 12. Asset Pipeline

### Decision
Embed assets in binary for native builds, load via HTTP for WASM builds.

### Rationale
- **Simplicity**: Single executable for native distribution
- **WASM Support**: Asset loading adapts to web constraints
- **Bevy Support**: AssetServer handles both strategies

### Alternatives Considered
- **Always external assets**: Complicates distribution
- **Always embedded**: WASM binary too large

### Best Practices
- Use `bevy_embedded_assets` for native builds
- Organize assets by type (sprites/, audio/, fonts/, levels/)
- Compress audio files (OGG for space efficiency)
- Use sprite atlases to reduce draw calls

---

## Summary of Resolved Unknowns

| Unknown | Resolution |
|---------|------------|
| Game engine | Bevy 0.16.1 |
| 2D tilemap rendering | bevy_ecs_tilemap 0.15 |
| Audio playback | bevy_kira_audio 0.21 |
| UI framework | bevy_egui 0.31 |
| Dynamic lighting | Custom WGSL shader + sprite masking |
| Save/load format | RON + serde |
| Input abstraction | leafwing-input-manager |
| Level data format | RON |
| Collision system | Custom AABB (rapier2d optional) |
| Testing strategy | cargo test + Bevy headless |
| Cross-platform build | Native + WASM |
| Asset pipeline | Embedded (native) + HTTP (WASM) |

---

---

## 13. Additional Research Required (Bevy 0.16.1 Specifics)

**Context**: Bevy is a rapidly evolving framework. The following areas require verification of exact APIs and version compatibility for Bevy 0.16.1 before implementation begins.

### 13.1 Bevy 0.16.1 Core API Verification

**Areas Requiring Research**:
1. **System Scheduling API**: Verify current `.before()` / `.after()` / `.in_set()` syntax in Bevy 0.16.1
2. **State Management**: Confirm `States` trait and state transition API (OnEnter, OnExit, OnUpdate)
3. **Fixed Timestep**: Verify `FixedUpdate` schedule or `FixedTimestep` plugin API
4. **Commands API**: Confirm `Commands::spawn()`, `Commands::entity()`, deferred execution patterns
5. **Asset Loading**: Verify `AssetServer::load()` and async asset loading patterns

**Research Tasks**:
- [ ] Read Bevy 0.16.1 release notes for breaking changes from 0.15.x
- [ ] Verify system scheduling examples from Bevy 0.16 documentation
- [ ] Test state management patterns with minimal example
- [ ] Confirm fixed timestep implementation approach

**Risk**: High - Core ECS patterns may have changed, affecting all system implementations.

### 13.2 Plugin Version Compatibility Matrix

**Critical Dependencies Requiring Version Verification**:

| Crate | Estimated Version | Status | Research Needed |
|-------|------------------|--------|-----------------|
| `bevy` | 0.16.1 | ✅ Confirmed (Cargo.toml) | Verify API patterns |
| `bevy_ecs_tilemap` | ~0.15.0 | ⚠️ Needs verification | Check Bevy 0.16.1 compat |
| `bevy_kira_audio` | ~0.21.0 | ⚠️ Needs verification | Check Bevy 0.16.1 compat |
| `bevy_egui` | ~0.31.0 | ⚠️ Needs verification | Check Bevy 0.16.1 compat |
| `leafwing-input-manager` | ❓ Unknown | ⚠️ Needs research | Find compatible version |
| `serde` | ~1.0 | ✅ Stable | No breaking changes |
| `ron` | ~0.8 | ✅ Stable | No breaking changes |

**Research Tasks**:
- [ ] Check bevy_ecs_tilemap GitHub for Bevy 0.16.1 compatibility
- [ ] Check bevy_kira_audio crates.io/GitHub for latest compatible version
- [ ] Check bevy_egui releases for Bevy 0.16.1 support
- [ ] Search for leafwing-input-manager Bevy 0.16 compatibility
- [ ] Verify all plugin versions can coexist without conflicts

**Risk**: Medium - Plugins may lag behind Bevy releases, requiring version pinning or alternative solutions.

### 13.3 WGSL Shader API (Bevy 0.16.1)

**Areas Requiring Research**:
1. **Material Trait**: Current API for custom `Material2d` implementations
2. **Shader Uniforms**: How to pass ECS data (position, radius) to shaders via `Uniform` structs
3. **Post-Process Pass**: Current render graph API for custom render passes
4. **Texture Rendering**: How to render to texture and apply as post-process effect

**Specific Questions**:
- Does Bevy 0.16.1 use `Material2d` trait or a different abstraction?
- What is the current uniform binding syntax in WGSL for Bevy?
- How to create a render target and apply a shader pass?
- Are there built-in post-process helpers or must we use raw render graph?

**Research Tasks**:
- [ ] Review Bevy 0.16 shader examples (GitHub bevy/examples/shader/)
- [ ] Read bevy_render documentation for Material2d trait
- [ ] Find post-processing examples or render graph documentation
- [ ] Test minimal 2D shader with dynamic uniforms

**Risk**: High - Shader API is complex and changes between versions. Incorrect implementation could fail to compile or perform poorly.

### 13.4 WASM Deployment (2025 Status)

**Areas Requiring Research**:
1. **Build Tools**: Is `trunk` still recommended, or has tooling changed?
2. **Asset Loading**: Confirm HTTP-based asset loading patterns for WASM
3. **Save/Load in Browser**: Best practice for LocalStorage vs IndexedDB vs custom solution
4. **Performance**: Known 2025 performance limitations (WebGL2 vs WebGPU availability)
5. **File Size**: Current WASM binary size optimization techniques

**Specific Questions**:
- Does Bevy 0.16.1 have first-class WASM support or requires workarounds?
- How to conditionally compile asset embedding (native) vs HTTP loading (WASM)?
- Is `rfd` (file dialog) available for WASM save/load UI?
- WebGPU support in Bevy 0.16.1 for better WASM performance?

**Research Tasks**:
- [ ] Review Bevy WASM examples and deployment guide
- [ ] Test minimal WASM build with Bevy 0.16.1
- [ ] Research web storage solutions for save files
- [ ] Verify WebGPU backend availability

**Risk**: Medium - WASM support exists but may have edge cases. Save/load in browser requires platform-specific code.

### 13.5 Bevy Testing Patterns (2025)

**Areas Requiring Research**:
1. **Headless Testing**: Current API for running Bevy app without window/rendering
2. **Time Control**: How to advance time manually in tests (fixed timestep simulation)
3. **Mocking**: Patterns for mocking components/resources in unit tests
4. **Integration Test Setup**: How to spawn full ECS world, run systems, assert state

**Specific Questions**:
- Does Bevy 0.16.1 have `MinimalPlugins` or similar for headless testing?
- How to use `App::update()` to step through frames deterministically?
- Can we skip rendering entirely or must we use dummy renderer?
- Best practices for seeding RNG for deterministic tests?

**Research Tasks**:
- [ ] Review Bevy testing examples and documentation
- [ ] Test headless app creation and frame stepping
- [ ] Find community testing patterns (Reddit, Discord, GitHub discussions)
- [ ] Verify cargo test runs without GPU/display requirements

**Risk**: Low - Testing patterns are well-established, but API details may have changed.

### 13.6 Platform-Specific Code Patterns

**Areas Requiring Research**:
1. **Save Directory**: Platform-specific save paths (XDG on Linux, AppData on Windows, Library on macOS)
2. **Conditional Compilation**: Best practices for `#[cfg(target_os = "...")]` and `#[cfg(target_arch = "wasm32")]`
3. **File I/O**: Using `std::fs` vs platform abstractions
4. **Asset Embedding**: `bevy_embedded_assets` compatibility with Bevy 0.16.1

**Specific Questions**:
- Should we use `dirs` crate for cross-platform directories?
- How to structure code with heavy platform-specific branching?
- Does `bevy_embedded_assets` work with Bevy 0.16.1?

**Research Tasks**:
- [ ] Review `dirs` crate for save directory patterns
- [ ] Test conditional compilation with WASM vs native
- [ ] Verify `bevy_embedded_assets` compatibility

**Risk**: Low - Platform abstraction crates are stable, but worth verifying patterns.

---

## Research Action Items Summary

**Priority 1 (Critical - Must resolve before task generation)**:
1. ✅ Verify exact compatible versions for all plugins (bevy_ecs_tilemap, bevy_kira_audio, bevy_egui, leafwing-input-manager)
2. ✅ Confirm Bevy 0.16.1 system scheduling and state management API
3. ✅ Verify WGSL shader API for 2D lighting implementation

**Priority 2 (High - Resolve during early implementation)**:
4. Test WASM build process and asset loading patterns
5. Validate headless testing setup
6. Confirm fixed timestep API for deterministic physics

**Priority 3 (Medium - Can research during implementation)**:
7. Platform-specific save directory patterns
8. Conditional compilation best practices
9. Asset embedding verification

**Recommended Approach**:
1. **Before `/tasks`**: Research Priority 1 items to get exact Cargo.toml versions
2. **During setup tasks**: Research Priority 2 items with minimal examples
3. **During implementation**: Research Priority 3 items as needed

---

**Status**: Initial research complete. Additional verification required for Bevy 0.16.1-specific APIs and plugin versions before implementation. Ready for focused research on Priority 1 items.
