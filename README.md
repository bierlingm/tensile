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

Database stored at `~/.tensile/db.ron` (human-readable RON format).

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

- **Phase 1 ✓**: CLI scaffolding and core commands
- **Phase 2 ✓**: Data models and RON persistence
- **Phase 3 ✓**: MVP CLI implementation
- **Phase 4**: Business logic (pattern analysis, tension metrics)
- **Phase 5**: TUI dashboard
- **Phase 6**: Cloud sync via Turso

## License

MIT OR Apache-2.0
