use bevy::prelude::*;
use leafwing_input_manager::InputManagerBundle;

use crate::{
    animation::Facing,
    consts,
    fighter::Inventory,
    input::PlayerAction,
    metadata::{AttackMeta, FighterMeta, FighterSpawnMeta, GameMeta, Settings},
};

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerIndex(pub usize);

#[derive(Bundle)]
pub struct PlayerBundle {
    player: Player,
    index: PlayerIndex,
    facing: Facing,
    inventory: Inventory,
    #[bundle]
    transform_bundle: TransformBundle,
    fighter_handle: Handle<FighterMeta>,
    #[bundle]
    input_manager_bundle: InputManagerBundle<PlayerAction>,
    available_attacks: AvailableAttacks,
}

impl PlayerBundle {
    pub fn new(
        player_meta: &FighterSpawnMeta,
        player_i: usize,
        game_meta: &GameMeta,
        settings: Option<&Settings>,
        fighter_assets: &Res<Assets<FighterMeta>>,
    ) -> Self {
        let ground_offset = Vec3::new(0.0, consts::GROUND_Y, 0.0);
        let player_pos = player_meta.location + ground_offset;

        let transform_bundle =
            TransformBundle::from_transform(Transform::from_translation(player_pos));

        let fighter_handle = player_meta.fighter_handle.clone();

        let input_manager_bundle = InputManagerBundle {
            input_map: settings
                .unwrap_or(&game_meta.default_settings)
                .player_controls
                .get_input_map(player_i),
            ..default()
        };

        let available_attacks = AvailableAttacks(vec![fighter_assets
            .get(&player_meta.fighter_handle)
            .expect("Fighter not loaded")
            .attack
            .clone()]);

        PlayerBundle {
            player: Player,
            index: PlayerIndex(player_i),
            facing: Facing::Right,
            transform_bundle,
            fighter_handle,
            input_manager_bundle,
            inventory: Inventory(None),
            available_attacks,
        }
    }
}

#[derive(Component)]
pub struct AvailableAttacks(pub Vec<AttackMeta>);
