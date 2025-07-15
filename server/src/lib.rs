mod character_logic;

use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use log::{info, warn};
use spacetimedb::*;
use spacetimedb::rand::Rng;






#[reducer(init)]
pub fn init(_ctx: &ReducerContext) -> Result<(), String>{
    let loop_duration: TimeDuration = TimeDuration::from_micros(10_000_000);
    _ctx.db.tick_schedule().insert(TickSchedule {
        scheduled_id: 0,
        scheduled_at: loop_duration.into()
    });
    Ok(())
}

#[reducer(client_connected)]
pub fn identity_connected(_ctx: &ReducerContext) {
    // Called everytime a new client connects
    log::info!("Identity connected");
}

#[reducer(client_disconnected)]
pub fn identity_disconnected(ctx: &ReducerContext) {
    move_session_to_persistence(ctx);
}


#[table(name = tick_schedule, scheduled(tick))]
pub struct TickSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: ScheduleAt,
}
#[reducer]
pub fn tick(ctx: &ReducerContext, args: TickSchedule) {
    info!("Tick hit");
}


#[reducer]
pub fn schedule_tick(ctx: &ReducerContext) {
    info!("Schedule tick hit");
}

#[table(name = characters)]
pub struct Character {
    #[primary_key]
    pub character_id: u128,

    #[unique]
    pub identity: Identity,

    #[unique]
    pub name: String,
    pub level: u32,
    pub online: bool,
}

#[table(name = character_movement)]
pub struct CharacterMovement {
    #[primary_key]
    pub character_id: u128,

    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    pub dir_x: f32,
    pub dir_y: f32,
    pub dir_z: f32,

    pub mode: u8,
}


#[table(name = session_characters, public)]
pub struct SessionCharacter {
    #[primary_key]
    pub identity: Identity,
    pub character_id: u128,
}

#[table(name = session_character_movement, public)]
pub struct SessionCharacterMovement {
    #[primary_key]
    pub identity: Identity,
    #[unique]
    pub character_id: u128,

    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    pub dir_x: f32,
    pub dir_y: f32,
    pub dir_z: f32,

    pub mode: u8,
}


#[reducer]
pub fn create_character(ctx: &ReducerContext, char_name: String) {
    let identity = ctx.sender;
    if ctx.db().characters().name().find(char_name.clone()).is_some() {
        return;
    }

    let id = generate_id(ctx);
    let char = Character {
        character_id: id.clone(),
        identity: identity,
        name: char_name,
        level: 1,
        online: true,
    };

    info!("Identity {} created {} ", identity, char.name);
    ctx.db().characters().insert(char);
}


#[reducer]
pub fn select_character(
    ctx: &ReducerContext,
    character_id: u128
) {
    let identity = ctx.sender;
    let Some(character) = ctx.db.characters()
        .identity()
        .find(identity) else {
        warn!("Character with id {} were not found by identity: {}", character_id, identity);
        return;
    };

    ctx.db.session_characters().insert(SessionCharacter {
        character_id: character.character_id,
        identity: identity
    });

    if let Some(movement) = ctx.db.character_movement().character_id().find(character_id) {
        ctx.db.session_character_movement().insert(SessionCharacterMovement {
            character_id: character.character_id,
            identity: identity,
            pos_x: movement.pos_x,
            pos_y: movement.pos_y,
            pos_z: movement.pos_z,

            dir_x: movement.dir_x,
            dir_y: movement.dir_y,
            dir_z: movement.dir_z,

            mode: movement.mode,
        });
    }
}

fn generate_id(ctx: &ReducerContext) -> u128 {
    let rand = ctx.rng().gen::<u128>();
    let sender_hash = fxhash::hash64(&ctx.sender.to_u256()) as u128;
    (rand << 64) | sender_hash
}


pub fn move_session_to_persistence(ctx: &ReducerContext) {
    let identity = ctx.identity();
    let Some(selected_character) = ctx.db.session_characters().identity().find(&identity) else {
        warn!("selected character {} was not found in session characters",  identity);
        return;
        return;
    };
    // move character
    if let Some(mut character) = ctx.db.characters().character_id().find(&selected_character.character_id) {
        character.online = false;
        ctx.db().characters().character_id().update(character);
        ctx.db().session_characters().identity().delete(selected_character.identity);
    }

    // move character movement
    if let Some(movement_session) = ctx.db.session_character_movement().identity().find(identity) {
        ctx.db.character_movement().insert(CharacterMovement {
            character_id: movement_session.character_id,
            pos_x: movement_session.pos_x,
            pos_y: movement_session.pos_y,
            pos_z: movement_session.pos_z,

            dir_x: movement_session.dir_x,
            dir_y: movement_session.dir_y,
            dir_z: movement_session.dir_z,

            mode: movement_session.mode,
        });
    }

    // move other things
}


pub fn move_persistence_to_session(ctx: &ReducerContext, character_id: u128 ) {
    let identity = ctx.identity();
    let Some(character) = ctx.db.characters()
        .identity()
        .find(identity)
    else {
        warn!("Character with id {} were not found by identity: {}", character_id, identity);
        return;
    };


    ctx.db.session_characters().insert(SessionCharacter {
        character_id: character.character_id,
        identity: identity
    });

    if let Some(movement) = ctx.db.character_movement().character_id().find(character_id) {
        ctx.db.session_character_movement().insert(SessionCharacterMovement {
            character_id: character.character_id,
            identity: identity,
            pos_x: movement.pos_x,
            pos_y: movement.pos_y,
            pos_z: movement.pos_z,

            dir_x: movement.dir_x,
            dir_y: movement.dir_y,
            dir_z: movement.dir_z,

            mode: movement.mode,
        });
    }
}