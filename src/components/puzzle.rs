use bevy::prelude::*;
use crate::components::inventory::Item;
use crate::components::room::RoomId;

#[derive(Component)]
pub enum Puzzle {
    CircuitBreaker(CircuitBreakerPuzzle),
    PressurePlate(PressurePlatePuzzle),
    SymbolMatch(SymbolMatchPuzzle),
    MirrorReflection,
    LeverCombination(LeverCombinationPuzzle),
}

#[derive(Component, Debug, PartialEq)]
pub enum PuzzleState {
    Unsolved,
    InProgress,
    Solved,
}

#[derive(Component, Clone)]
pub enum PuzzleReward {
    UnlockDoor(RoomId),
    RevealPassage(RoomId),
    SpawnItem(Item),
}

pub struct CircuitBreakerPuzzle {
    pub fuse_slots: Vec<Option<Entity>>,
    pub correct_sequence: Vec<usize>,
}

pub struct PressurePlatePuzzle {
    pub plates: Vec<Entity>,
    pub required_items: Vec<Entity>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Symbol {
    Circle,
    Triangle,
    Square,
    Star,
}

pub struct SymbolMatchPuzzle {
    pub input_sequence: Vec<Symbol>,
    pub correct_sequence: Vec<Symbol>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LeverState {
    Up,
    Down,
}

pub struct LeverCombinationPuzzle {
    pub levers: Vec<Entity>,
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
            .spawn((
                puzzle,
                PuzzleState::Unsolved,
                PuzzleReward::UnlockDoor(1),
            ))
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
        let symbols = vec![
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
