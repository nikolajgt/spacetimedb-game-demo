use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHash, PasswordHasher, PasswordVerifier, Version};
use log::{info, warn};
use once_cell::sync::Lazy;
use spacetimedb::*;
use spacetimedb::rand::{Rng, RngCore};
use spacetimedb::rand::rngs::OsRng;



#[table(name = user_accounts, public)]
pub struct UserAccount {
    #[primary_key]
    id: u128,
    #[unique]
    email: String,
    password_hash: String,
}

#[table(name = identity_bindings, public)]
pub struct IdentityBinding {
    #[primary_key]
    identity: Identity,
    user_id: u128,
    character_id: Option<u128>,
}


#[table(name = characters, public)]
pub struct Character {
    #[primary_key]
    character_id: u128,
    #[unique]
    user_id: u128,
    #[unique]
    name: String,
    level: u32,
    online: bool,
}

#[table(name = character_movement, public)]
pub struct CharacterMovementState {
    #[primary_key]
    pub id: u128,

    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,

    pub dir_x: f32,
    pub dir_y: f32,
    pub dir_z: f32,

    pub mode: u8,
    pub direction: u8,
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
    let sender = ctx.sender.clone();
    if let Some(binding) = ctx.db().identity_bindings().identity().find(&sender) {
        if let Some(mut character) = ctx.db().characters().character_id().find(binding.user_id) {
            character.online = false;
            ctx.db().characters().user_id().update(character);
        }

        ctx.db().identity_bindings().identity().delete(sender);
    } else {
        warn!("Disconnected identity {:?} had no binding", ctx.sender);
    }
}

// 
// #[reducer]
// pub fn login(ctx: &ReducerContext, email: String, password: String) {
//     let user = ctx.db().user_accounts().email().find(email.clone());
//     if let Some(user) = user {
//         if verify_password(&user.password_hash, &password) {
//             ctx.db().identity_bindings().insert(IdentityBinding {
//                 identity: ctx.sender.clone(),
//                 user_id: user.id.clone(),
//                 character_id: None,
//             });
// 
//             info!("Login successful");
//         }
//     }
// }
// 




#[reducer]
pub fn create_character(ctx: &ReducerContext, char_name: String) {
    let identity = ctx.db().identity_bindings().identity();
    if let Some(binding) = identity.find(ctx.sender.clone()) {
        if ctx.db().characters().name().find(char_name.clone()).is_some() {
            return;
        }
        let id = generate_id(ctx);
        ctx.db().characters().insert(Character {
            character_id: id.clone(),
            user_id: binding.user_id,
            name: char_name,
            level: 1,
            online: true,
        });

        info!("user {} created {} ", id, binding.user_id);
    }
}

#[reducer]
pub fn select_character(
    ctx: &ReducerContext,
    character_id: u128
) {
    if let Some(mut binding) = ctx.db().identity_bindings().identity().find(ctx.sender.clone()) {
        if let Some(character) = ctx.db().characters().character_id().find(binding.user_id) {
            if character.online {
                warn!("Another character is already online for this account");
                return;
            }
        }
        if let Some(mut selected) = ctx.db().characters().user_id().find(&character_id) {
            info!("User {} selected character {}", &character_id, &selected.character_id);
            selected.online = true;
            binding.character_id = Some(character_id);
            ctx.db.identity_bindings().identity().update(binding);
            ctx.db().characters().user_id().update(selected);
        }
    }
}

fn generate_id(ctx: &ReducerContext) -> u128 {
    let rand = ctx.rng().gen::<u128>();
    let sender_hash = fxhash::hash64(&ctx.sender.to_u256()) as u128;
    (rand << 64) | sender_hash
}
