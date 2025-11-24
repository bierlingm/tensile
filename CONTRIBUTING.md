# Contributing to Tensile

Welcome! Tensile is being rebuilt as a **cognitive prosthetic, not a productivity tool**.

Before you contribute, please read this entire document and `TENSILE_2_0_SPEC.md`.

---

## Core Philosophy

Tensile operates on **Natural Law Grammar**:

- **Vision** = Claim (a declared future state)
- **Reality** = Testimony (current, falsifiable state)
- **Action** = Demonstrated Interest (costly, real behavior)
- **Tension** = Liability (gap between claim and testimony)
- **Pattern** = Trajectory (evolution of behavior over time)

**The system's only job:** Reveal the gap between what users *say* they want and what they *actually do*.

---

## The Non-Negotiable Rules

### 1. Cognitive Ergonomics First

- No feature is allowed if it increases user overhead
- No data field unless it computes toward tension, pattern, or leverage-point
- User interactions must complete in <60 seconds
- Max 3 editable fields per interaction

### 2. No User Micro-Logging

Forbidden:
- Daily check-ins
- Habit streaks
- Task decomposition
- Custom fields
- Arbitrary tagging

Allowed:
- Sparse structural actions
- Reality assessments (1–2 sentences)
- Vision updates (weekly)

### 3. All Data Must Compute

Ask before adding any field:

> "How does this improve tension ranking, leverage-point accuracy, or cognitive ergonomics?"

If you can't answer, don't add it.

### 4. Invariants Must Be Enforced

Never violate:
- Direction vector max length = 3
- Leverage-point = single actionable item
- Vision count soft limit ~20
- No arbitrary custom fields

---

## Development Workflow

### For AI Agents (Droids)

**Before you start:**

1. Read `TENSILE_2_0_SPEC.md` Section G (AI Instructions)
2. Check `IMPLEMENTATION_ROADMAP.md` for current phase
3. Verify your task doesn't violate rules above

**During implementation:**

1. Update tests first (TDD)
2. Reference Section B (System Behaviors) as invariants
3. Use Natural Law language in all reasoning
4. Validate changes against Section E (Feature Rules)

**Before submitting:**

1. Confirm no cognitive overhead increased
2. Verify all tests pass
3. Check: "Does this reduce user friction or increase accuracy?"
4. Update relevant docs

### For Human Developers

**Setup:**

```bash
git clone git@github.com:bierlingm/tensile.git
cd tensile

# Build CLI only
cargo build

# Build with TUI
cargo build --features tui

# Run tests
cargo test --all

# Format and lint
cargo fmt
cargo clippy -- -D warnings
```

**Branching:**

```bash
git checkout -b feature/your-feature-name
# Work...
git commit -m "[PHASE]: description

Natural-law explanation of why this aligns with Tensile philosophy."

git push origin feature/your-feature-name
# Create PR
```

**Commit Message Format:**

```
[PHASE-NUMBER]: Brief description

Body explaining:
- What problem this solves
- How it improves tension/pattern/leverage-point
- Why it reduces cognitive load
- Natural-law reasoning if applicable
```

Example:

```
[6]: Implement direction vector generation

Adds algorithmic derivation of direction_vector from gap between
desired_state and current_state. Extracts top constraint, capability,
and condition delta (max 3 items).

This enables leverage-point inference, reducing user micro-management
while increasing accuracy of actionable guidance.

Natural Law: Transforms testimony + claim into testimony of necessary
structural movements.
```

---

## Code Standards

### Rust Conventions

- Use `cargo fmt` before committing
- Follow clippy lints (no warnings allowed)
- 100% coverage on core logic (tension, pattern, leverage-point)
- Write tests before features

### Documentation

- Update `TENSILE_2_0_SPEC.md` if schema changes
- Update `IMPLEMENTATION_ROADMAP.md` as phases complete
- Add doc comments to public APIs
- Use Natural Law grammar in comments

### Error Handling

- Use typed errors (`TensileError` enum)
- Provide context about what failed
- Never hide errors; surface them to user

### Module Organization

```
src/
├── models/           # Data structures (Vision, Reality, etc.)
├── persistence/      # Storage layer (SQLite)
├── engine/           # Business logic (tension, pattern, etc.)
├── cli/              # CLI commands
├── tui/              # TUI implementation
├── error.rs          # Error types
├── config.rs         # Configuration
└── lib.rs / main.rs  # Exports and entry point
```

---

## Testing Strategy

### Unit Tests (Always)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tension_calculation_with_stakes() {
        // Test: Tension increases with higher stakes
    }

    #[test]
    fn test_direction_vector_max_length() {
        // Test: Never exceeds 3 items
    }

    #[test]
    fn test_leverage_point_inference() {
        // Test: Correctly derives from vector
    }
}
```

### Integration Tests

- CLI: `ts focus` returns correct vision
- Persistence: Save/load round-trip
- TUI: Navigation and input handling

### Manual Testing

- Create test vision with stakes
- Update reality
- Verify tension calculation
- Check leverage-point accuracy
- Test TUI if modifying screens

---

## Feature Request Process

**If you want to propose a new feature:**

1. **Check the rules first** (above)
2. **Ask the question:** "How does this improve:
   - Tension accuracy?
   - Leverage-point clarity?
   - Cognitive ergonomics?"
3. **Open an issue** with:
   - Problem statement (what gap does it close?)
   - Proposed solution
   - Answer to the question above
   - Natural-law reasoning

**Features likely to be REJECTED:**

- New user input fields (increases logging burden)
- Micro-action tracking (violates cognitive ergonomics)
- Arbitrary customization (complexity creep)
- Gamification (streaks, badges, etc.)
- "Nice to have" without clear computational purpose

**Features likely to be ACCEPTED:**

- Improved tension calculation
- Better leverage-point inference
- Reduced user friction
- Enhanced pattern accuracy
- Derived analytics (no new input)

---

## Getting Help

### For Questions About...

- **System design**: Read `TENSILE_2_0_SPEC.md`
- **Current state**: Check `IMPLEMENTATION_ROADMAP.md`
- **Code patterns**: Look at existing implementation in `src/`
- **Natural Law grammar**: Review Section 2 of TENSILE_2_0_SPEC.md

### For Bug Reports

Include:
- Steps to reproduce
- Expected vs actual behavior
- Environment (OS, Rust version)
- Any error messages

### For Security Issues

**Do not open a public issue.** Email: [security contact]

---

## Release Cycle

Tensile follows semantic versioning:

- **v1.x**: MVP (CLI + TUI foundation) ✅ Complete
- **v2.x**: Agency Kernel (Tensile 2.0) — In progress
- **v3.x**: Predictive analytics and integrations

Release checklist:
- [ ] All tests pass
- [ ] No clippy warnings
- [ ] Documentation updated
- [ ] Version bumped in Cargo.toml
- [ ] Changelog entry written
- [ ] Tag created and pushed

---

## Code Review Checklist

**For reviewers:**

- [ ] Does this follow Natural Law grammar?
- [ ] Is cognitive overhead increased or decreased?
- [ ] Are tests comprehensive?
- [ ] Does it violate any non-negotiable rules?
- [ ] Is documentation updated?
- [ ] Does it improve tension/pattern/leverage-point?

**For authors:**

- [ ] Tests pass locally (`cargo test --all`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy -- -D warnings`)
- [ ] Docs updated
- [ ] Commit message explains *why*, not just *what*

---

## Questions?

1. Read the docs (they're comprehensive)
2. Check existing issues
3. Open a new issue with context
4. Reference the spec by section

---

**Remember:** Tensile's power comes from simplicity. Every feature we add costs the user cognitive energy. We only add features that reduce that cost more than they consume it.

Welcome to the project. 🎯
