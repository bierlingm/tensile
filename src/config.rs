use std::path::PathBuf;

pub fn db_path() -> PathBuf {
    dirs::home_dir()
        .map(|home| home.join(".tensile").join("db.ron"))
        .unwrap_or_else(|| PathBuf::from("tensile_db.ron"))
}

pub fn ensure_db_dir() -> std::io::Result<()> {
    let db_path_val = db_path();
    if let Some(db_dir) = db_path_val.parent() {
        std::fs::create_dir_all(db_dir)?;
    }
    Ok(())
}
