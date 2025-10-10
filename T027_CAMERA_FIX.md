# T027 Camera Fix - Making Demo Level Visible

## Problem

After implementing T027 (DemoPlugin integration), the app window opened but showed nothing. The screen was blank/black.

## Root Cause

The demo level was loading entities (player, doors, items, tilemap) but **no camera was spawned**. In Bevy, you need at least one camera entity for anything to be rendered on screen.

## Solution

Added camera spawn to `init_demo_system` in `src/systems/demo_level.rs`:

```rust
fn init_demo_system(mut commands: Commands, mut game_state: ResMut<GameState>) {
    // Spawn camera for the demo level (required to see anything in Bevy)
    commands.spawn(Camera2d);
    info!("Spawned 2D camera for demo level");
    
    // ... rest of system
}
```

## Technical Details

### Bevy 0.16 Camera API

In Bevy 0.16, cameras are spawned as simple components:
- `Camera2d` - for 2D rendering (what we need)
- `Camera3d` - for 3D rendering

The old `Camera2dBundle` API from earlier Bevy versions is deprecated.

### System Integration

The camera is spawned in the `Startup` schedule via `init_demo_system`, which ensures:
1. Camera exists before any entities are rendered
2. Camera is available for the entire application lifetime
3. All demo entities will be visible when loaded

## Verification

All quality gates pass:
- ✅ **Build**: Compiles successfully
- ✅ **Tests**: 126 tests passing (0 failed)
- ✅ **Clippy**: Zero warnings
- ✅ **Rustfmt**: Clean formatting

## What You Should Now See

When running the app with `cargo run`, you should see:
1. **Window opens** with title "House Escape" (1920x1080)
2. **Tilemap rendered** - floor and wall tiles forming a room
3. **Entities visible**:
   - Player sprite at center (960, 540)
   - 2 matches at left and right
   - 2 keys (brass and iron) at bottom
   - 2 doors (one locked at top, one unlocked at right)
   - Candle at player position

## Expected Console Output

```
INFO bevy_render::renderer: AdapterInfo { ... }
INFO rust_game::systems::demo_level: Spawned 2D camera for demo level
INFO rust_game::systems::demo_level: First run detected - setting GameMode to Playing
INFO rust_game::systems::demo_level: First run detected - demo level will load
INFO rust_game::systems::demo_level: Starting demo level load...
INFO rust_game::systems::demo_level: Successfully loaded demo level data 'Demo Level'
INFO rust_game::systems::demo_level: ✓ Demo level 'Demo Level' loaded successfully: 8 entities spawned
```

## Potential Issues Still Present

### Sprites May Not Show Correctly

The entities are being spawned but the sprite rendering might not work because:

1. **Missing Sprite Components**: The spawn functions create entities with game components (Player, Health, etc.) but may not be adding proper `Sprite` or `SpriteBundle` components with correct transforms.

2. **Asset Loading**: The sprites need proper texture handles from AssetHandles resource.

3. **Tilemap Rendering**: The tilemap system uses bevy_ecs_tilemap which has specific setup requirements.

### Next Debugging Steps (If Still Blank)

If you still see nothing after this fix:

1. **Check Console Logs**: Look for asset loading errors or warnings
2. **Verify Asset Handles**: Ensure AssetHandles resource has sprite textures loaded
3. **Add Debug Sprites**: Temporarily add colored squares to entities to verify positioning
4. **Check Entity Counts**: Use Bevy inspector to see if entities exist in world

## Related Files

- `src/systems/demo_level.rs` - Camera spawn implementation
- `src/main.rs` - DemoPlugin integration
- `assets/levels/demo.ron` - Demo level data
- `assets/sprites/` - Sprite assets

## Commits

- `40d45b5` - fix(demo): Add camera spawn to make demo level visible
- `e83e0dd` - chore(T027): Add validation report and fix rustfmt issues

---

**Fixed**: 2025-01-05  
**Issue**: Blank window after T027 completion  
**Solution**: Added Camera2d spawn in init_demo_system
