use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SaveData {
    pub current_level: String,
    pub chapter_flags: HashMap<String, bool>,
}

pub fn save_path() -> std::path::PathBuf {
    std::path::PathBuf::from("saves/save.ron")
}

pub fn save_exists() -> bool {
    save_path().exists()
}

pub fn save_game(data: &SaveData) -> Result<(), Box<dyn std::error::Error>> {
    let path = save_path();
    std::fs::create_dir_all(path.parent().unwrap())?;
    let serialized = ron::to_string(data)?;
    std::fs::write(path, serialized)?;
    Ok(())
}

pub fn load_game() -> Option<SaveData> {
    let content = std::fs::read_to_string(save_path()).ok()?;
    ron::from_str(&content).ok()
}

pub fn save_game_to(data: &SaveData, path: &std::path::Path) -> Result<(), Box<dyn std::error::Error>> {
    std::fs::create_dir_all(path.parent().unwrap())?;
    let serialized = ron::to_string(data)?;
    std::fs::write(path, serialized)?;
    Ok(())
}

pub fn load_game_from(path: &std::path::Path) -> Option<SaveData> {
    let content = std::fs::read_to_string(path).ok()?;
    ron::from_str(&content).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_exists_false_when_no_file() {
        assert!(!std::path::Path::new("/tmp/flow_test_nonexistent_save.ron").exists());
    }

    #[test]
    fn round_trip_serialize() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("saves/save.ron");

        let mut data = SaveData::default();
        data.current_level = "level_01".to_string();
        data.chapter_flags.insert("kai_met_sela".to_string(), true);

        save_game_to(&data, &path).unwrap();
        assert!(path.exists());

        let loaded = load_game_from(&path).unwrap();
        assert_eq!(loaded.current_level, "level_01");
        assert_eq!(loaded.chapter_flags["kai_met_sela"], true);
    }

    #[test]
    fn serialization_round_trip_no_fs() {
        let mut data = SaveData::default();
        data.current_level = "test".to_string();
        data.chapter_flags.insert("flag_a".to_string(), false);

        let serialized = ron::to_string(&data).unwrap();
        let deserialized: SaveData = ron::from_str(&serialized).unwrap();
        assert_eq!(deserialized.current_level, "test");
        assert_eq!(deserialized.chapter_flags["flag_a"], false);
    }
}
