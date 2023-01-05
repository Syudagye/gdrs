use serde::{Serialize, Deserialize};

use self::{game_manager::{PlayerInfo, GameInfos}, local_levels::CustomLevel};

pub mod game_manager;
pub mod local_levels;

pub type Icon = i64;

/// Data held by the "CCGameManager.dat" save file
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameManager {
    /// Informations related to the player
    #[serde(flatten)]
    player_infos: PlayerInfo,

    /// Other game related data
    #[serde(flatten)]
    game_infos: GameInfos,
}

/// Data held by the "CCLocalLevels.dat" save file
#[derive(Debug, Serialize, Deserialize)]
pub struct LocalLevels {
    #[serde(rename = "LLM_02")]
    binary_version: i64,

    #[serde(rename = "LLM_01")]
    levels: Vec<CustomLevel>,
}
