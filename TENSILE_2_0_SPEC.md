# TENSILE 2.0 — THE AGENCY ENGINE

**Unified Specification Document + Executable Developer Instructions**

**Version:** 1.0  
**Purpose:** Guide reconstruction and evolution of Tensile into a Natural-Law-compatible, cognitively ergonomic structural tension engine.

---

## PART I — FULL SYSTEM SPECIFICATION

### 1. Purpose

Tensile exists to operationalize structural tension into a personal intelligence system.

**It is NOT:**
- A task manager
- A habit tracker
- A productivity database

**Its purpose IS:**
- To bring future → present
- To maintain directionality
- To minimize cognitive overhead
- To compute and surface the single next structural action
- To reflect the user's demonstrated interests
- To align perceived intentions with real behavior

**Tensile is a cognitive prosthetic, not a to-do list.**

---

### 2. First Principles (Natural Law Grammar)

#### 2.1 Objects in Natural Law Terms

| Tensile Concept | Natural Law Equivalent |
|-----------------|------------------------|
| Vision | Claim of a future state; a declaration under implied warranty |
| Reality | Testimony, current state, falsifiable |
| Action | Demonstrated Interest; costly, real behavior |
| Tension | Outstanding Liability between claim and testimony |
| Pattern | Evolutionary Trajectory of behavior across time |

#### 2.2 Behavior Model

- All behavior is acquisition
- All acquisition demonstrates an interest
- A vision is a declared interest
- A structural action is an actual interest
- **Tension = the gap between words and deeds**

**The system's job:** Reveal this gap and help the user close it.

---

### 3. The Agency Kernel

A minimal computational core that captures:

1. **Claim** (vision)
2. **Testimony** (reality)
3. **Tension** (gap)
4. **Trajectory** (pattern)
5. **Leverage-Point** (next structural action)

#### 3.1 Minimal Schema

**Vision**
- `id`: UUID
- `title`: String
- `stakes`: String (why this matters)
- `horizon`: Optional Date
- `desired_state`: String

**Reality**
- `current_state`: String
- `constraints`: Vec<String>
- `capacities`: Vec<String>

**Engine Fields** (computed, not user-entered)
- `tension_score`: f32
- `pattern_class`: Advancing | Oscillating | Stagnant
- `direction_vector`: Vec<String> (max 3 items)
- `leverage_point`: String (single actionable item)
- `last_action_timestamp`: Option<DateTime>

**None of this requires user micromanagement.**

---

### 4. Direction Vector

A generated structure based on the delta between Vision and Reality.

Contains exactly **2–3 items** representing:
- A constraint to remove
- A capability to build
- A condition that must change

This gives the system its "implied directionality."

---

### 5. Cognitive Ergonomics Rule

**Representation must never exceed what is required to compute:**

1. Tension ranking
2. Trajectory class
3. Leverage-point

**Any data without computational consequence = forbidden.**

---

### 6. Core Interactions (UX Philosophy)

The system has three primary actions:

#### 6.1 `ts focus`

Returns a concise card with:
- Highest-tension vision
- Why it matters
- Current reality
- Tension score
- Leverage-point
- "Do this now" directive

**This is the daily mode.**

#### 6.2 `ts now`

A short guided session:
1. Update reality (1–2 lines)
2. Confirm or adjust leverage-point
3. Optionally log a structural action

**Duration:** 30–60 seconds  
**This is the action mode.**

#### 6.3 `ts review`

A weekly ritual:
- Relist visions
- Recheck alignment between claims and testimony
- Adjust visions or desired states
- Refresh direction vectors

**Duration:** 5–10 minutes  
**This is the maintenance mode.**

---

### 7. Metrics (Sparse, High-Value Only)

**No micro-actions. No counts. No streaks.**

**Metrics:**

1. **Success Trajectory**
   - Advancing (>70% structural adherence)
   - Oscillating (30–70%)
   - Stagnant (<30%)

2. **Velocity**
   - Structural actions per week/month (smoothed)

3. **Tension Magnitude**
   - Computed from gap + stakes + time-decay

4. **Recency**
   - How long since last structural contact

This is enough to compute which vision "needs you now."

---

### 8. The TUI (Structural Redesign)

#### 8.1 Screens

1. **Focus Screen**
   - Vision title
   - Why it matters
   - Reality snapshot
   - Tension score
   - Leverage-point

2. **Now Screen**
   - Reflection prompt
   - Action logging
   - Auto regeneration of direction vector

3. **Review Screen**
   - Weekly resets
   - Vision adjustments

#### 8.2 Keyboard Philosophy

- `[Space]` = accept
- `[Enter]` = commit
- `[Esc]` = cancel
- `[1/2/3]` = switch modes
- Arrow keys = navigate simple lists

**Minimalism is a feature.**

---

### 9. Extension Roadmap (No Increased Logging)

Future enhancements come from **analysis, not more data input:**

- Predictive stagnation scoring
- Risk maps
- Trajectory forecasting
- Pattern clustering
- Coaching prompts
- Accountability loops
- Cross-vision conflict detection
- Habit slope estimation
- Constraint modeling
- External integrations

**All computation is derived from the Agency Kernel alone.**

---

### 10. System Guardrails

To prevent bloat, enforce:

1. No more than 3 editable fields in any daily interaction
2. No micro-action logging
3. Vision count must remain small
4. Every feature must improve:
   - Tension ranking
   - Leverage-point accuracy
   - Cognitive ergonomics
5. Any data not used to compute tension → removed
6. No new user-required fields without Kernel update approval

---

## PART II — EXECUTABLE DEVELOPER INSTRUCTION FILE

### Section A — Core Data Models (Canonical Specification)

#### A.1 Vision Object (Canonical Schema)

```rust
Vision {
    id: UUID,
    title: String,
    stakes: String,
    horizon: Option<Date>,
    desired_state: String,
    current_state: String,
    constraints: Vec<String>,
    capacities: Vec<String>,
    tension_score: f32,
    pattern_class: Enum { Advancing, Oscillating, Stagnant },
    direction_vector: Vec<String>,   // max 3 items
    leverage_point: String,          // single actionable item
    last_action_timestamp: Option<DateTime>
}
```

---

### Section B — System Behaviors (Required Invariants)

#### B.1 Tension Score Calculation

Must include:
- Distance between `desired_state` and `current_state`
- `stakes` weighting
- Time since last action
- Pattern class modifier

#### B.2 Direction Vector Generation

Algorithm:
1. Extract differences between `desired_state` and `current_state`
2. Identify top constraint
3. Identify top capability requirement
4. Identify top condition delta
5. Limit vector to max length 3

#### B.3 Leverage-Point Inference

```
If leverage_point is empty OR last_action_timestamp is old:
    leverage_point = highest-impact item from direction_vector
```

---

### Section C — CLI Implementation Specification

#### C.1 Command: `ts focus`

Returns JSON or TUI card with:

```json
{
   "vision_id": "uuid",
   "title": "string",
   "stakes": "string",
   "current_state": "string",
   "tension_score": 75.5,
   "leverage_point": "actionable string"
}
```

#### C.2 Command: `ts now`

Flow:
1. Prompt: "Update reality (1–2 sentences)."
2. Update `current_state`
3. Regenerate `direction_vector`
4. Recompute `leverage_point`
5. Ask: "Did you complete a structural action?"
   - If yes: update `last_action_timestamp`
   - Recompute metrics

#### C.3 Command: `ts review`

Flow:
- Iterate through visions
- Allow adjustments to:
  - `title`
  - `stakes`
  - `desired_state`
- Auto regenerate `direction_vector`
- Recompute tension

---

### Section D — TUI Specification

#### D.1 Screens
- Focus
- Now
- Review

#### D.2 Requirements
- Must start on Focus screen
- Must avoid multi-pane complexity
- Must show only what is needed for action

---

### Section E — Contribution & Feature Rules

#### E.1 Forbidden
- Task lists
- Daily micro-logging
- Arbitrary user-defined custom fields
- Streaks
- Gamification

#### E.2 Allowed
- Sparse structural actions
- Generated analytics
- Predictive modeling
- Adaptive coaching

#### E.3 Required
- Maintain minimal cognitive footprint
- Enforce invariant schema
- Keep user interactions under 60 seconds

---

### Section F — Implementation Roadmap (Canonical Order)

1. Implement Agency Kernel schema
2. Implement tension calculation
3. Implement direction vector generation
4. Implement leverage-point inference
5. Implement `ts focus`
6. Implement `ts now`
7. Implement `ts review`
8. Rebuild TUI around 3 screens
9. Add pattern classification
10. Add predictive analytics modules
11. Add optional integrations

---

### Section G — AI Agent Instructions

**If you are an AI building or modifying Tensile:**

1. **Always preserve cognitive ergonomics.**
2. **Never add a feature that increases user logging.**
3. **All new data must be derived from the Agency Kernel.**
4. **All reasoning must use Natural Law grammar:**
   - Vision = Claim
   - Reality = Testimony
   - Action = Demonstrated Interest
   - Tension = Liability
   - Pattern = Trajectory

5. **Prioritize:**
   - Directionality
   - Structural action
   - Real-world behavioral change

6. **Reject any change that:**
   - Introduces complexity
   - Creates data overhead
   - Distracts from action

7. **All improvements must reduce:**
   - Cognitive load
   - Friction
   - Inaction

8. **All improvements must increase:**
   - Accuracy of leverage-points
   - Alignment with user's demonstrated interests
   - Truthfulness about constraints

---

## How to Use This Document

**For AI Agents (Droids):**
- Read Section G first (AI Agent Instructions)
- Use Section B (System Behaviors) as invariants
- Reference Section F (Implementation Roadmap) for task prioritization
- Validate all changes against Section E (Feature Rules)

**For Human Developers:**
- Section I is the "white paper" explaining philosophy
- Section II is the executable specification
- Use Section F as the development roadmap
- Follow Section E guardrails strictly

**For All Contributors:**
- This is the source of truth
- Questions? Check this document first
- Propose changes via issues, referencing this spec

---

**Last Updated:** 2025-11-24  
**Spec Version:** 1.0  
**Status:** Active Development
