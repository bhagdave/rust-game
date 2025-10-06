//! # Rust Game - House Escape Game
//!
//! A 2D platformer escape room game built with Bevy engine. Players navigate through
//! a dark house using a candle for light while solving puzzles and avoiding traps.
//!
//! ## Core Features
//!
//! - **Lighting System**: Dynamic candle-based visibility with wax depletion
//! - **Platforming**: Physics-based movement with jumping and double-jump mechanics
//! - **Puzzle Solving**: Circuit breakers, pressure plates, symbol matching, and more
//! - **Trap System**: Environmental hazards and deadly traps to avoid
//! - **Room Navigation**: Multi-room house with locked doors and secret passages
//! - **Inventory Management**: Collect and use items like keys, tools, and puzzle pieces
//!
//! ## Module Organization
//!
//! - [`components`] - ECS components for game entities (player, candles, traps, puzzles)
//! - [`resources`] - Global game state and configuration resources
//! - [`systems`] - Game logic systems for movement, collision, puzzles, etc.
//! - [`audio`] - Sound event handling and audio playback
//! - [`ui`] - HUD and user interface components
//! - [`entities`] - Entity spawning and initialization (currently empty)
//!
//! ## Technology Stack
//!
//! - Bevy 0.16.1 - ECS game engine
//! - bevy_ecs_tilemap - 2D tilemap rendering
//! - bevy_kira_audio - Audio playback
//! - bevy_egui - UI rendering
//! - leafwing_input_manager - Input handling
//! - serde - Save/load serialization

/// Audio system components for event-based sound playback
pub mod audio;

/// ECS components for game entities
pub mod components;

/// Entity spawning and initialization utilities
pub mod entities;

/// Global game state and configuration resources
pub mod resources;

/// Game logic systems for movement, physics, puzzles, and more
pub mod systems;

/// UI components for HUD and menus
pub mod ui;
