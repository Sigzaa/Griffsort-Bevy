mod default;
pub mod jacqueline;
pub mod soul;
pub mod tamara;
pub mod yui_shang;
pub mod zero;

use actions::Actions;
use bevy::prelude::*;
use heroes::*;
pub use heroes_structs::*;

use crate::{heroes_mapping::spawn_hero, Action};

use self::{
    jacqueline::resources::JacquelineConfig, soul::SoulConfig, yui_shang::resources::ShangConfig,
};

pub struct Characters;
impl Plugin for Characters {
    fn build(&self, app: &mut App) {
        app
            // Additional Events
            .add_event::<SpawnHeroEv>()
            .add_event::<DespawnHeroEv>()
            // Handling this Events
            .add_system(spawn)
            .add_system(despawn)
            // Extends with every hero
            .add_system(shared)
            // Adding heroes implementations:
            .add_plugin(Controller1::<Jacqueline, JacquelineConfig>::new(
                "./config/jacqueline.ron",
                Jacqueline,
            ))
            .add_plugin(Controller1::<YuiShang, ShangConfig>::new(
                "./config/yui_shang.ron",
                YuiShang,
            ))
            .add_plugin(Controller1::<Soul, SoulConfig>::new(
                "./config/soul.ron",
                Soul,
            ));
    }
}

pub fn spawn(mut spawn_ev: EventReader<SpawnHeroEv>, mut commands: Commands) {
    for props in spawn_ev.iter()
    {
        let mut ec = commands.spawn_bundle(TransformBundle::from_transform(
            Transform::from_translation(props.position),
        ));
        ec.insert(props.team.clone());

        spawn_hero(props.name.clone(), ec);
    }
}
pub fn despawn(
    mut despawn_ev: EventReader<DespawnHeroEv>,
    mut idm: ResMut<IdManager>,
    mut commands: Commands,
) {
    for props in despawn_ev.iter()
    {
        commands.entity(props.entity).despawn();

        idm.remove_id(props.id);
    }
}

pub fn shared(q: Query<Entity, Added<Hero>>, mut commands: Commands) {
    for ent in &q
    {
        commands
            .entity(ent)
            .insert(Actions::<Action>::new())
            .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1))
            .insert(Velocity::default());
    }
}

impl SpawnHeroEv {
    pub fn new(name: String, team: Team, position: Vec3) -> Self {
        Self {
            name,
            team,
            position,
        }
    }
}

#[allow(dead_code)]
impl DespawnHeroEv {
    pub fn new(entity: Entity, id: i32) -> Self {
        // Todo
        Self { entity, id }
    }
}

pub mod heroes_structs {

    use super::*;

    pub struct SpawnHeroEv {
        pub name: String,
        pub team: Team,
        pub position: Vec3,
    }
    pub struct DespawnHeroEv {
        pub entity: Entity,
        pub id: i32,
    }

    #[derive(Copy, Clone, Component)]
    pub struct Soul;

    #[derive(Copy, Clone, Component)]
    pub struct Tamara;

    #[derive(Copy, Clone, Component)]
    pub struct Zero;

    #[derive(Copy, Clone, Component)]
    pub struct YuiShang;

    #[derive(Copy, Clone, Component)]
    pub struct Jacqueline;
}
