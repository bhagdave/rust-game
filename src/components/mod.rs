//! ECS components for game entities.
//!
//! This module contains all the components used to define entity behavior
//! in the house escape game. Components are organized by functionality.

/// Demo level components for testing and validation
pub mod demo;

/// Inventory management components for items and player storage
pub mod inventory;

/// Lighting and candle components for visibility mechanics
pub mod lighting;

/// Player character components for movement and state
pub mod player;

/// Puzzle components for circuit breakers, levers, and symbol matching
pub mod puzzle;

/// Room and door components for level navigation
pub mod room;

/// Trap and environmental hazard components
pub mod trap;
