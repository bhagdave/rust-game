//! Game logic systems for movement, physics, puzzles, and more.
//!
//! Systems contain the core game logic that operates on components and resources.
//! Each module focuses on a specific aspect of gameplay.

/// Candle wax depletion and state management system
pub mod candle_burn;

/// Collision detection and physics system
pub mod collision;

/// Demo level loading and management system
pub mod demo_level;

/// Fixed timestep scheduling for deterministic physics
pub mod fixed_timestep;

/// Inventory management and item collection systems
pub mod inventory;

/// Level loading from RON files
pub mod level_loader;

/// Dynamic lighting and visibility systems
pub mod lighting;

/// Player movement, jumping, and physics
pub mod player_movement;

/// Puzzle interaction and solving systems
pub mod puzzle;

/// Player death and respawn system
pub mod respawn;

/// Room transition and door interaction systems
pub mod room_transition;

/// Save and load game state systems
pub mod save_load;

/// Tilemap rendering and management
pub mod tilemap;

/// Trap triggering and hazard systems
pub mod trap;
