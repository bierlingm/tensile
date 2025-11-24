use crate::error::{TensileError, TensileResult};
use crate::models::*;
use rusqlite::{params, Connection};
use std::path::PathBuf;
use uuid::Uuid;

pub struct SqliteStore {
    db_path: PathBuf,
}

impl SqliteStore {
    pub fn new(db_path: Option<PathBuf>) -> TensileResult<Self> {
        let path = db_path.unwrap_or_else(|| {
            dirs::home_dir()
                .map(|h| h.join(".tensile").join("tensile.db"))
                .unwrap_or_else(|| PathBuf::from("tensile.db"))
        });

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let store = SqliteStore { db_path: path };
        store.init_db()?;
        Ok(store)
    }

    fn init_db(&self) -> TensileResult<()> {
        let conn = Connection::open(&self.db_path)?;
        let schema = include_str!("../../migrations/001_initial_schema.sql");
        conn.execute_batch(schema)
            .map_err(|e| TensileError::Serialization(e.to_string()))?;
        Ok(())
    }

    fn get_connection(&self) -> TensileResult<Connection> {
        Connection::open(&self.db_path).map_err(|e| e.into())
    }
}

impl SqliteStore {
    pub fn load() -> TensileResult<Database> {
        let store = SqliteStore::new(None)?;
        let conn = store.get_connection()?;

        // Load visions
        let mut stmt = conn
            .prepare("SELECT id, title, description, parent_id, created_at, completed, state FROM visions")
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        let visions = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let title: String = row.get(1)?;
                let description: Option<String> = row.get(2)?;
                let parent_id: Option<String> = row.get(3)?;
                let created_at: String = row.get(4)?;
                let completed: bool = row.get(5)?;
                let state_str: String = row.get(6)?;

                let id = Uuid::parse_str(&id_str).map_err(|_| rusqlite::Error::InvalidQuery)?;
                let parent = parent_id.and_then(|p| Uuid::parse_str(&p).ok());
                let created_at = chrono::DateTime::parse_from_rfc3339(&created_at)
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .ok_or(rusqlite::Error::InvalidQuery)?;

                let state = match state_str.as_str() {
                    "Conceived" => VisionState::Conceived,
                    "InProgress" => VisionState::InProgress,
                    "Blocked" => VisionState::Blocked,
                    "Reassessed" => VisionState::Reassessed,
                    "Achieved" => VisionState::Achieved,
                    _ => VisionState::Conceived,
                };

                Ok(Vision {
                    id,
                    title,
                    description,
                    parent,
                    children: vec![],
                    created_at,
                    completed,
                    state,
                })
            })
            .map_err(|e| TensileError::Serialization(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        // Rebuild parent-child relationships
        let mut visions_map: std::collections::HashMap<Uuid, Vision> =
            visions.into_iter().map(|v| (v.id, v)).collect();

        let child_ids: Vec<(Uuid, Option<Uuid>)> =
            visions_map.values().map(|v| (v.id, v.parent)).collect();

        for (child_id, parent_id) in child_ids {
            if let Some(parent_uuid) = parent_id {
                if let Some(parent) = visions_map.get_mut(&parent_uuid) {
                    parent.children.push(child_id);
                }
            }
        }

        let visions: Vec<Vision> = visions_map.into_values().collect();

        // Load reality assessments
        let mut stmt = conn
            .prepare("SELECT id, vision_id, entry, timestamp FROM reality_assessments ORDER BY timestamp DESC")
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        let realities = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let vision_id_str: String = row.get(1)?;
                let entry: String = row.get(2)?;
                let timestamp_str: String = row.get(3)?;

                let id = Uuid::parse_str(&id_str).map_err(|_| rusqlite::Error::InvalidQuery)?;
                let vision_id =
                    Uuid::parse_str(&vision_id_str).map_err(|_| rusqlite::Error::InvalidQuery)?;
                let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .ok_or(rusqlite::Error::InvalidQuery)?;

                Ok(RealityAssessment {
                    id,
                    vision_id,
                    entry,
                    timestamp,
                })
            })
            .map_err(|e| TensileError::Serialization(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        // Load action logs
        let mut stmt = conn
            .prepare("SELECT id, vision_id, entry, timestamp, success FROM action_logs ORDER BY timestamp DESC")
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        let actions = stmt
            .query_map([], |row| {
                let id_str: String = row.get(0)?;
                let vision_id_str: String = row.get(1)?;
                let entry: String = row.get(2)?;
                let timestamp_str: String = row.get(3)?;
                let success: bool = row.get(4)?;

                let id = Uuid::parse_str(&id_str).map_err(|_| rusqlite::Error::InvalidQuery)?;
                let vision_id =
                    Uuid::parse_str(&vision_id_str).map_err(|_| rusqlite::Error::InvalidQuery)?;
                let timestamp = chrono::DateTime::parse_from_rfc3339(&timestamp_str)
                    .ok()
                    .map(|dt| dt.with_timezone(&chrono::Utc))
                    .ok_or(rusqlite::Error::InvalidQuery)?;

                Ok(ActionLog {
                    id,
                    vision_id,
                    entry,
                    timestamp,
                    success,
                })
            })
            .map_err(|e| TensileError::Serialization(e.to_string()))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        Ok(Database {
            visions,
            realities,
            actions,
            user: None,
        })
    }

    pub fn save(db: &Database) -> TensileResult<()> {
        let store = SqliteStore::new(None)?;
        let mut conn = store.get_connection()?;

        // Start transaction
        let tx = conn
            .transaction()
            .map_err(|e| TensileError::Database(e.to_string()))?;

        // Clear existing data
        tx.execute("DELETE FROM action_logs", [])
            .map_err(|e| TensileError::Serialization(e.to_string()))?;
        tx.execute("DELETE FROM reality_assessments", [])
            .map_err(|e| TensileError::Serialization(e.to_string()))?;
        tx.execute("DELETE FROM visions", [])
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        // Insert visions
        for vision in &db.visions {
            tx.execute(
                "INSERT INTO visions (id, title, description, parent_id, created_at, completed, state) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
                params![
                    vision.id.to_string(),
                    &vision.title,
                    &vision.description,
                    vision.parent.map(|p| p.to_string()),
                    vision.created_at.to_rfc3339(),
                    vision.completed,
                    vision.state.to_string(),
                ],
            )
            .map_err(|e| TensileError::Serialization(e.to_string()))?;
        }

        // Insert reality assessments
        for reality in &db.realities {
            tx.execute(
                "INSERT INTO reality_assessments (id, vision_id, entry, timestamp) VALUES (?1, ?2, ?3, ?4)",
                params![
                    reality.id.to_string(),
                    reality.vision_id.to_string(),
                    &reality.entry,
                    reality.timestamp.to_rfc3339(),
                ],
            )
            .map_err(|e| TensileError::Serialization(e.to_string()))?;
        }

        // Insert action logs
        for action in &db.actions {
            tx.execute(
                "INSERT INTO action_logs (id, vision_id, entry, timestamp, success) VALUES (?1, ?2, ?3, ?4, ?5)",
                params![
                    action.id.to_string(),
                    action.vision_id.to_string(),
                    &action.entry,
                    action.timestamp.to_rfc3339(),
                    action.success,
                ],
            )
            .map_err(|e| TensileError::Serialization(e.to_string()))?;
        }

        tx.commit()
            .map_err(|e| TensileError::Serialization(e.to_string()))?;

        Ok(())
    }
}
