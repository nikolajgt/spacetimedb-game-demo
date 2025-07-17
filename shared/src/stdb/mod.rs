use spacetimedb::{table, Identity};

// should not store offline data
#[table(name = characters, public)]
pub struct Character {
    #[primary_key]
    pub identity: Identity,

    #[unique]
    pub character_id: u128,

    #[unique]
    pub name: String,
    pub level: u32,
}
#[table(name = test_table, public)]
pub struct TestTable {
    //  pub identity: Identity,
    pub name: String
}

// should not store offline data
#[table(name = character_movement, public)]
pub struct CharacterMovement {
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


#[table(name = character_movement_commands)]
pub struct CharacterMovementCommands {
    #[primary_key]
    #[auto_inc]
    pub id:  u128,

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