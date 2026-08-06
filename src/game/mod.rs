pub mod camera;
pub mod dialogue;
pub mod disruption;
pub mod finale;
pub mod flow;
pub mod movement;
pub mod player;
pub mod wall_effect;
pub mod wall_movement;
pub mod wisp;
pub mod world;

use bevy::prelude::*;
use crate::state::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<disruption::DisruptionEvent>()
           .add_event::<wall_effect::WallEffectEvent>()
           .add_event::<finale::BubbleCloseEvent>()
           .init_resource::<disruption::DisruptionState>()
           .init_resource::<wall_effect::WallEffectState>()
           .init_resource::<dialogue::DialogueQueue>()
           .init_resource::<finale::FinaleState>()
           .add_systems(OnEnter(GameState::InGame), (
               camera::spawn_game_camera,
               player::spawn_player,
               world::setup_world,
           ))
           .add_systems(Update, (
               world::spawn_level_scene,
               player::read_player_input,
               flow::update_flow,
               movement::apply_movement,
               wall_movement::update_wall_movement,
               camera::follow_player,
               disruption::handle_disruption_events,
               disruption::clear_shaken_state,
               disruption::apply_shaken_movement,
               wisp::update_wisps,
               wisp::check_wisp_player_contact,
               dialogue::update_dialogue,
               dialogue::dialogue_ui,
               wall_effect::handle_wall_effect_events,
               wall_effect::tick_wall_effect,
               // Finale systems run in sequence: detect → respond → advance
               finale::check_origin_trigger,
               finale::handle_bubble_close_event,
               finale::advance_finale,
           ).run_if(in_state(GameState::InGame)))
           .add_systems(OnExit(GameState::InGame), teardown_game);
    }
}

fn teardown_game(mut commands: Commands, entities: Query<Entity, With<Camera3d>>) {
    for e in &entities {
        commands.entity(e).despawn();
    }
}
