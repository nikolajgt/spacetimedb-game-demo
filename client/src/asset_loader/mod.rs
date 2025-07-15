use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use crate::GameState;

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::LoadingAssets)
                    .continue_to_state(GameState::AssetsLoaded)
                    .load_collection::<PlayerModelsAssets>()
                    .load_collection::<ModularCharacterParts>()
                    .load_collection::<PlayerAnimationsClips>()
            );
    }
}


#[derive(AssetCollection, Resource)]
pub struct PlayerModelsAssets {
    #[asset(
        paths("main_skeleton.glb", "Soldier.gltf", "Witch.gltf", "Adventurer.gltf", "SciFi.gltf"),
        collection(typed, mapped)
    )]
    pub models: HashMap<String, Handle<Gltf>>,

    #[asset(
        paths("main_skeleton.glb", "scifi_torso.glb", "witch_legs.glb"),
        collection(typed, mapped)
    )]
    pub models_parts: HashMap<String, Handle<Gltf>>,

    #[asset(
        paths("sword.glb"),
        collection(typed, mapped)
    )]
    pub items: HashMap<String, Handle<Gltf>>,
}
#[derive(AssetCollection, Resource)]
pub struct PlayerAnimationsClips {
    #[asset(path = "SciFi.gltf#Animation0")]  pub(crate) death: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation1")]  pub(crate) gun_shoot: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation2")]  pub(crate) hit_receive: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation3")]  pub(crate) hit_receive_2: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation4")]  pub(crate) idle: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation5")]  pub(crate) idle_gun: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation6")]  pub(crate) idle_gun_pointing: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation7")]  pub(crate) idle_gun_shoot: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation8")]  pub(crate) idle_neutral: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation9")]  pub(crate) idle_sword: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation10")] pub(crate) interact: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation11")] pub(crate) kick_left: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation12")] pub(crate) kick_right: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation13")] pub(crate) punch_left: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation14")] pub(crate)punch_right: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation15")] pub(crate) roll: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation16")] pub(crate)run: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation17")] pub(crate) run_back: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation18")] pub(crate) run_left: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation19")] pub(crate) run_right: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation20")] pub(crate) run_shoot: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation21")] pub(crate) sword_slash: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation22")] pub(crate) walk: Handle<AnimationClip>,
    #[asset(path = "SciFi.gltf#Animation23")] pub(crate) wave: Handle<AnimationClip>,
}

#[derive(AssetCollection, Resource)]
pub struct ModularCharacterParts {
    #[asset(
        paths(
            "Witch.gltf#Scene2",
            "SciFi.gltf#Scene2",
            "Soldier.gltf#Scene2",
            "Adventurer.gltf#Scene2"
        ),
        collection(typed, mapped)
    )]
    pub heads: HashMap<String, Handle<Scene>>,

    #[asset(
        paths(
            "Witch.gltf#Scene3",
            "SciFi.gltf#Scene3",
            "Soldier.gltf#Scene3",
            "Adventurer.gltf#Scene3",
            "scifi_torso.glb#Scene0"
        ),
        collection(typed, mapped)
    )]
    pub bodies: HashMap<String, Handle<Scene>>,

    #[asset(
        paths(
            "Witch.gltf#Scene4",
            "SciFi.gltf#Scene4",
            "Soldier.gltf#Scene4",
            "Adventurer.gltf#Scene4",
            "witch_legs.glb#Scene0"
        ),
        collection(typed, mapped)
    )]
    pub legs: HashMap<String, Handle<Scene>>,

    #[asset(
        paths(
            "Witch.gltf#Scene5",
            "SciFi.gltf#Scene5",
            "Soldier.gltf#Scene5",
            "Adventurer.gltf#Scene5"
        ),
        collection(typed, mapped)
    )]
    pub feet: HashMap<String, Handle<Scene>>,
}