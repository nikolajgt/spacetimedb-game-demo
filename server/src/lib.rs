use log::warn;
use spacetimedb::*;
use spacetimedb::sats::AlgebraicType;
use spacetimedb::sats::typespace::TypespaceBuilder;


#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    id: Identity,
    name: Option<String>,
    online: bool,
}

#[table(name = player_movement, public)]
pub struct PlayerMovementState {
    #[primary_key]
    pub player_id: Identity,

    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    pub dir_x: f32,
    pub dir_y: f32,
    pub dir_z: f32,

    pub mode: u8,
    pub direction: u8,
}

#[reducer]
pub fn register_player(ctx: &ReducerContext, name: String) {
    if let Some(mut player) = ctx.db().player().id().find(ctx.sender) {
        player.online = true;
        ctx.db().player().id().update(player);
    } else {
        ctx.db().player().insert(Player {
            id: ctx.sender.clone(),
            name: Some(name),
            online: true,
        });
    }
}



#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    // Called when the module is initially published
    log::info!("Initializing module");
}

#[reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {
    // Called everytime a new client connects
    log::info!("Identity connected");
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    if let Some(mut player) = ctx.db().player().id().find(ctx.sender) {
        player.online = false;
        ctx.db().player().id().update(player);
    } else {
        warn!("Disconnected player was not found to set offline")
    }
}

