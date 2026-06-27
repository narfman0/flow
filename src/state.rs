use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Loading,
    MainMenu,
    InGame,
    Paused,
    Settings,
    Cutscene,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn states_are_distinct() {
        assert_ne!(GameState::Loading, GameState::MainMenu);
        assert_ne!(GameState::InGame, GameState::Paused);
        assert_ne!(GameState::Settings, GameState::Cutscene);
    }

    #[test]
    fn default_state_is_loading() {
        assert_eq!(GameState::default(), GameState::Loading);
    }
}
