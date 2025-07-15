use crate::stdb::module_bindings::characters_table::CharactersTableAccess;
use crate::stdb::module_bindings::user_accounts_table::UserAccountsTableAccess;
use bevy::log;
use bevy::prelude::*;
use bevy_spacetimedb::{register_reducers, tables, ReadReducerEvent, ReadUpdateEvent, StdbConnectedEvent, StdbConnection, StdbConnectionErrorEvent, StdbDisconnectedEvent, StdbPlugin};
use spacetimedb_sdk::ReducerEvent;
use crate::stdb::module_bindings::{DbConnection, Reducer, UserAccount};

pub mod module_bindings;

pub struct STDB {
    access_token:  String,
}

impl STDB {
    pub(crate) fn new(access_token: String) -> Self {
        Self { access_token }
    }
}


#[derive(Clone, Debug, Event)]
pub struct RegisterPlayerEvent {
    pub name: String,
}

#[derive(Clone, Debug, Event)]
pub struct OnSelectCharacterEvent {
    pub event: ReducerEvent<Reducer>,
    pub id: u128,
}

impl Plugin for STDB {
    fn build(&self, app: &mut App) {
        let token = self.access_token.clone();
        
        app.add_plugins(
            StdbPlugin::default()
                .with_connection(move |send_connected, send_disconnected, send_connect_error, _| {
                    let token = token.clone();
                    let conn = DbConnection::builder()
                        .with_module_name("game-demo")
                        .with_uri("ws://localhost:3000")
                        .with_token(Some(token))
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


                    conn.run_threaded();
                    conn
                })
                .with_events(|plugin, app, db, reducers| {
                    tables!(
                        user_accounts,
                        characters
                    );

                    register_reducers!(
                       
                    );
                }),
        )
            .add_systems(
                Update,
                (on_connected, on_select_character, on_player, on_disconnect),
            );
    }
}



fn on_connected(
    mut events: EventReader<StdbConnectedEvent>,
    stdb: Res<StdbConnection<DbConnection>>,
) {
    for _ in events.read() {
        info!("Connected to SpacetimeDB with identity: {:?}", stdb.identity());
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