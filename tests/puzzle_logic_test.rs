use bevy::prelude::*;
use rust_game::components::inventory::*;
use rust_game::components::puzzle::*;
use rust_game::resources::game_state::*;
use rust_game::systems::puzzle::*;

/// Unit test: Symbol match puzzle validates correct sequence
/// From tasks.md T023: Test symbol match puzzle validation with correct sequence
#[test]
fn symbol_puzzle_validates_correct_sequence() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 2,
        player_spawn_point: Vec2::new(100.0, 100.0),
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: SymbolMatchPuzzle with correct input sequence
    let puzzle_entity = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![Symbol::Circle, Symbol::Triangle, Symbol::Square],
                correct_sequence: vec![Symbol::Circle, Symbol::Triangle, Symbol::Square],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::SpawnItem(Item::Key(KeyType::Master)),
        ))
        .id();

    // Assert: Puzzle starts unsolved
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::Unsolved,
            "Puzzle should start unsolved"
        );
    }

    // Act: Trigger puzzle validation
    app.world_mut().send_event(PuzzleInteractEvent {
        puzzle: puzzle_entity,
    });
    app.update();

    // Assert: PuzzleState changes to Solved
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::Solved,
            "Puzzle should be solved with correct sequence"
        );
    }
}

/// Unit test: Symbol match puzzle rejects incorrect sequence
/// From tasks.md T023: Test symbol match puzzle validation with wrong order
#[test]
fn symbol_puzzle_rejects_incorrect_sequence() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::ZERO,
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: SymbolMatchPuzzle with wrong input sequence
    let puzzle_entity = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![Symbol::Circle, Symbol::Square, Symbol::Triangle],
                correct_sequence: vec![Symbol::Circle, Symbol::Triangle, Symbol::Square],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::UnlockDoor(2),
        ))
        .id();

    // Act: Trigger puzzle validation
    app.world_mut().send_event(PuzzleInteractEvent {
        puzzle: puzzle_entity,
    });
    app.update();

    // Assert: PuzzleState remains Unsolved
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::Unsolved,
            "Puzzle should remain unsolved with wrong sequence"
        );
    }
}

/// Unit test: Symbol puzzle tracks partial progress
/// Validates InProgress state when sequence partially entered
#[test]
fn symbol_puzzle_tracks_partial_progress() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::ZERO,
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: Puzzle with partial input (2 out of 4 symbols)
    let puzzle_entity = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![Symbol::Circle, Symbol::Triangle],
                correct_sequence: vec![
                    Symbol::Circle,
                    Symbol::Triangle,
                    Symbol::Square,
                    Symbol::Star,
                ],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::UnlockDoor(3),
        ))
        .id();

    // Act: Trigger validation with partial sequence
    app.world_mut().send_event(PuzzleInteractEvent {
        puzzle: puzzle_entity,
    });
    app.update();

    // Assert: Transitions to InProgress (partial but correct sequence)
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::InProgress,
            "Puzzle should be InProgress with partial correct sequence"
        );
    }
}

/// Unit test: Symbol puzzle resets on wrong symbol
/// Validates that entering wrong symbol mid-sequence resets progress
#[test]
fn symbol_puzzle_resets_on_wrong_symbol() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::ZERO,
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: Puzzle with wrong symbol mid-sequence
    let puzzle_entity = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![Symbol::Circle, Symbol::Star],
                correct_sequence: vec![Symbol::Circle, Symbol::Triangle, Symbol::Square],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::UnlockDoor(2),
        ))
        .id();

    // Act: Trigger validation
    app.world_mut().send_event(PuzzleInteractEvent {
        puzzle: puzzle_entity,
    });
    app.update();

    // Assert: Remains Unsolved (wrong symbol in sequence)
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::Unsolved,
            "Puzzle should remain unsolved with wrong symbol"
        );
    }
}

/// Unit test: Empty correct sequence edge case
/// Validates puzzle behavior with no required symbols (auto-solved?)
#[test]
fn symbol_puzzle_handles_empty_sequence() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::ZERO,
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: Puzzle with empty sequences (edge case - auto-solves)
    let puzzle_entity = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![],
                correct_sequence: vec![],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::UnlockDoor(1),
        ))
        .id();

    // Act: Trigger validation
    app.world_mut().send_event(PuzzleInteractEvent {
        puzzle: puzzle_entity,
    });
    app.update();

    // Assert: Empty sequences match - puzzle solves
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::Solved,
            "Empty sequence puzzle should auto-solve (empty == empty)"
        );
    }
}

/// Unit test: Single symbol sequence
/// Validates simplest case (1 symbol required)
#[test]
fn symbol_puzzle_single_symbol_sequence() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::ZERO,
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: Puzzle with single symbol
    let puzzle_entity = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![Symbol::Circle],
                correct_sequence: vec![Symbol::Circle],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::SpawnItem(Item::DoubleJumpItem),
        ))
        .id();

    // Act: Trigger validation
    app.world_mut().send_event(PuzzleInteractEvent {
        puzzle: puzzle_entity,
    });
    app.update();

    // Assert: Puzzle solved
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::Solved,
            "Single symbol puzzle should solve"
        );
    }
}

/// Unit test: Long symbol sequence
/// Validates complex puzzle with many symbols (6+)
#[test]
fn symbol_puzzle_long_sequence() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::ZERO,
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: Puzzle with 6 symbols matching
    let puzzle_entity = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![
                    Symbol::Circle,
                    Symbol::Triangle,
                    Symbol::Square,
                    Symbol::Star,
                    Symbol::Circle,
                    Symbol::Triangle,
                ],
                correct_sequence: vec![
                    Symbol::Circle,
                    Symbol::Triangle,
                    Symbol::Square,
                    Symbol::Star,
                    Symbol::Circle,
                    Symbol::Triangle,
                ],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::SpawnItem(Item::Key(KeyType::Brass)),
        ))
        .id();

    // Act: Trigger validation
    app.world_mut().send_event(PuzzleInteractEvent {
        puzzle: puzzle_entity,
    });
    app.update();

    // Assert: Long sequence puzzle solved
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::Solved,
            "Long sequence should solve when all correct"
        );
    }
}

/// Unit test: Multiple puzzle instances
/// Validates that different symbol puzzles operate independently
#[test]
fn multiple_symbol_puzzles_independent() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::ZERO,
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: Two puzzles - one solved, one unsolved
    let puzzle1 = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![Symbol::Circle, Symbol::Triangle],
                correct_sequence: vec![Symbol::Circle, Symbol::Triangle],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::UnlockDoor(2),
        ))
        .id();

    let puzzle2 = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![],
                correct_sequence: vec![Symbol::Star, Symbol::Square],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::UnlockDoor(3),
        ))
        .id();

    // Act: Solve only puzzle1
    app.world_mut()
        .send_event(PuzzleInteractEvent { puzzle: puzzle1 });
    app.update();

    // Assert: puzzle1 solved, puzzle2 still unsolved
    {
        let state1 = app.world().get::<PuzzleState>(puzzle1).unwrap();
        let state2 = app.world().get::<PuzzleState>(puzzle2).unwrap();
        assert_eq!(*state1, PuzzleState::Solved, "Puzzle 1 should be solved");
        assert_eq!(
            *state2,
            PuzzleState::Unsolved,
            "Puzzle 2 should remain unsolved"
        );
    }
}

/// Unit test: All symbol types in sequence
/// Validates that all 4 symbol types (Circle, Triangle, Square, Star) work correctly
#[test]
fn symbol_puzzle_all_symbol_types() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    app.add_event::<PuzzleInteractEvent>();
    app.add_event::<PuzzleSolvedEvent>();
    app.add_systems(Update, puzzle_interaction_system);

    app.insert_resource(GameState {
        current_room: 0,
        player_spawn_point: Vec2::ZERO,
        completion_time: std::time::Duration::ZERO,
        collected_secrets: std::collections::HashSet::new(),
        game_mode: GameMode::Playing,
        deaths: 0,
    });

    // Setup: Puzzle using all 4 symbol types
    let puzzle_entity = app
        .world_mut()
        .spawn((
            Puzzle::SymbolMatch(SymbolMatchPuzzle {
                input_sequence: vec![
                    Symbol::Circle,
                    Symbol::Triangle,
                    Symbol::Square,
                    Symbol::Star,
                ],
                correct_sequence: vec![
                    Symbol::Circle,
                    Symbol::Triangle,
                    Symbol::Square,
                    Symbol::Star,
                ],
            }),
            PuzzleState::Unsolved,
            PuzzleReward::RevealPassage(4),
        ))
        .id();

    // Act: Trigger validation
    app.world_mut().send_event(PuzzleInteractEvent {
        puzzle: puzzle_entity,
    });
    app.update();

    // Assert: Puzzle solved with all symbol types
    {
        let state = app.world().get::<PuzzleState>(puzzle_entity).unwrap();
        assert_eq!(
            *state,
            PuzzleState::Solved,
            "All symbol types should be validated"
        );
    }
}
