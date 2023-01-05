use gd_plist::Dictionary;
use serde::{Serialize, Deserialize};

use super::Icon;

/// The players icons customisations
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerIcons {
    player_frame: Icon,
    player_ship: Icon,
    player_ball: Icon,
    player_bird: Icon,
    player_dart: Icon,
    player_robot: Icon,
    player_spider: Icon,
    player_color: Icon,
    player_color2: Icon,
    player_streak: Icon,
    player_death_effect: Icon,
    player_icon_type: Icon,
    player_glow: bool,
}

/// Data Related to the player.
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    player_name: String,
    #[serde(rename = "playerUDID")]
    player_udid: String,
    #[serde(rename = "playerUserID")]
    player_user_id: i64,

    #[serde(flatten)]
    player_icons: PlayerIcons,
}

/// Generic Informations on this game session
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfos {
    bg_volume: f64,
    sfx_volume: f64,
    show_song_markers: bool,
    show_progress_bar: bool,

    clicked_garage: bool,
    clicked_editor: bool,
    clicked_practice: bool,
    showed_editor_guide: bool,
    showed_low_detail_dialog: bool,

    bootups: i64,
    has_rated_game: bool,
    binary_version: i64,
    resolution: i64,
    tex_quality: i64,

    custom_object_dict: Dictionary,
    reported_achievements: Dictionary,
}

/// Secrets from the game
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GameSecrets {
    secret_number: Option<i64>,
    #[serde(rename = "has_RP")]
    has_rp: Option<bool>,
    value_keeper: Dictionary,
    unlock_value_keeper: Dictionary,
}
