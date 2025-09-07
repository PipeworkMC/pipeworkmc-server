use bevy_ecs::component::Component;


#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, Component)]
#[repr(u8)]
pub enum GameMode { // TODO: Detect changes and set player game mode.
    Survival  = 0,
    Creative  = 1,
    #[default]
    Adventure = 2,
    Spectator = 3
}
