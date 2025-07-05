mod module_bindings;

use bevy::log::LogPlugin;
use bevy::log::tracing::instrument::WithSubscriber;
use bevy::prelude::*;
use bevy_spacetimedb::*;
use spacetimedb_sdk::{ReducerEvent, SubscriptionHandle, Table};
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



fn main() {
    App::new()
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
                        player_movement
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
        stdb.subscribe()
            .on_applied(|_| info!("Subscription to players applied"))
            .on_error(|_, err| error!("Subscription to players failed for: {}", err))
            .subscribe("SELECT * FROM player");
        
        
        let name = "Player8".to_string();
        stdb.reducers().register_player(name.clone()).expect("TODO: panic message");
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

        let players = stdb.db().player().count();
        info!("Existing player count: {}", players);
    }
}


fn on_player(mut events: ReadUpdateEvent<Player>) {
    for event in events.read() {
        let test = event.new.clone();
        info!("Player inserted: {:?}", test);
    }
}