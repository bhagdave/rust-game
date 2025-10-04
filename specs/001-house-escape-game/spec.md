# Feature Specification: House Escape Game

**Feature Branch**: `001-house-escape-game`
**Created**: 2025-10-03
**Status**: Draft
**Input**: User description: "2D atmospheric platformer - candle-based lighting, puzzle solving, trap navigation"

---

## Clarifications

### Session 2025-10-03
- Q: How should traps affect the player? → A: Instant death - all traps kill player immediately, respawn at room entrance
- Q: What jump mechanic should the game use? → A: Context-sensitive - single jump normally, double jump unlocked via specific item/powerup
- Q: How should game progress be saved? → A: Auto-save on room entry
- Q: Should the game include a map system? → A: Auto-revealing map that gradually fills in as player explores rooms
- Q: Which accessibility features should be included? → A: Configurable controls + colorblind-friendly palette options

---

## User Scenarios & Testing

### Primary User Story

A player starts in the entry hall of a dark Victorian mansion holding a lit candle. They explore room by room, managing their limited candle wax and matches while solving puzzles to unlock new areas. The player must navigate platforming challenges, avoid deadly traps, collect key items, and ultimately find and unlock the main exit door to escape the house.

### Acceptance Scenarios

1. **Given** player is in a dark room with an unlit candle, **When** player uses a match, **Then** candle lights and visibility radius expands from 1-2 tiles to 6-8 tiles
2. **Given** player has a lit candle, **When** candle wax depletes completely, **Then** candle extinguishes and visibility returns to minimal radius
3. **Given** player encounters a locked door, **When** player has collected the correct key, **Then** player can unlock and pass through the door
4. **Given** player finds a pressure plate puzzle, **When** player places correct item combination on plates, **Then** locked door or passage opens
5. **Given** player is navigating platforming sections, **When** player falls into spike pit, **Then** player dies and respawns at room entrance with inventory intact
6. **Given** player has collected all required items and solved final puzzle, **When** player reaches exit door, **Then** game completes with success metrics displayed

### Edge Cases

- What happens when player extinguishes candle manually in safe area? (Conserves wax, visibility reduced)
- How does system handle player running out of matches? (Must explore carefully in darkness to find more matches)
- What happens when environmental hazard (wind, water) extinguishes candle? (Candle goes out, must use match to relight)
- How does system handle player attempting to use item in wrong location? (No effect, item remains in inventory)
- What happens when player dies during timed trap sequence? (Room resets to safe state, player respawns at entrance)

## Requirements

### Functional Requirements

**Core Gameplay:**
- **FR-001**: Game MUST provide 2D side-scrolling platformer movement (left/right, jump, climb)
- **FR-002**: Player MUST be able to carry and light a candle that provides circular illumination
- **FR-003**: Candle MUST deplete wax over time when lit at measurable rate
- **FR-004**: Player MUST be able to use matches to relight extinguished candle
- **FR-005**: Player MUST be able to manually extinguish lit candle to conserve wax
- **FR-006**: System MUST support context-sensitive jump mechanic (single jump normally, double jump unlocked via specific item/powerup)

**Lighting & Visibility:**
- **FR-007**: Game MUST render different visibility radii based on candle state (unlit: 1-2 tiles, lit: 6-8 tiles)
- **FR-008**: Environmental hazards (wind, water) MUST extinguish lit candles
- **FR-009**: Fog of war MUST obscure unexplored/distant areas beyond visibility radius
- **FR-010**: Subtle ambient objects (windows, doorways) MUST remain faintly visible in darkness

**Inventory & Items:**
- **FR-011**: Player MUST be able to carry multiple items simultaneously
- **FR-012**: Inventory MUST support stackable items (matches) with quantity display
- **FR-013**: Inventory MUST support unique items (keys, tools, puzzle items)
- **FR-014**: System MUST display inventory as icon bar with quantity indicators
- **FR-015**: Player MUST be able to interact with objects to use/apply inventory items

**Level Design:**
- **FR-016**: Game MUST provide 20-30 interconnected rooms across 4 floors (Ground, First, Second, Basement)
- **FR-017**: Rooms MUST be connected via doors, hallways, staircases, and ladders
- **FR-018**: Some doors MUST be locked and require specific keys to unlock
- **FR-019**: Hidden passages MUST be revealed through puzzle completion
- **FR-020**: System MUST provide auto-revealing map that gradually fills in as player explores rooms

**Traps & Hazards:**
- **FR-021**: Game MUST include environmental traps (spike pits, falling chandeliers, collapsing floors, pendulums, arrow traps)
- **FR-022**: Game MUST include environmental hazards (drafty windows, water puddles, broken floors, fan blades, steam vents)
- **FR-023**: All traps MUST cause instant death to player
- **FR-024**: Trap activation MUST be triggered by player actions (pressure plates, proximity, timed sequences)

**Puzzle Systems:**
- **FR-025**: Game MUST support circuit breaker puzzles (collect fuses, restore power, unlock electronic doors)
- **FR-026**: Game MUST support weight pressure plate puzzles requiring item placement
- **FR-027**: Game MUST support symbol matching puzzles (keypads, dial locks)
- **FR-028**: Game MUST support mirror reflection puzzles (angle mirrors to redirect light beams)
- **FR-029**: Game MUST support lever combination puzzles with environmental clues
- **FR-030**: Tools MUST enable specific progression (wrench for grates, crowbar for boards, wire cutters for electric traps, etc.)

**Progression & Completion:**
- **FR-031**: Game MUST gate progression through item collection (metroidvania-style)
- **FR-032**: Early game MUST limit player to ground floor as tutorial area
- **FR-033**: Mid game MUST unlock upper floors with increased puzzle complexity
- **FR-034**: Late game MUST unlock basement with final puzzle sequence
- **FR-035**: Exit door MUST require specific key items and puzzle completion to unlock
- **FR-036**: System MUST track completion time and collection percentage (matches, secrets, diary pages)

**Death & Respawn:**
- **FR-037**: System MUST implement checkpoint system with respawn at room entrance
- **FR-038**: On death, player MUST respawn at room entrance
- **FR-039**: On death, player MUST retain collected items and inventory
- **FR-040**: On death, room state (traps) MUST reset to initial configuration
- **FR-041**: On death, candle state MUST be preserved

**Secondary Objectives:**
- **FR-042**: Game MUST hide collectible diary pages throughout house revealing backstory
- **FR-043**: Game MUST include secret rooms with bonus items
- **FR-044**: Game MUST include optional challenge rooms
- **FR-045**: System MUST display collection percentage on completion

### Key Entities

- **Player Character**: Main actor, carries candle, navigates house, manages inventory, dies/respawns
- **Candle**: Light source with wax meter, can be lit/unlit/burning states, affected by environment
- **Match**: Consumable stackable item, relights candle to full wax
- **Keys**: Unique items matching specific locked doors
- **Tools**: Unique items enabling access to areas (wrench, crowbar, wire cutters, magnet, oil can, ladder)
- **Double Jump Item**: Powerup that unlocks mid-air jump capability when collected
- **Puzzle Items**: Unique items for specific puzzles (fuses, gemstones, circuit components)
- **Map**: Auto-revealing navigation aid that tracks explored rooms and fills in gradually
- **Rooms**: Distinct areas with connections (doors, stairs, ladders), contain items/traps/puzzles
- **Doors**: Barriers between rooms, can be locked/unlocked, require specific keys
- **Traps**: Hazardous obstacles causing instant death (spikes, chandeliers, floors, pendulums, arrows)
- **Environmental Hazards**: Non-lethal obstacles affecting candle or movement (wind, water, broken floors, fans, steam)
- **Puzzles**: Multi-step challenges requiring item placement, lever sequences, or light redirection
- **Diary Pages**: Collectible lore items revealing backstory
- **Exit Door**: Final objective requiring key items and puzzle completion

### Performance & UX Requirements

- **Response Time**: Input lag <16ms for responsive platforming controls
- **Frame Rate**: Maintain 60 FPS on target hardware, never drop below 30 FPS during gameplay
- **Loading Time**: Room transitions <1s, game startup <5s
- **Accessibility**: Configurable controls and colorblind-friendly palette options
- **Visual Clarity**: Lighting gradients must clearly distinguish safe/hazardous areas despite darkness theme
- **Save System**: Auto-save on room entry

---

## Review & Acceptance Checklist

### Content Quality
- [x] No implementation details (languages, frameworks, APIs)
- [x] Focused on user value and business needs
- [x] Written for non-technical stakeholders
- [x] All mandatory sections completed

### Requirement Completeness
- [x] No [NEEDS CLARIFICATION] markers remain (5 resolved, 2 deferred to future scope)
- [x] Requirements are testable and unambiguous
- [x] Success criteria are measurable
- [x] Scope is clearly bounded
- [x] Dependencies and assumptions identified

---

## Execution Status

- [x] User description parsed
- [x] Key concepts extracted
- [x] Ambiguities marked and resolved (5 critical clarifications completed)
- [x] User scenarios defined
- [x] Requirements generated (45 functional requirements)
- [x] Entities identified (14 key entities)
- [x] Review checklist passed

---

## Notes for Planning Phase

**Deferred Design Decisions** (optional future scope):
- Enemy/creature encounters: Current scope is purely environmental challenges; NPC encounters deferred
- Advanced difficulty modes: Permadeath/lives system deferred in favor of checkpoint system
- Extended accessibility: Text scaling and advanced visual assist modes deferred; MVP includes configurable controls and colorblind palettes

**Replayability Features** (optional scope expansion):
- Speedrun timer mode with leaderboard
- Item randomization for replay variety
- Difficulty levels affecting match scarcity and trap frequency
- Multiple endings based on secrets discovered
- Challenge mode with limited matches/candle wax

**Reference Inspirations:**
- Jet Set Willy (structure and exploration)
- Spelunky (lighting mechanics)
- La-Mulana (puzzle complexity)
- Environmental Station Alpha (metroidvania gating)
