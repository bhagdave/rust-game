//! Global game state and configuration resources.
//!
//! Resources are singleton data structures accessible from any system.
//! This module contains the core game state, input configuration,
//! map tracking, and asset management.

/// Asset handle management for sprites, audio, and fonts
pub mod asset_handles;

/// Global game state including current room, player status, and game mode
pub mod game_state;

/// Input action mapping and configuration
pub mod input_config;

/// Map exploration tracking and room layout data
pub mod map_state;
