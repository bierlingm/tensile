# Tensile (ts) - A Terminal-Based Cognitive Prosthetic for Structural Dynamics

A Rust CLI tool designed to support users in practicing structural dynamics by establishing and maintaining structural tension between clear visions and current reality, driving consistent, creative action toward desired outcomes.

## Installation

```bash
cargo build --release
```

The binary will be available at `target/release/tensile`.

## Quick Start

### Create a Vision
```bash
tensile vision new "Learn Rust programming"
```

### View All Visions
```bash
tensile vision tree
```

### Log an Action
```bash
tensile action log <vision-id> "Read Rust book chapter 1"
```

### Update Current Reality
```bash
tensile reality update <vision-id> "Currently on chapter 1, understanding ownership"
```

### Check Patterns
```bash
tensile pattern check
```

### View Vision State
```bash
tensile state show <vision-id>
```

### Get Coaching Prompts
```bash
tensile prompt
```

## Architecture

- **Models**: Vision, RealityAssessment, ActionLog, User state
- **Persistence**: RON-based file storage (upgradeable to SQLite)
- **Engine**: Pattern analysis (advancing vs oscillating), state machine validation
- **CLI**: Full command structure with subcommands and aliases

## Database

**Local Storage**: SQLite database at `~/.tensile/tensile.db`

**Cloud Sync**: Integrated with Turso for automatic cloud synchronization
- Local-first: Works offline, syncs when connected
- Multi-device: Access your visions from anywhere
- Configuration: Set `TURSO_URL` and `TURSO_TOKEN` environment variables

**Schema**: Tables for visions, reality_assessments, action_logs, and user state with proper indexes for performance.

## Development

Run tests:
```bash
cargo test
```

Run linter:
```bash
cargo clippy -- -D warnings
```

Format code:
```bash
cargo fmt
```

## Features (Phases)

### Completed
- **Phase 1 ✓**: CLI scaffolding and core commands
- **Phase 2 ✓**: Data models and RON persistence
- **Phase 3 ✓**: MVP CLI implementation
- **Phase 4 ✓**: Business logic (pattern analysis, tension metrics)
- **Phase 5 ✓**: TUI dashboard framework (ratatui integration)
- **Phase 6 ✓**: Cloud sync infrastructure (reqwest/async)

## Phase 4: Business Logic & Metrics

### New Commands
```bash
# Show visions prioritized by structural tension
tensile metrics priority

# View detailed metrics for a vision
tensile metrics detail <vision-id>

# Display summary dashboard
tensile metrics summary
```

### Features
- **Tension Calculation**: Automatically calculates gap between vision and reality
- **Pattern Metrics**: Detailed success rates, velocity (actions/day), recent activity
- **Priority Ranking**: Surfaces highest-tension visions needing attention
- **Enhanced Pattern Detection**: 
  - Advancing (>70% success rate)
  - Oscillating (30-70% success rate)
  - Stagnant (<30% success rate)

### Engine Enhancements
- `TensionCalculator`: Calculates tension scores with completed vision filtering
- `PatternMetrics`: Tracks success rate, velocity, and 7-day recent actions
- `PatternAnalyzer::get_detailed_metrics()`: Provides comprehensive behavior analysis

## Phase 5: TUI Dashboard (In Progress)

### Structure
```
src/tui/
├── mod.rs          # TUI orchestration  
└── dashboard.rs    # Dashboard rendering (framework ready)
```

### Build & Run
```bash
# Build with TUI support
cargo build --features tui

# Interactive dashboard (coming soon)
cargo run --features tui -- dashboard
```

### Technologies
- **ratatui**: Modern TUI rendering
- **crossterm**: Terminal event handling

## Cloud Sync via Turso

Tensile uses **Turso** (SQLite in the cloud) for automatic cloud synchronization:

```bash
# Set up Turso integration
export TURSO_URL="libsql://your-db-url.turso.io"
export TURSO_TOKEN="your-auth-token"

# Automatic sync happens in background
# Local SQLite db stays in sync with Turso
```

**Benefits**:
- ✓ Managed cloud infrastructure (no custom API)
- ✓ Local-first architecture (works offline)
- ✓ Multi-device sync (access from anywhere)
- ✓ Automatic conflict resolution
- ✓ SQL queries for advanced filtering

## License

MIT OR Apache-2.0
