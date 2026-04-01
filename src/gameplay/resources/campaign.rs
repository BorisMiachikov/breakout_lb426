use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct CampaignManifest {
    pub levels: Vec<String>,
}

impl Default for CampaignManifest {
    fn default() -> Self {
        Self {
            levels: vec![
                "assets/levels/level1.json".to_string(),
                "assets/levels/level2.json".to_string(),
                "assets/levels/level3.json".to_string(),
                "assets/levels/level4.json".to_string(),
                "assets/levels/level5.json".to_string(),
            ],
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrentLevelIndex(pub usize);
