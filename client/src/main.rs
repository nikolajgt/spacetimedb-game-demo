mod module_bindings;

use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy_spacetimedb::*;
use spacetimedb_sdk::{ReducerEvent, Table};
use module_bindings::*;

#[derive(Clone, Debug, Event)]
pub struct RegisterPlayerEvent {
    pub event: ReducerEvent<Reducer>,
    pub name: String,
}

#[derive(Clone, Debug, Event)]
pub struct OnRegisterPlayerEvent {
    pub event: ReducerEvent<Reducer>,
    pub name: String,
}



fn main() {
    App::new()
        .add_event::<RegisterPlayerEvent>()
        .add_event::<OnRegisterPlayerEvent>()
        .add_plugins((MinimalPlugins, LogPlugin::default()))
        .add_plugins(
            StdbPlugin::default()
                .with_connection(|send_connected, send_disconnected, send_connect_error, _| {
                    let conn = DbConnection::builder()
                        .with_module_name("game-demo")
                        .with_uri("ws://localhost:3000")
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
                        player,
                    );

                    register_reducers!(
       
                        on_register_player(ctx, name) => OnRegisterPlayerEvent {
                            event: ctx.event.clone(),
                            name: name.clone()
                        }
                    );
                }),
        )
        .add_systems(
            Update,
            (on_connected, on_register_player, on_player),
        )
        .run();
    

}


fn on_connected(
    mut events: EventReader<StdbConnectedEvent>,
    stdb: Res<StdbConnection<DbConnection>>,
) {
    for _ in events.read() {
        info!("Connected to SpacetimeDB");

        // Call any reducers
        stdb.reducers().register_player("test".to_string()).expect("TODO: panic message");

        // Subscribe to any tables
        stdb.subscribe()
            .on_applied(|_| info!("Subscription to players applied"))
            .on_error(|_, err| error!("Subscription to players failed for: {}", err))
            .subscribe("SELECT * FROM player");

        // Access your database cache (since it's not yet populated here this line might return 0)
        let playerCount = stdb.db().player().count();
        info!("Players count: {}", playerCount);
    }
}

fn on_register_player(mut events: ReadReducerEvent<OnRegisterPlayerEvent>) {
    for event in events.read() {
        info!("Registered player: {:?}", event);
    }
}


fn on_player(mut events: ReadInsertEvent<Player>) {
    for event in events.read() {
        info!("Player inserted: {:?}", event.row);
    }
}