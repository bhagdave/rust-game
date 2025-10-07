# Quickstart: Demo Level Validation

## Purpose
Validate that the demo level feature meets all functional requirements through manual and automated testing.

## Prerequisites
- Rust 1.75+ installed
- Project built: `cargo build --release`
- All tests passing: `cargo test`

## Manual Validation Steps

### Step 1: First Run Auto-Load (FR-001, FR-005)
**Validates**: System automatically loads demo level without configuration

```bash
# Clean any existing save data
rm -rf ~/.local/share/rust-game/ 2>/dev/null

# Run the game
cargo run --release
```

**Expected Result**:
- ✅ Game window opens within 10 seconds
- ✅ Demo level visible immediately (no main menu, no load screen pause)
- ✅ Tilemap rendered with tiles
- ✅ Player character visible at spawn point

**Failure Indicators**:
- ❌ Black screen or crash
- ❌ Main menu appears instead
- ❌ Load time exceeds 10 seconds

---

### Step 2: Visual Asset Display (FR-002)
**Validates**: Visible game assets displayed without manual configuration

**Actions**:
1. Observe the rendered demo level
2. Identify visible elements: tiles, player sprite, interactive objects

**Expected Result**:
- ✅ Tilemap texture applied (not solid colors)
- ✅ Player sprite visible (not placeholder)
- ✅ Interactive objects visible (doors, items)
- ✅ All graphics properly loaded

**Failure Indicators**:
- ❌ Magenta placeholder sprites (indicates missing assets)
- ❌ Solid color tiles (tilemap texture failed)
- ❌ Invisible or transparent sprites

---

### Step 3: Player Movement (FR-003, FR-004)
**Validates**: Basic player interactions enabled - movement mechanic

**Actions**:
1. Press arrow keys / WASD / gamepad stick to move
2. Observe player character movement
3. Try jumping (if implemented) with Space / gamepad button

**Expected Result**:
- ✅ Player moves left/right with keyboard input
- ✅ Player responds to gamepad input (if connected)
- ✅ Movement feels responsive (<50ms input lag)
- ✅ Player sprite animates (if walk animation exists)

**Failure Indicators**:
- ❌ Player does not move
- ❌ Visible input delay (>50ms lag)
- ❌ Movement jittery or stuttering

---

### Step 4: Object Interaction (FR-004)
**Validates**: Basic interactions with objects - doors, items, pickups

**Actions**:
1. Move player near an interactive object (door, item)
2. Press interaction key (E / gamepad A)
3. Observe interaction result

**Expected Result**:
- ✅ Interaction prompt appears when near object ("Press E to open")
- ✅ Door opens / item collected on key press
- ✅ Visual feedback shown (animation, sprite change)
- ✅ Interaction completes within 50ms of input

**Failure Indicators**:
- ❌ No interaction prompt displayed
- ❌ Key press has no effect
- ❌ Crash or error on interaction

---

### Step 5: Tilemap Rendering (FR-004)
**Validates**: 2D tilemap rendering capability

**Actions**:
1. Observe the game environment
2. Move player around to view different areas
3. Check for tile variety and proper rendering

**Expected Result**:
- ✅ Tiles properly aligned (no gaps or overlaps)
- ✅ Multiple tile types visible (floor, wall, decoration)
- ✅ Tilemap scrolls smoothly with player movement
- ✅ No visual artifacts or rendering glitches

**Failure Indicators**:
- ❌ Misaligned tiles (gaps between tiles)
- ❌ Flickering or z-fighting
- ❌ Tiles not updating with camera movement

---

### Step 6: Asset Fallback Behavior (FR-008)
**Validates**: Placeholder graphics when assets missing/corrupted

**Actions**:
1. Exit the game
2. Rename/delete an asset: `mv assets/sprites/door.png assets/sprites/door.png.bak`
3. Run game again: `cargo run --release`
4. Observe the area where door sprite should be

**Expected Result**:
- ✅ Game still runs (no crash)
- ✅ Magenta placeholder sprite shown where door.png should be
- ✅ Warning logged to console about missing asset
- ✅ Player can still interact with door (functionality preserved)

**Cleanup**:
```bash
mv assets/sprites/door.png.bak assets/sprites/door.png
```

**Failure Indicators**:
- ❌ Game crashes on startup
- ❌ Nothing rendered where sprite should be
- ❌ No console warning about missing asset

---

### Step 7: Performance Validation
**Validates**: 30 FPS minimum, <10s load, <50ms input lag

**Actions**:
1. Run with performance monitoring:
   ```bash
   cargo run --release --features bevy/trace
   ```
2. Observe FPS counter (if available) or frame timing
3. Measure load time with stopwatch from launch to playable
4. Test input responsiveness subjectively

**Expected Result**:
- ✅ FPS stays >= 30 (ideally 60) during gameplay
- ✅ Load time < 10 seconds (typically 1-3s)
- ✅ Input feels responsive (< 50ms perceptible lag)
- ✅ No frame stutters or hitches

**Failure Indicators**:
- ❌ FPS drops below 30 during normal gameplay
- ❌ Load time exceeds 10 seconds
- ❌ Noticeable input delay

---

### Step 8: Repeat Run Consistency (FR-007)
**Validates**: Demo accessible on every run without persistent state

**Actions**:
1. Exit the game (close window)
2. Run again: `cargo run --release`
3. Observe demo level state

**Expected Result**:
- ✅ Demo level loads again automatically
- ✅ Player spawns at same starting position
- ✅ All interactive objects reset to initial state
- ✅ No saved progress from previous run

**Failure Indicators**:
- ❌ Demo doesn't load on second run
- ❌ Player spawns at different location
- ❌ Objects remember state from previous run

---

## Automated Test Validation

### Run All Tests
```bash
# Unit and integration tests
cargo test --all

# Specific demo tests
cargo test demo_level_loading
cargo test demo_asset_fallback
cargo test demo_performance
```

**Expected Output**:
```
test tests::demo_level_loading::loads_within_10_seconds ... ok
test tests::demo_level_loading::spawns_player_correctly ... ok
test tests::demo_asset_fallback::uses_placeholder_on_missing ... ok
test tests::demo_performance::maintains_30_fps ... ok
test tests::demo_performance::input_lag_under_50ms ... ok
```

### Performance Benchmarks (Optional)
```bash
cargo bench --bench lighting_bench  # Existing benchmark
# Add demo_bench if created
```

---

## Success Criteria Checklist

All items must pass for feature acceptance:

**Functional Requirements**:
- [ ] FR-001: Demo level loads automatically on first run
- [ ] FR-002: Visible assets displayed without configuration
- [ ] FR-003: Player movement enabled and responsive
- [ ] FR-004: Movement, interactions, tilemap all demonstrated
- [ ] FR-005: No user configuration required
- [ ] FR-006: Developers can validate functionality through demo
- [ ] FR-007: Demo accessible on every run (no persistent state)
- [ ] FR-008: Placeholder graphics shown for missing assets

**Performance Requirements**:
- [ ] Demo loads within 10 seconds
- [ ] Maintains minimum 30 FPS during gameplay
- [ ] Input lag < 50ms (subjectively responsive)

**Quality Gates**:
- [ ] All cargo tests pass
- [ ] No clippy warnings: `cargo clippy -- -D warnings`
- [ ] Code formatted: `cargo fmt --check`
- [ ] No console errors during demo run

---

## Troubleshooting

### Issue: Demo doesn't auto-load
**Diagnosis**: Check GameState initialization in main.rs
**Fix**: Ensure DemoPlugin is added to app and GameState starts as Loading

### Issue: Magenta placeholders everywhere
**Diagnosis**: Assets not found or paths incorrect
**Fix**: Verify assets/ directory structure matches demo_level.ron sprite_path values

### Issue: FPS < 30
**Diagnosis**: Performance bottleneck in rendering or systems
**Fix**: Run `cargo build --release` (not debug), profile with `cargo flamegraph`

### Issue: Input lag noticeable
**Diagnosis**: System ordering or heavy frame processing
**Fix**: Check system ordering in DemoPlugin, ensure input processed early in schedule

---

## Next Steps

After successful quickstart validation:
1. Mark all FR requirements as validated ✅
2. Document any performance metrics in plan.md
3. Proceed to production integration (if demo becomes permanent feature)
4. Or archive demo and build main game content
