mod module_bindings;

use bevy::log;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_spacetimedb::*;
use clap::Parser;
use serde::{Deserialize, Serialize};
use spacetimedb_sdk::{ReducerEvent, Table};
use module_bindings::*;

#[derive(Clone, Debug, Event)]
pub struct RegisterPlayerEvent {
    pub name: String,
}

#[derive(Clone, Debug, Event)]
pub struct OnSelectCharacterEvent {
    pub event: ReducerEvent<Reducer>,
    pub id: u128,
}

#[derive(Debug, Deserialize)]
pub struct IdentityResponse {
    pub token: String,
    pub identity: String,
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
    
    info!("IDENTITY: {}", identity_res.identity);
    App::new()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(
            StdbPlugin::default()
                .with_connection(move |send_connected, send_disconnected, send_connect_error, _| {
                    let conn = DbConnection::builder()
                        .with_module_name("game-demo")
                        .with_uri("ws://localhost:3000")
                        .with_token(Some(identity_res.token.clone()))
                        .on_connect_error(move |_ctx, err| {
                            send_connect_error
                                .send(StdbConnectionErrorEvent { err })
                                .unwrap();
                        })
                        .on_disconnect(move |_ctx, err| {
                            send_disconnected
                                .send(StdbDisconnectedEvent { err })
                                .unwrap();
                        })
                        .on_connect(move |_ctx, _id, _c| {
                            send_connected.send(StdbConnectedEvent {}).unwrap();
                        })
                        .build()
                        .expect("SpacetimeDB connection failed");

                    info!("IDENTITY: {}", identity_res.identity);

                    conn.run_threaded();
                    conn
                })
                .with_events(|plugin, app, db, reducers| {
                    tables!(
                        user_accounts,
                        characters
                    );

                    register_reducers!(
                        on_select_character(ctx, id) => OnSelectCharacterEvent {
                            event: ctx.event.clone(),
                            id: id.clone()
                        }
                    );
                }),
        )
        .add_systems(
            Update,
            (on_connected, on_select_character, on_player, on_disconnect),
        )
        .run();
}


fn on_connected(
    mut events: EventReader<StdbConnectedEvent>,
    stdb: Res<StdbConnection<DbConnection>>,
) {
    for _ in events.read() {
        info!("Connected to SpacetimeDB");
        stdb.subscribe()
            .on_applied(|_| info!("Subscription to players applied"))
            .on_error(|_, err| error!("Subscription to players failed for: {}", err))
            .subscribe("SELECT * FROM user_accounts");
    }
}

fn on_select_character(
    mut events: ReadReducerEvent<OnSelectCharacterEvent>
) {
    for event in events.read() {
        let cloned_event = event.result.clone();
        info!("Selected character: {:?} status: {:?}", &cloned_event.id, &cloned_event.event.status);
    }
}


fn on_player(mut events: ReadUpdateEvent<UserAccount>) {
    for event in events.read() {
        let test = event.new.clone();
        info!("Player inserted: {:?}", test);
    }
}

fn on_disconnect(
    mut events: EventReader<StdbDisconnectedEvent>,
    stdb: Res<StdbConnection<DbConnection>>,
) {
    for _ in events.read() {
        match stdb.disconnect() {
            Ok(_) => {},
            Err(err) => {
                log::warn!("{:?}", err);
            }
        }
    }
}