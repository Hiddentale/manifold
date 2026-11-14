use super::world_meta::WorldMeta;
use std::{
    fs,
    path::{Path, PathBuf},
};

const WORLDS_DIR: &str = "worlds";
const META_FILE: &str = "world.toml";

/// Create a new world directory with metadata. Returns the world directory path.
pub fn create_world(name: &str, seed: u32) -> anyhow::Result<PathBuf> {
    let dir_name = sanitize_dir_name(name);
    let dir = Path::new(WORLDS_DIR).join(&dir_name);
    anyhow::ensure!(!dir.exists(), "world directory already exists: {}", dir.display());
    let meta = WorldMeta {
        name: name.to_string(),
        seed,
        created_at: current_timestamp(),
    };
    meta.save(&dir)?;
    fs::create_dir_all(dir.join("svdag"))?;
    Ok(dir)
}

fn sanitize_dir_name(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

fn current_timestamp() -> String {
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("{secs}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_removes_special_chars() {
        assert_eq!(sanitize_dir_name("My World!"), "My_World_");
        assert_eq!(sanitize_dir_name("test-world_1"), "test-world_1");
    }
}
