-- Tensile initial schema

CREATE TABLE IF NOT EXISTS visions (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    parent_id TEXT REFERENCES visions(id),
    created_at TIMESTAMP NOT NULL,
    completed BOOLEAN NOT NULL DEFAULT 0,
    state TEXT NOT NULL CHECK(state IN ('Conceived', 'InProgress', 'Blocked', 'Reassessed', 'Achieved'))
);

CREATE TABLE IF NOT EXISTS reality_assessments (
    id TEXT PRIMARY KEY,
    vision_id TEXT NOT NULL REFERENCES visions(id) ON DELETE CASCADE,
    entry TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL
);

CREATE TABLE IF NOT EXISTS action_logs (
    id TEXT PRIMARY KEY,
    vision_id TEXT NOT NULL REFERENCES visions(id) ON DELETE CASCADE,
    entry TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL,
    success BOOLEAN NOT NULL DEFAULT 1
);

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY,
    current_focus TEXT REFERENCES visions(id),
    last_reviewed TIMESTAMP
);

-- Indexes for common queries
CREATE INDEX IF NOT EXISTS idx_reality_vision ON reality_assessments(vision_id);
CREATE INDEX IF NOT EXISTS idx_action_vision ON action_logs(vision_id);
CREATE INDEX IF NOT EXISTS idx_vision_parent ON visions(parent_id);
CREATE INDEX IF NOT EXISTS idx_reality_timestamp ON reality_assessments(timestamp DESC);
CREATE INDEX IF NOT EXISTS idx_action_timestamp ON action_logs(timestamp DESC);
