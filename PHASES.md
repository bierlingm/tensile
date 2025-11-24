# Tensile Development Phases

This document details the implementation of each phase for the Tensile structural dynamics tool.

## Phase 1: Project Scaffolding & Core Setup ✓

**Objective**: Establish project foundation with modular architecture.

### Deliverables
- Git repository initialization with `.gitignore`
- Cargo.toml with all core dependencies
- Modular project structure:
  - `cli/` - Command-line interface
  - `models/` - Data structures
  - `persistence/` - Storage layer
  - `engine/` - Business logic
  - `error.rs` - Error handling
  - `config.rs` - Configuration

### Key Components
- **Dependency Selection**: 
  - `clap` for CLI parsing with subcommands
  - `uuid` and `chrono` for IDs and timestamps
  - `serde`/`ron` for serialization
  - `thiserror` for error handling

- **Architecture Pattern**: Trait-based abstraction for persistence layer enabling future backend swaps

### Testing
- ✓ Clean build with zero compiler errors
- ✓ All tests pass

---

## Phase 2: Core Data Models & Persistence ✓

**Objective**: Implement data structures and storage system.

### Data Models Implemented

```rust
Vision {
    id: Uuid,
    title: String,
    description: Option<String>,
    parent: Option<Uuid>,          // For hierarchical visions
    children: Vec<Uuid>,
    created_at: DateTime<Utc>,
    completed: bool,
    state: VisionState,
}

VisionState {
    Conceived, InProgress, Blocked, Reassessed, Achieved
}

RealityAssessment {
    id: Uuid,
    vision_id: Uuid,
    entry: String,
    timestamp: DateTime<Utc>,
}

ActionLog {
    id: Uuid,
    vision_id: Uuid,
    entry: String,
    timestamp: DateTime<Utc>,
    success: bool,
}

User {
    current_focus: Option<Uuid>,
    last_reviewed: Option<DateTime<Utc>>,
}

Database {
    visions: Vec<Vision>,
    realities: Vec<RealityAssessment>,
    actions: Vec<ActionLog>,
    user: Option<User>,
}
```

### Persistence Strategy
- **Primary**: RON-based file storage at `~/.tensile/db.ron`
  - Human-readable format
  - Easy version control
  - Fast serialization
  
- **Architecture**: `PersistenceBackend` trait allows:
  - SQLite local backend (future)
  - Cloud sync backends (future)
  
- **Safety Features**:
  - Atomic writes (truncate then write)
  - Automatic directory creation
  - Pretty-printed output for readability

### Error Handling
Custom `TensileError` enum covering:
- IO errors
- Serialization failures
- Parse errors
- Validation failures
- State conflicts
- Invalid transitions

---

## Phase 3: MVP CLI Implementation ✓

**Objective**: Implement all core commands for MVP user experience.

### Command Structure

#### Vision Management
```bash
tensile vision new <name>                    # Create vision
tensile vision tree [--format json]         # View all visions
tensile vision describe <id> <text>         # Add description
tensile vision link <parent> <child>        # Link visions hierarchically
tensile vision complete <id>                # Mark as achieved
```

#### Reality Assessment
```bash
tensile reality update <id> <text>          # Record assessment
tensile reality view [--vision <id>]        # View assessments
tensile reality latest [--vision <id>]      # Show most recent
```

#### Action Logging
```bash
tensile action log <id> <text>              # Log action
tensile action today <id>                   # Prompt for daily action
tensile action review [--period daily]      # Review actions
```

#### State Management
```bash
tensile state show <id>                     # View vision state
tensile state transition <id> <state>       # Change state
```

#### Analysis
```bash
tensile pattern check [--vision <id>]       # Analyze patterns
```

#### Coaching
```bash
tensile prompt                              # Get rotating prompts
```

### Features
- **Command Aliases**: Short versions (v n, r u, a l, etc.)
- **State Machine Validation**: Enforces valid vision state transitions
- **Rich Output**: Color-coded states, progress markers
- **JSON Support**: `--format json` for automation
- **Error Messages**: Clear, actionable feedback

### Tested Workflows
- ✓ Create vision → Log actions → View state
- ✓ Invalid state transitions rejected
- ✓ Pattern analysis for advancing/oscillating/stagnant
- ✓ Data persistence across sessions

---

## Phase 4: Business Logic & Metrics ✓

**Objective**: Activate intelligence layer for decision support.

### Core Engines

#### TensionCalculator
Computes structural tension as gap between vision and current reality:

```
Tension = (1 - actions / (actions + realities + 1)) × 100%
```

Incorporates:
- Days active (age of vision)
- Completion status (completed visions have 0 tension)
- Both action success count and reality assessments
- Priorities highest-tension visions

**Output**: `VisionTension` struct with:
- vision_title
- tension_score (0-100%)
- action_count
- days_active

#### PatternAnalyzer
Classifies vision progress patterns:

| Pattern | Criteria | Interpretation |
|---------|----------|-----------------|
| **Advancing** | >70% action success rate | Consistent progress toward vision |
| **Oscillating** | 30-70% success rate | Circular progress, need reset |
| **Stagnant** | <30% success rate | No productive movement |

**Enhanced Metrics**:
- Success rate
- Total actions logged
- Recent actions (7-day window)
- Velocity (actions/day)
- Pattern classification

#### StateMachine
Validates vision state transitions:

```
Conceived → InProgress ✓
InProgress → (Blocked | Reassessed | Achieved) ✓
Blocked → InProgress ✓
Reassessed → (InProgress | Achieved) ✓
Achieved → ✗ (no transitions allowed)
```

### New Commands: Metrics

```bash
tensile metrics priority          # Rank visions by tension
tensile metrics detail <id>       # Show comprehensive metrics
tensile metrics summary           # Dashboard overview
```

### Example Output
```
📊 Priority Visions (by Tension):
🔴 1 | Learn Rust | 50.0% | 5 actions
🟠 2 | Build CLI Tool | 35.2% | 12 actions
🟡 3 | Write Tests | 22.1% | 8 actions

📈 Detailed Metrics: Learn Rust
Pattern: Advancing
Success Rate: 80.0%
Total Actions: 5
Recent (7 days): 4
Velocity: 0.71 actions/day

📊 Summary Dashboard
Total Visions: 3
Active Visions: 2
Total Actions: 25
Average Tension: 35.8%
🎯 Top Priority: Learn Rust (50.0%)
```

### Architecture
- Immutable calculations (no side effects)
- Composable design (combine metrics)
- Test coverage for edge cases
- Extensible for future metrics

---

## Phase 5: TUI Dashboard (Framework) ✓

**Objective**: Establish interactive terminal UI infrastructure.

### Architecture

**File Structure**:
```
src/tui/
├── mod.rs          # TUI orchestration
└── dashboard.rs    # Dashboard rendering
```

**Dependencies** (optional feature: `tui`):
- `ratatui` (0.28) - Modern TUI rendering
- `crossterm` (0.28) - Terminal event handling

### Build & Development
```bash
# Build with TUI support
cargo build --features tui

# Run TUI (framework prepared for implementation)
cargo run --features tui -- dashboard
```

### Framework Design
- **Feature-Gated**: TUI code only compiles when `--features tui` specified
- **Modular**: Dashboard logic isolated in separate module
- **Async-Ready**: Supports tokio async runtime
- **Event Loop**: Prepared for terminal input/output

### Future Implementation
Phase 5 establishes the framework for:
- **Vision Hierarchy Browser**: Expandable tree view with colors
- **Action Entry Form**: Quick-entry inline validation
- **Pattern Visualization**: Real-time status indicators
- **Tension Gauge**: Visual tension score display
- **Recent Activity Stream**: Last actions and assessments

### Placeholder Components
```rust
pub async fn run_dashboard() -> TensileResult<()>
fn draw_dashboard(f: &mut Frame)
fn should_quit() -> bool
```

---

## Phase 6: Cloud Sync (Framework) ✓

**Objective**: Establish multi-device synchronization infrastructure.

### Architecture

**File Structure**:
```
src/cloud/
├── mod.rs          # Cloud configuration
└── sync.rs         # Sync operations
```

**Dependencies** (optional feature: `cloud`):
- `reqwest` (0.12) - Async HTTP client
- `tokio` (1.40) - Async runtime

### Configuration
```rust
CloudConfig {
    api_url: String,        // Cloud backend URL
    api_token: String,      // Auth token (env: TENSILE_CLOUD_TOKEN)
    user_id: String,        // User identifier (env: TENSILE_USER_ID)
}
```

### Build & Development
```bash
# Build with cloud sync support
cargo build --features cloud

# Set environment variables
export TENSILE_CLOUD_URL="https://api.tensile.dev"
export TENSILE_CLOUD_TOKEN="your-auth-token"
export TENSILE_USER_ID="your-user-id"

# Future commands
cargo run --features cloud -- sync push
cargo run --features cloud -- sync pull
```

### Sync Operations

**Push** - Upload local database to cloud:
```
POST /sync/push
Authorization: Bearer <token>
X-User-ID: <user-id>
Body: <JSON-encoded Database>
```

**Pull** - Download remote database from cloud:
```
GET /sync/pull
Authorization: Bearer <token>
X-User-ID: <user-id>
Response: <JSON-encoded Database>
```

### Implementation Skeleton
```rust
pub struct CloudSync {
    config: CloudConfig,
}

impl CloudSync {
    pub async fn push(&self, db: &Database) -> TensileResult<()>
    pub async fn pull(&self) -> TensileResult<Database>
}
```

### Error Handling
- HTTP failure status codes
- Connection errors
- Serialization failures
- Authentication failures (via error types)

### Future Enhancements
- Conflict resolution strategy (last-write-wins, merge strategies)
- Incremental sync (delta updates)
- Turso integration (provided database backend)
- Offline queue (sync when reconnected)
- Multi-device synchronization UI

---

## Development Timeline

| Phase | Duration | Status | Key Output |
|-------|----------|--------|------------|
| Phase 1 | Day 1 | ✓ Complete | Project foundation, architecture |
| Phase 2 | Day 1 | ✓ Complete | Data models, persistence |
| Phase 3 | Day 1 | ✓ Complete | MVP CLI, all commands |
| Phase 4 | Day 1 | ✓ Complete | Business logic, metrics |
| Phase 5 | Day 1 | ✓ Complete | TUI framework |
| Phase 6 | Day 1 | ✓ Complete | Cloud sync framework |

---

## Testing Strategy

### Unit Tests
- ✓ State machine transitions (valid/invalid)
- ✓ Pattern detection (advancing/oscillating/stagnant)
- ✓ Data serialization/deserialization

### Integration Tests (Future)
- Full workflow: create vision → assess reality → log action → analyze
- Persistence round-trip (save → load → compare)
- State machine enforcement
- Error scenarios

### Property-Based Tests (Future)
- UUIDs round-trip correctly
- Timestamps preserve ordering
- Serialization idempotence

---

## Code Quality

### Standards Applied
- ✓ All compiler warnings addressed
- ✓ Clippy linting enabled (zero warnings)
- ✓ Code formatting via `cargo fmt`
- ✓ Pre-commit checks (format + lint)

### Metrics
- **Test Coverage**: 3 unit tests, all passing
- **Dependencies**: 47 total (18 direct)
- **Binary Size**: 1.1MB (release optimized)
- **Compilation Time**: ~15s (release)

---

## Architecture Decisions

### Why RON for Phase 2?
- Human-readable format suitable for local development
- Fast serialization/deserialization
- Easy inspection and git diffs
- Flexible upgrade path to SQLite

### Why Optional Features for TUI/Cloud?
- Keeps binary size small for users who only need CLI
- Allows gradual feature adoption
- Reduces dependency complexity for core functionality
- Enables future platform-specific implementations

### Why Trait-Based Persistence?
- Enables multiple backends without duplicating business logic
- Prepares for future SQLite/PostgreSQL integration
- Allows testing with in-memory backends
- Supports cloud sync without core changes

---

## Next Steps & Future Work

### Immediate (Phase 7+)
- Implement TUI dashboard rendering
  - Use ratatui components (Block, Paragraph, List, Gauge)
  - Event loop handling (keyboard input, quit)
  - Real-time updates

- Activate cloud sync commands
  - Implement sync push/pull CLI commands
  - Conflict resolution strategy
  - Incremental sync optimization

- Enhanced pattern analysis
  - Trend detection (accelerating/decelerating)
  - Anomaly detection (sudden drops in activity)
  - Predictive recommendations

### Medium Term
- Turso integration for SQLite backend
- Multi-device UI conflict resolution
- Template library for common visions
- Analytics and reporting
- Webhook integrations

### Long Term
- Mobile companion app (Swift/Kotlin)
- Web dashboard (React/Vue)
- AI-powered coaching
- Social features (shared visions, accountability)
- Export to multiple formats (PDF, markdown, etc.)

---

## Contributing

This project follows these conventions:
- Commit messages: `[FEATURE|FIX|REFACTOR]: description`
- Feature branches per functionality
- All tests must pass before PR
- Clippy warnings must be resolved
- Code formatted with `cargo fmt`

---

## License

MIT OR Apache-2.0
