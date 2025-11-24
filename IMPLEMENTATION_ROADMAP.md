# Tensile Implementation Roadmap

**Current State:** MVP CLI + TUI Foundation  
**Target State:** Tensile 2.0 Agency Engine  
**Last Updated:** 2025-11-24

---

## Current Architecture (v0.1.0)

### ✅ Completed

**Phase 1: Project Scaffolding**
- Modular Rust project structure
- Cargo.toml with all core dependencies
- Git repository initialized

**Phase 2: Data Models & Persistence**
- SQLite backend (Turso-ready)
- Core models: Vision, RealityAssessment, ActionLog, User
- Migrations and schema

**Phase 3: CLI Implementation**
- Command structure: vision, reality, action, pattern, state, metrics, prompt
- Command aliases for rapid use
- Error handling and validation

**Phase 4: Business Logic**
- TensionCalculator (gap-based scoring)
- PatternAnalyzer (advancing/oscillating/stagnant detection)
- StateMachine (state transition validation)
- MetricsCommands for insights

**Phase 5: TUI Foundation**
- Ratatui integration (feature-gated)
- Event handling with crossterm
- Basic dashboard layout
- Navigation and input handling

### ❌ Not Yet Implemented

- Direction vector generation
- Leverage-point inference
- Stress/stakes field
- Horizon date
- Constraint/capacity modeling
- Advanced pattern analytics
- Predictive modeling

---

## Tensile 2.0 Roadmap

### Phase 6: Agency Kernel Modernization (PRIORITY 1)

**Goal:** Implement the core computational engine per TENSILE_2_0_SPEC.md

**Tasks:**

1. **Update Vision Schema**
   - Add: `stakes`, `horizon`, `desired_state`, `constraints`, `capacities`
   - Update database schema (new migration)
   - Update SQLite store

2. **Implement Direction Vector Generation**
   - Algorithm: Extract differences between `desired_state` and `current_state`
   - Identify top constraint, capability, condition delta
   - Limit to max 3 items
   - Store in `direction_vector` field

3. **Implement Leverage-Point Inference**
   - Rule: If empty or stale, derive from `direction_vector`
   - Update on every reality update
   - Cache computation results

4. **Enhance Tension Calculation**
   - Add stakes weighting
   - Add time-decay factor
   - Add pattern class modifier
   - Make deterministic and reproducible

**Files to Modify:**
- `src/models/vision.rs` — Add schema fields
- `src/models/mod.rs` — Update Database struct
- `migrations/001_initial_schema.sql` — New columns
- `src/persistence/sqlite_store.rs` — Update load/save
- `src/engine/tension.rs` — Enhanced calculation
- Create `src/engine/direction.rs` — New module

**Tests Required:**
- Direction vector generation with mock data
- Leverage-point inference edge cases
- Tension calculation with all factors

---

### Phase 7: CLI Redesign (PRIORITY 2)

**Goal:** Implement the three core commands per spec

**Tasks:**

1. **Implement `ts focus`**
   - Query highest-tension vision
   - Return concise card (JSON or formatted)
   - Show: title, stakes, current_state, tension, leverage_point

2. **Implement `ts now`**
   - Prompt for reality update (1–2 lines)
   - Regenerate direction_vector
   - Ask if structural action was completed
   - Update last_action_timestamp if yes

3. **Implement `ts review`**
   - Interactive iteration through visions
   - Allow edits to: title, stakes, desired_state
   - Auto-regenerate direction_vector
   - Recompute tension

**Files to Create/Modify:**
- `src/cli/commands/focus.rs` — New command module
- `src/cli/commands/now.rs` — New command module
- `src/cli/commands/review.rs` — New command module
- `src/cli/commands/mod.rs` — Register new commands

**Tests Required:**
- Focus command returns correct vision
- Now command flow (input → update → regenerate)
- Review command allows valid edits only

---

### Phase 8: TUI Redesign (PRIORITY 3)

**Goal:** Rebuild TUI around 3 screens per spec

**Tasks:**

1. **Focus Screen**
   - Display: title, stakes, current_state, tension, leverage_point
   - Action: Log structural action or move to Next

2. **Now Screen**
   - Input: Update reality
   - Display: Regenerated direction_vector
   - Confirm: Log structural action (Y/N)
   - Auto-update last_action_timestamp

3. **Review Screen**
   - List visions
   - Edit mode: title, stakes, desired_state
   - Auto-regenerate on edit
   - Confirm changes

**Files to Modify:**
- `src/tui/app.rs` — Add new Screen variants
- `src/tui/dashboard.rs` — Redesigned layouts
- `src/tui/events.rs` — Input handling for new screens

**Keyboard Shortcuts:**
- `[1/2/3]` switch between Focus/Now/Review
- `[Space]` accept
- `[Enter]` commit
- `[Esc]` cancel

**Tests Required:**
- Screen transitions work correctly
- Input validation (not too long, required fields)
- Data persistence after edit

---

### Phase 9: Metrics Refinement (PRIORITY 4)

**Goal:** Implement sparse, high-value metrics only

**Tasks:**

1. **Pattern Classification**
   - Already implemented, verify correctness
   - Advancing: >70% structural adherence
   - Oscillating: 30–70%
   - Stagnant: <30%

2. **Velocity Calculation**
   - Structural actions per week (smoothed)
   - Time-windowed average
   - Exclude micro-actions

3. **Recency Scoring**
   - Days since last_action_timestamp
   - Use in tension calculation decay

**Files to Modify:**
- `src/engine/pattern.rs` — Verify logic
- `src/engine/tension.rs` — Add recency factor

---

### Phase 10: Guardrails & Validation (PRIORITY 5)

**Goal:** Enforce system guardrails

**Tasks:**

1. **Schema Validation**
   - Max 3 items in direction_vector
   - Prevent arbitrary custom fields
   - Validate stakes, horizon, desired_state formats

2. **Input Limits**
   - Daily interactions max 3 editable fields
   - Prevent micro-action logging
   - Vision count soft limit (warn at 20+)

3. **Data Cleanup**
   - Identify unused fields
   - Remove non-computational data
   - Update tests

**Files to Create:**
- `src/validation.rs` — New module for guardrails

---

### Phase 11: Predictive Analytics (PRIORITY 6)

**Goal:** Begin derived analytics

**Tasks:**

1. **Stagnation Predictor**
   - Model: If pattern stagnant for >2 weeks, flag
   - Output: Actionable recommendation

2. **Trajectory Forecast**
   - Based on last 4 weeks pattern
   - Predict next 2 weeks class

3. **Conflict Detection**
   - Find visions with incompatible constraints
   - Warn user of structural conflicts

**Files to Create:**
- `src/engine/analytics.rs` — New module

---

### Phase 12: Integrations (PRIORITY 7)

**Goal:** Optional external connections (no new logging)

**Possible:**
- Turso cloud sync
- Slack daily focus reminders
- Calendar export
- Markdown export/import

**Rule:** No additional user data collection. All data flows from Agency Kernel.

---

## Migration Path

### Step 1: Backup Current State
```bash
git tag -a v1.0-pre-agency-kernel -m "Before Tensile 2.0 transition"
git push origin v1.0-pre-agency-kernel
```

### Step 2: Branch for Development
```bash
git checkout -b feature/agency-kernel-v2
```

### Step 3: Implement in Order
1. Phase 6 (Agency Kernel)
2. Phase 7 (CLI)
3. Phase 8 (TUI)
4. Phase 9 (Metrics)
5. Phase 10 (Guardrails)
6. Phase 11 (Analytics)
7. Phase 12 (Integrations)

### Step 4: Test Thoroughly
- Unit tests for all new logic
- Integration tests for CLI flow
- Manual testing of TUI
- Load testing with 50+ visions

### Step 5: Release
```bash
git tag -a v2.0-agency-engine -m "Tensile 2.0: Agency Engine"
git push origin feature/agency-kernel-v2
git push origin v2.0-agency-engine
```

---

## Success Criteria

**Phase 6 Complete When:**
- Direction vector generates correctly
- Leverage-point infers from vector
- Tension includes all factors
- Tests pass with 100% coverage of logic

**Phase 7 Complete When:**
- `ts focus` returns correct highest-tension vision
- `ts now` flow: update → regenerate → log
- `ts review` allows edits and auto-regenerates

**Phase 8 Complete When:**
- TUI shows 3 screens cleanly
- Navigation works with [1/2/3]
- Input handling is robust
- No crashes on edge cases

**Full Tensile 2.0 Complete When:**
- All phases 6–12 implemented
- All tests pass
- User can complete daily focus → now → action cycle in <60 seconds
- System accurately reflects gap between claims and testimony

---

## Developer Checklist

For any developer (human or AI) working on this project:

- [ ] Read `TENSILE_2_0_SPEC.md` completely
- [ ] Understand the Agency Kernel before coding
- [ ] Follow Phase sequence (don't skip ahead)
- [ ] Write tests before features
- [ ] Validate against guardrails (Section E)
- [ ] Keep interactions under 3 editable fields
- [ ] Review all changes use Natural Law grammar
- [ ] Ensure changes reduce cognitive load

---

## Questions?

Refer to:
1. `TENSILE_2_0_SPEC.md` — Full system specification
2. `PHASES.md` — Historical development phases (v1.0)
3. `README.md` — Quick start and current state

---

**Next Step:** Start Phase 6 (Agency Kernel Modernization)

**Estimated Timeline:** 2–4 weeks for full Tensile 2.0 implementation
