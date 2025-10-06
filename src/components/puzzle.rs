use crate::components::inventory::Item;
use crate::components::room::RoomId;
use bevy::prelude::*;

/// Component defining the type and data for a puzzle.
///
/// Puzzles block progression until solved. Each variant contains
/// puzzle-specific data needed for validation and interaction.
#[derive(Component)]
pub enum Puzzle {
    /// Electrical circuit puzzle requiring fuses in correct order
    CircuitBreaker(CircuitBreakerPuzzle),
    /// Weight-based puzzle requiring items on plates
    PressurePlate(PressurePlatePuzzle),
    /// Pattern matching puzzle with symbols
    SymbolMatch(SymbolMatchPuzzle),
    /// Light reflection puzzle (no additional data needed)
    MirrorReflection,
    /// Lever combination puzzle requiring correct switch positions
    LeverCombination(LeverCombinationPuzzle),
}

/// Component tracking the current state of a puzzle.
///
/// State transitions:
/// - `Unsolved` -> `InProgress` (when player starts interacting)
/// - `InProgress` -> `Solved` (when correct solution achieved)
/// - `InProgress` -> `Unsolved` (when player resets or gives up)
#[derive(Component, Debug, PartialEq)]
pub enum PuzzleState {
    /// Puzzle has not been attempted
    Unsolved,
    /// Player is currently working on puzzle
    InProgress,
    /// Puzzle has been successfully completed
    Solved,
}

/// Component defining what reward is granted when puzzle is solved.
///
/// Rewards are applied immediately upon puzzle completion.
#[derive(Component, Clone)]
pub enum PuzzleReward {
    /// Unlocks a door to specified room
    UnlockDoor(RoomId),
    /// Reveals a hidden passage to specified room
    RevealPassage(RoomId),
    /// Spawns an item for the player to collect
    SpawnItem(Item),
}

/// Data for circuit breaker puzzle requiring fuses in correct sequence.
///
/// Player must insert fuses (entities) into slots in the correct order
/// specified by `correct_sequence`.
pub struct CircuitBreakerPuzzle {
    /// Fuse slots that can contain fuse entities (None = empty slot)
    pub fuse_slots: Vec<Option<Entity>>,
    /// Indices representing correct fuse placement order
    pub correct_sequence: Vec<usize>,
}

/// Data for pressure plate puzzle requiring weighted items.
///
/// Player must place specific items on plates to activate the mechanism.
pub struct PressurePlatePuzzle {
    /// Pressure plate entities in the puzzle
    pub plates: Vec<Entity>,
    /// Item entities that must be placed on plates
    pub required_items: Vec<Entity>,
}

/// Symbol types used in symbol matching puzzles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symbol {
    /// Circle symbol
    Circle,
    /// Triangle symbol
    Triangle,
    /// Square symbol
    Square,
    /// Star symbol
    Star,
}

/// Data for symbol matching puzzle requiring pattern recreation.
///
/// Player must input the correct sequence of symbols to solve.
pub struct SymbolMatchPuzzle {
    /// Symbols player has input so far
    pub input_sequence: Vec<Symbol>,
    /// Required symbol sequence to solve puzzle
    pub correct_sequence: Vec<Symbol>,
}

/// State of a lever in lever combination puzzles.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeverState {
    /// Lever is in up position
    Up,
    /// Lever is in down position
    Down,
}

/// Data for lever combination puzzle requiring correct lever positions.
///
/// Player must set all levers to their correct up/down states.
pub struct LeverCombinationPuzzle {
    /// Lever entities in the puzzle
    pub levers: Vec<Entity>,
    /// Required states for each lever to solve puzzle
    pub correct_states: Vec<LeverState>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::inventory::{Item, KeyType};

    #[test]
    fn can_create_puzzle_entity() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);

        // Create a symbol match puzzle
        let puzzle = Puzzle::SymbolMatch(SymbolMatchPuzzle {
            input_sequence: vec![],
            correct_sequence: vec![Symbol::Circle, Symbol::Triangle, Symbol::Square],
        });

        let entity = app
            .world_mut()
            .spawn((puzzle, PuzzleState::Unsolved, PuzzleReward::UnlockDoor(1)))
            .id();

        // Verify puzzle components
        let puzzle_component = app.world().get::<Puzzle>(entity);
        assert!(puzzle_component.is_some());

        let state = app.world().get::<PuzzleState>(entity);
        assert!(state.is_some());
        assert_eq!(*state.unwrap(), PuzzleState::Unsolved);

        let reward = app.world().get::<PuzzleReward>(entity);
        assert!(reward.is_some());
    }

    #[test]
    fn puzzle_types_instantiable() {
        // Circuit breaker puzzle
        let _circuit = Puzzle::CircuitBreaker(CircuitBreakerPuzzle {
            fuse_slots: vec![None, None, None],
            correct_sequence: vec![0, 2, 1],
        });

        // Pressure plate puzzle
        let _pressure = Puzzle::PressurePlate(PressurePlatePuzzle {
            plates: vec![],
            required_items: vec![],
        });

        // Symbol match puzzle
        let _symbol = Puzzle::SymbolMatch(SymbolMatchPuzzle {
            input_sequence: vec![],
            correct_sequence: vec![Symbol::Circle, Symbol::Star],
        });

        // Mirror reflection puzzle (no data)
        let _mirror = Puzzle::MirrorReflection;

        // Lever combination puzzle
        let _lever = Puzzle::LeverCombination(LeverCombinationPuzzle {
            levers: vec![],
            correct_states: vec![LeverState::Up, LeverState::Down, LeverState::Up],
        });
    }

    #[test]
    fn puzzle_state_transitions() {
        assert_eq!(PuzzleState::Unsolved, PuzzleState::Unsolved);
        assert_ne!(PuzzleState::Unsolved, PuzzleState::InProgress);
        assert_ne!(PuzzleState::InProgress, PuzzleState::Solved);
    }

    #[test]
    fn puzzle_rewards_definable() {
        // Door unlock reward
        let _unlock = PuzzleReward::UnlockDoor(5);

        // Passage reveal reward
        let _passage = PuzzleReward::RevealPassage(3);

        // Item spawn reward
        let _item = PuzzleReward::SpawnItem(Item::Key(KeyType::Brass));
    }

    #[test]
    fn symbol_enum_values() {
        let symbols = [
            Symbol::Circle,
            Symbol::Triangle,
            Symbol::Square,
            Symbol::Star,
        ];

        assert_eq!(symbols.len(), 4);
        assert_eq!(symbols[0], Symbol::Circle);
        assert_ne!(symbols[0], symbols[1]);
    }

    #[test]
    fn lever_state_values() {
        assert_eq!(LeverState::Up, LeverState::Up);
        assert_eq!(LeverState::Down, LeverState::Down);
        assert_ne!(LeverState::Up, LeverState::Down);
    }

    #[test]
    fn circuit_breaker_puzzle_structure() {
        let puzzle = CircuitBreakerPuzzle {
            fuse_slots: vec![None, None, None, None],
            correct_sequence: vec![0, 2, 1, 3],
        };

        assert_eq!(puzzle.fuse_slots.len(), 4);
        assert_eq!(puzzle.correct_sequence.len(), 4);
        assert_eq!(puzzle.correct_sequence[0], 0);
    }

    #[test]
    fn symbol_match_puzzle_structure() {
        let puzzle = SymbolMatchPuzzle {
            input_sequence: vec![Symbol::Circle],
            correct_sequence: vec![Symbol::Circle, Symbol::Triangle, Symbol::Square],
        };

        assert_eq!(puzzle.input_sequence.len(), 1);
        assert_eq!(puzzle.correct_sequence.len(), 3);
        assert_eq!(puzzle.correct_sequence[0], Symbol::Circle);
    }

    #[test]
    fn lever_combination_puzzle_structure() {
        let puzzle = LeverCombinationPuzzle {
            levers: vec![],
            correct_states: vec![
                LeverState::Up,
                LeverState::Up,
                LeverState::Down,
                LeverState::Up,
            ],
        };

        assert_eq!(puzzle.correct_states.len(), 4);
        assert_eq!(puzzle.correct_states[0], LeverState::Up);
        assert_eq!(puzzle.correct_states[2], LeverState::Down);
    }

    #[test]
    fn puzzle_reward_cloning() {
        let reward = PuzzleReward::UnlockDoor(5);
        let cloned = reward.clone();

        match (reward, cloned) {
            (PuzzleReward::UnlockDoor(id1), PuzzleReward::UnlockDoor(id2)) => {
                assert_eq!(id1, id2);
            }
            _ => panic!("Clone should preserve variant"),
        }
    }
}
