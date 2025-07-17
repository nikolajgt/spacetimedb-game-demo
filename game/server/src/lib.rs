mod character_logic;

use argon2::{Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use log::{info, warn};
use spacetimedb::*;

use shared::stdb::*;





#[reducer(init)]
pub fn init(_ctx: &ReducerContext) -> Result<(), String>{
    let loop_duration: TimeDuration = TimeDuration::from_micros(50_000);
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
    // should flush data 
}


#[table(name = tick_schedule, scheduled(schedule_tick))]
pub struct TickSchedule {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: ScheduleAt,
}

#[reducer]
pub fn schedule_tick(ctx: &ReducerContext, args: TickSchedule) {
    process_movement_commands(ctx)    
}



#[derive(SpacetimeType)]
pub struct MovementCommand {
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
pub fn move_character_command(ctx: &ReducerContext, cmd: MovementCommand) {
    let identity = ctx.sender;
    ctx.db.character_movement_commands().insert(CharacterMovementCommands {
        id: 0,
        identity,
        character_id: cmd.character_id,
        pos_x: cmd.pos_x,
        pos_y: cmd.pos_y,
        pos_z: cmd.pos_z,
        dir_x: cmd.dir_x,
        dir_y: cmd.dir_y,
        dir_z: cmd.dir_z,
        mode: cmd.mode,
    });
}

// logic heere
pub fn process_movement_commands(ctx: &ReducerContext) {
    for command in  ctx.db.character_movement_commands().iter() {

        ctx.db
            .character_movement()
            .identity()
            .insert_or_update(CharacterMovement {
                identity: ctx.identity(),
                character_id: command.character_id,
                pos_x: command.pos_x,
                pos_y: command.pos_y,
                pos_z: command.pos_z,
                dir_x: command.dir_x,
                dir_y: command.dir_y,
                dir_z: command.dir_z,
                mode: command.mode
            });

        ctx.db
            .character_movement_commands()
            .id()
            .delete(command.id);
    }
}







