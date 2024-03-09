mod character;
mod combat;
mod enemy;
mod game;
mod overworld;
mod player;
mod stats;

pub mod prelude {
    use bevy::app::App;
    use bevy::prelude::*;

    use crate::character::CharacterPlugin;
    use crate::combat::CombatPlugin;
    use crate::enemy::EnemyPlugin;
    use crate::overworld::OverworldPlugin;
    use crate::player::PlayerPlugin;
    use crate::stats::StatsPlugin;

    #[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
    pub enum GameState {
        #[default]
        Overworld,
        Combat,
        CombatEnd,
    }

    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.init_state::<GameState>().add_plugins((
                OverworldPlugin,
                PlayerPlugin,
                EnemyPlugin,
                StatsPlugin,
                CombatPlugin,
            ));
        }
    }
}
