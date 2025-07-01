use spacetimedb::*;
use spacetimedb::sats::AlgebraicType;
use spacetimedb::sats::typespace::TypespaceBuilder;


#[table(name = player, public)]
pub struct Player {
    #[primary_key]
    id: Identity,
    identity: Identity,
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
    if ctx.db().player().iter().any(|p| p.id == ctx.identity()) {
        log::warn!("Player {} was already registered", ctx.identity());
        return;
    }

    ctx.db().player().insert(Player {
        id: ctx.identity(),
        identity: ctx.identity(),
        name: Some(name),
        online: true,
    });

    log::info!("Player {} registered", ctx.identity());
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
    let identity = ctx.identity();
    let player = ctx.db().player().iter().find(|p| p.identity == identity);
    if let Some(mut player) = player {
        player.online = false;
        log::info!("Player {} disconnected", player.identity);
    }
}

