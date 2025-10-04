# Quickstart: House Escape Game

**Date**: 2025-10-03
**Feature**: House Escape Game (001)

## Purpose

This quickstart guide validates the core game loop through manual testing scenarios. Execute these tests after implementation to verify acceptance criteria from spec.md.

---

## Prerequisites

1. **Build the game**:
   ```bash
   cargo build --release
   ```

2. **Run the game**:
   ```bash
   cargo run --release
   ```

3. **Start a new game** from the main menu.

---

## Test Scenario 1: Candle and Lighting System

**Objective**: Verify candle wax depletion, match usage, and visibility radius changes (FR-002, FR-003, FR-004, FR-005, FR-007).

**Steps**:
1. Start game in entry hall with lit candle
2. Observe candle wax meter (top left) - should be at 100%
3. Wait 30 seconds
4. **Expected**: Candle wax decreases visibly (meter updates)
5. **Expected**: Visibility radius remains large (6-8 tiles)
6. Press **E** key to manually extinguish candle
7. **Expected**: Visibility radius shrinks to 1-2 tiles (fog of war expands)
8. **Expected**: Candle wax meter stops decreasing
9. Press **E** key again to relight candle
10. **Expected**: Candle lights, visibility expands, wax resumes decreasing
11. Wait until wax meter reaches 0%
12. **Expected**: Candle automatically extinguishes, visibility shrinks
13. Press **M** key to use a match (pick one up from platform first)
14. **Expected**: Candle relights to full wax (100%), visibility expands
15. Check match count (top left) - should decrease by 1

**Pass Criteria**:
- ✅ Candle wax depletes over time when lit
- ✅ Manual toggle works (extinguish/relight)
- ✅ Visibility radius changes based on candle state
- ✅ Match usage refills wax to 100%
- ✅ Match count decrements correctly

---

## Test Scenario 2: Player Movement and Jump Mechanics

**Objective**: Verify platformer controls, jump mechanics, and double jump powerup (FR-001, FR-006).

**Steps**:
1. Use **A**/**D** keys (or Left/Right arrows) to move player
2. **Expected**: Player moves smoothly left and right
3. Press **Space** to jump from ground
4. **Expected**: Player jumps upward
5. While in mid-air, press **Space** again
6. **Expected**: Nothing happens (double jump not unlocked yet)
7. Navigate to platform with Double Jump Item (glowing blue orb)
8. Walk over item to collect
9. **Expected**: Item disappears, added to inventory
10. Jump from ground, then press **Space** in mid-air
11. **Expected**: Player performs double jump (boost upward)
12. Climb ladder by standing in front and pressing **W** (or Up arrow)
13. **Expected**: Player ascends ladder smoothly

**Pass Criteria**:
- ✅ Horizontal movement responsive (A/D keys)
- ✅ Single jump works from ground
- ✅ Double jump blocked until powerup collected
- ✅ Double jump works after powerup collected
- ✅ Ladder climbing functional

---

## Test Scenario 3: Inventory and Item Pickup

**Objective**: Verify inventory management, item collection, and UI display (FR-011, FR-012, FR-013, FR-014).

**Steps**:
1. Walk over Match item on ground
2. **Expected**: Match collected, disappears from world
3. **Expected**: Inventory bar (bottom center) shows match icon with count
4. Collect multiple Matches (stackable)
5. **Expected**: Match count increments (e.g., 1 → 2 → 3)
6. Collect Brass Key item
7. **Expected**: Key appears in inventory bar (separate icon)
8. **Expected**: Inventory slots fill left-to-right (max 10 items visible)
9. Collect 10 unique items
10. Try to collect 11th item
11. **Expected**: Inventory full message OR item cannot be picked up

**Pass Criteria**:
- ✅ Items picked up on collision
- ✅ Stackable items (matches) increment count
- ✅ Unique items occupy separate slots
- ✅ Inventory UI updates in real-time
- ✅ Inventory capacity limit enforced

---

## Test Scenario 4: Trap Collision and Instant Death

**Objective**: Verify trap detection, instant death mechanic, and checkpoint respawn (FR-021, FR-023, FR-037, FR-038, FR-039).

**Steps**:
1. Note current inventory (e.g., 3 matches, 1 key)
2. Note candle state (e.g., 75% wax, lit)
3. Walk into spike pit trap
4. **Expected**: Player dies instantly (death animation plays)
5. **Expected**: Player respawns at room entrance after 1 second
6. **Expected**: Inventory preserved (still have 3 matches, 1 key)
7. **Expected**: Candle state preserved (still 75% wax, still lit)
8. **Expected**: Trap resets to armed state (can trigger again)
9. Trigger falling chandelier trap (step on pressure plate)
10. **Expected**: Chandelier falls, player dies instantly
11. **Expected**: Respawn at room entrance with inventory intact

**Pass Criteria**:
- ✅ Trap collision triggers instant death
- ✅ Player respawns at room entrance
- ✅ Inventory items retained after death
- ✅ Candle state (wax, lit/unlit) preserved
- ✅ Traps reset to armed state after respawn

---

## Test Scenario 5: Door Unlocking and Room Transitions

**Objective**: Verify locked doors, key usage, room loading, and auto-save (FR-018, FR-020, Auto-save on room entry).

**Steps**:
1. Approach locked door (Brass Lock icon visible)
2. Press **F** to interact
3. **Expected**: "Door is locked. Brass Key required." message
4. Collect Brass Key from nearby platform
5. Approach same door, press **F**
6. **Expected**: Door unlocks, transition to next room
7. **Expected**: Loading screen (< 1 second per FR)
8. **Expected**: New room loads with different layout
9. **Expected**: Auto-save notification (brief UI popup)
10. Open map (press **Tab**)
11. **Expected**: Previous room marked as explored (visible on map)
12. **Expected**: Current room marked as explored
13. Exit game, restart, load save
14. **Expected**: Player spawns in current room (not entry hall)
15. **Expected**: Map shows previously explored rooms

**Pass Criteria**:
- ✅ Locked doors require correct key
- ✅ Door unlocking and room transition work
- ✅ Room loads in < 1 second
- ✅ Auto-save triggers on room entry
- ✅ Map updates with explored rooms
- ✅ Save/load preserves progress

---

## Test Scenario 6: Puzzle Solving (Pressure Plate)

**Objective**: Verify puzzle interaction, item placement, and reward (FR-026).

**Steps**:
1. Locate pressure plate puzzle room (e.g., library on ground floor)
2. Observe 3 pressure plates and locked door
3. Read environmental clue (painting on wall shows symbols: Book, Gemstone, Fuse)
4. Collect Book, Gemstone, Fuse items from around room
5. Stand on first pressure plate, open inventory (**I** key)
6. Select Book item, press **Use** (**U** key)
7. **Expected**: Book placed on pressure plate (item disappears from inventory)
8. **Expected**: Pressure plate depresses, light turns green
9. Repeat for second plate (Gemstone) and third plate (Fuse)
10. **Expected**: When all plates activated, locked door unlocks
11. **Expected**: Door opens, revealing passage
12. **Expected**: Success sound plays

**Pass Criteria**:
- ✅ Pressure plates respond to item placement
- ✅ Correct item combination unlocks door
- ✅ Incorrect items do not activate plates
- ✅ Puzzle state persists (if player leaves and returns, plates stay activated)

---

## Test Scenario 7: Auto-Revealing Map

**Objective**: Verify map system gradually fills in as rooms are explored (FR-020).

**Steps**:
1. Start new game (map empty)
2. Open map (**Tab** key)
3. **Expected**: Only entry hall visible (current room)
4. Exit to adjacent room (kitchen)
5. Open map
6. **Expected**: Kitchen now visible on map
7. **Expected**: Connection between entry hall and kitchen shown
8. Return to entry hall, then go to dining room
9. Open map
10. **Expected**: Dining room now visible
11. **Expected**: All visited rooms remain visible (persistent)
12. Navigate to unexplored room (basement via stairs)
13. **Expected**: Basement appears on map after entering
14. Exit game, reload save
15. Open map
16. **Expected**: All previously explored rooms still visible

**Pass Criteria**:
- ✅ Map starts empty (or only current room visible)
- ✅ Rooms appear on map after first visit
- ✅ Room connections displayed correctly
- ✅ Map state persists across sessions (saved with game)

---

## Test Scenario 8: Accessibility - Configurable Controls

**Objective**: Verify control remapping functionality (Constitutional Principle III).

**Steps**:
1. Open settings menu (press **Esc**, select "Settings")
2. Navigate to "Controls" tab
3. Click on "Jump" action
4. Press **W** key to rebind
5. **Expected**: Jump now bound to **W** instead of **Space**
6. Close settings, test in-game
7. Press **W**
8. **Expected**: Player jumps
9. Press **Space**
10. **Expected**: No jump (old binding removed)
11. Rebind other actions (Move Left → **J**, Move Right → **L**, etc.)
12. **Expected**: All actions respond to new bindings
13. Open settings, click "Reset to Defaults"
14. **Expected**: Controls revert to default (Space = Jump, etc.)

**Pass Criteria**:
- ✅ All actions can be rebound to different keys
- ✅ New bindings work in-game immediately
- ✅ Old bindings removed (no conflicts)
- ✅ Reset to defaults works
- ✅ Control config saved with game settings

---

## Test Scenario 9: Accessibility - Colorblind Mode

**Objective**: Verify colorblind-friendly palette options (FR per clarifications).

**Steps**:
1. Open settings menu
2. Navigate to "Accessibility" tab
3. Select "Colorblind Mode" dropdown
4. Choose "Deuteranopia" (red-green colorblind)
5. **Expected**: UI colors shift to deuteranopia-safe palette
6. **Expected**: Candle flame changes from orange to blue-white
7. **Expected**: Key/door colors use patterns + color
8. Navigate to puzzle room with colored gemstones
9. **Expected**: Gemstones use distinct patterns in addition to colors
10. Switch to "Protanopia" mode
11. **Expected**: Palette shifts again
12. Switch back to "Normal" mode
13. **Expected**: Original colors restored

**Pass Criteria**:
- ✅ Colorblind modes available in settings
- ✅ Palette changes apply immediately
- ✅ All gameplay-critical colors have pattern alternatives
- ✅ Mode persists across sessions

---

## Test Scenario 10: Performance - 60 FPS Target

**Objective**: Verify game maintains 60 FPS on target hardware (Constitutional Principle IV).

**Steps**:
1. Enable FPS counter (press **F3** for debug overlay)
2. **Expected**: FPS counter displays in top-right corner
3. Navigate through multiple rooms (entry hall → kitchen → library → upstairs)
4. **Expected**: FPS remains at or above 60 FPS
5. Light candle in room with many entities (10+ items, 5+ traps)
6. **Expected**: FPS does not drop below 30 FPS (constitutional minimum)
7. Trigger multiple traps simultaneously (falling chandelier + collapsing floor)
8. **Expected**: FPS dips allowed but recovers quickly
9. Open map while moving
10. **Expected**: FPS stable (map rendering does not cause stutter)
11. Save game (auto-save or manual)
12. **Expected**: No visible frame hitch (file I/O async)

**Pass Criteria**:
- ✅ Average FPS >= 60 during normal gameplay
- ✅ Minimum FPS >= 30 during intensive scenes
- ✅ Frame time variance < 16.67ms (constitutional requirement)
- ✅ No visible stuttering during room transitions or saves

---

## Test Scenario 11: Complete Game Loop

**Objective**: Verify entire game loop from start to exit door (FR-035, FR-036, FR-045).

**Steps**:
1. Start new game
2. Collect matches and keys throughout house
3. Solve puzzles to unlock progression (circuit breaker, pressure plates, etc.)
4. Unlock access to all floors (Ground, First, Second, Basement)
5. Find and collect all required key items for exit door
6. Locate exit door (basement, secret passage)
7. Use final key to unlock exit door
8. Interact with exit door
9. **Expected**: Victory screen displays
10. **Expected**: Completion time shown (e.g., "15:32")
11. **Expected**: Collection percentage shown (e.g., "Matches: 12/15, Diary Pages: 3/8, Secrets: 70%")
12. **Expected**: Option to continue exploring or return to menu

**Pass Criteria**:
- ✅ Exit door reachable after collecting required items
- ✅ Victory condition triggers on exit
- ✅ Completion time accurately tracked
- ✅ Collection stats displayed correctly

---

## Performance Benchmarks

Run these benchmarks to verify constitutional performance requirements:

### Startup Time

```bash
time cargo run --release
```

**Expected**: Game window appears in < 5 seconds (Constitutional Principle IV).

### Room Transition Time

1. Enable debug timer (**F3** overlay)
2. Walk through door to trigger room transition
3. Measure time from door interaction to new room fully loaded

**Expected**: < 1 second per spec (< 3 seconds constitutional max for level loading).

### Frame Time

1. Enable frame time graph (**F4** for detailed overlay)
2. Play for 5 minutes across various rooms
3. Review frame time graph

**Expected**:
- Average frame time: ~16.67ms (60 FPS)
- Max frame time: < 33.33ms (30 FPS minimum)
- Variance: < 16.67ms (constitutional requirement)

---

## Automated Test Validation

After manual testing, run automated test suite:

```bash
# Unit tests
cargo test

# Integration tests
cargo test --test integration

# Benchmark tests
cargo bench
```

**Expected**:
- All tests pass
- Test suite completes in < 30 seconds (Constitutional Principle II)
- Benchmark results show 60 FPS average for lighting system

---

## Acceptance Checklist

Mark each item as complete after manual validation:

- [ ] Candle lighting system works (wax depletion, match usage, visibility)
- [ ] Player movement and jump mechanics responsive (< 16ms input lag)
- [ ] Double jump unlocks via powerup item
- [ ] Inventory management functional (pickup, display, stackable/unique items)
- [ ] Traps cause instant death, checkpoint respawn preserves state
- [ ] Locked doors unlock with correct keys
- [ ] Room transitions load in < 1 second
- [ ] Auto-save triggers on room entry
- [ ] Map reveals explored rooms progressively
- [ ] Puzzles solvable, rewards granted
- [ ] Configurable controls functional
- [ ] Colorblind mode changes palette
- [ ] Game maintains 60 FPS average, never drops below 30 FPS
- [ ] Complete game loop playable (start to exit door)
- [ ] Completion stats displayed on victory

---

## Troubleshooting

**Issue**: FPS drops below 30
- **Solution**: Profile with `cargo flamegraph`, optimize hot paths

**Issue**: Room transition > 1 second
- **Solution**: Check asset loading (may need async pre-loading)

**Issue**: Candle wax not depleting
- **Solution**: Verify CandleBurnSystem running in correct stage, GameState not paused

**Issue**: Map not updating
- **Solution**: Verify RoomTransitionSystem emits events, MapState mutated correctly

---

**Status**: Quickstart complete. Use this guide to validate implementation against acceptance criteria.
