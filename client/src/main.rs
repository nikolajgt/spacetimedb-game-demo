mod module_bindings;

use bevy::log;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_spacetimedb::*;
use clap::Parser;
use serde::Deserialize;
use spacetimedb_sdk::{ReducerEvent, Table};
use module_bindings::*;

#[derive(Clone, Debug, Event)]
pub struct RegisterPlayerEvent {
    pub name: String,
}

#[derive(Clone, Debug, Event)]
pub struct OnRegisterPlayerEvent {
    pub event: ReducerEvent<Reducer>,
    pub name: String,
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
                    
                    info!("{:?}", &identity_res);
                    conn.run_threaded();
                    conn
                })
                .with_events(|plugin, app, db, reducers| {
                    tables!(
                        user_accounts,
                        characters
                    );

                    register_reducers!(
                        // on_register_player(ctx, name) => OnRegisterPlayerEvent {
                        //     event: ctx.event.clone(),
                        //     name: name.clone()
                        // }
                    );
                }),
        )
        .add_systems(
            Update,
            (on_connected, on_register_player, on_player, on_disconnect),
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
            .subscribe("SELECT * FROM player");
        
        
        let name = "Player54116".to_string();
       // stdb.reducers().register_player(name.clone()).expect("TODO: panic message");
        info!("Player {:?} trying to register", name);
    }
}

fn on_register_player(
    mut events: ReadReducerEvent<OnRegisterPlayerEvent>,
    stdb: Res<StdbConnection<DbConnection>>
) {
    for event in events.read() {
        let cloned_event = event.result.clone();
        info!("Registered player: {:?} status: {:?}", cloned_event.name, cloned_event.event.status);

        let users = stdb.db().user_accounts().count();
        info!("Existing player count: {}", users);
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