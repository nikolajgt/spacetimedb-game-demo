mod asset_loader;
mod stdb;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use clap::Parser;
use serde::{Deserialize, Serialize};
use spacetimedb_sdk::Table;
use self::stdb::module_bindings::*;
use crate::stdb::STDB;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum GameStage {
    Input,
    Triggered,
    Logic,
    Animation,
    Physics,
    Visuals,
    Cleanup,
}

#[derive(Resource, Default)]
pub struct SetupProgress {
    pub player_spawned: bool,
    pub enemies_spawned: bool,
    pub world_ready: bool,
}

#[derive(States, Clone, Eq, PartialEq, Default, Hash, Debug)]
pub enum GameState {
    #[default]
    LoadingAssets,
    AssetsLoaded,
    InGame,
}


#[derive(Debug, Deserialize)]
pub struct IdentityResponse {
    pub token: String
}


#[derive(Parser, Debug)]
#[command(name = "game-client", version, about = "SpaceTimeDB Bevy Client")]
struct Args {
    /// JWT token used for authentication
    #[arg(long)]
    token: String,
}
#[derive(Serialize)]
struct IdentityRequest {
    token: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let jwt_token = args.token;
    let identity_res = reqwest::Client::new()
        .post("http://localhost:3010/identity")
        .bearer_auth(&jwt_token)
        .send()
        .await
        .expect("Failed to fetch identity")
        .json::<IdentityResponse>()
        .await
        .expect("Failed to parse identity response");

    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(STDB::new(identity_res.token))
        .run();
}

