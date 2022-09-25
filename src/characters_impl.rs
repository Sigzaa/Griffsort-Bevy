mod default;
pub mod jacqueline;
pub mod soul;
pub mod tamara;
pub mod yui_shang;
pub mod zero;

use actions::Actions;
use bevy::prelude::*;
use bevy_inspector_egui::InspectorPlugin;
use heroes::*;
pub use heroes_structs::*;
use synx::Synx;

use crate::{commands_impl, Action, heroes_mapping::spawn_hero};

use self::{soul::SoulConfig, jacqueline::resources::JacquelineConfig, yui_shang::resources::ShangConfig};

pub struct Characters;
impl Plugin for Characters {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SpawnHeroEv>()
            .add_event::<DespawnHeroEv>()

            .add_plugin(InspectorPlugin::<HeroesConfig>::new())
            .add_plugin(Synx::<HeroesConfig>::new("./config/heroes.ron"))

            .add_plugin(Controller1::<Jacqueline, JacquelineConfig>::new("./config/jacqueline.ron", Jacqueline))
            //.add_plugin(Controller1::<Zero, Zero>::new(Zero))
            .add_plugin(Controller1::<YuiShang, ShangConfig>::new("./config/yui_shang.ron", YuiShang))
            //.add_plugin(Controller::<Jacqueline>::new(Jacqueline))
            .add_plugin(Controller1::<Soul, SoulConfig>::new("./config/soul.ron", Soul))

            .add_system(shared)
            .add_system(spawn)
            .add_system(despawn);
    }
}

pub fn spawn(
    mut spawn_ev: EventReader<SpawnHeroEv>,
    mut commands: Commands,
){
    for props in spawn_ev.iter()
    {
        let mut ec = commands
        .spawn_bundle(TransformBundle::from_transform(Transform::from_translation(props.position)));

        spawn_hero(props.name.clone(), ec);
    }

}
pub fn despawn(
    mut despawn_ev: EventReader<DespawnHeroEv>,
    mut idm: ResMut<IdManager>,
    mut commands: Commands,
){
    for props in despawn_ev.iter()
    {
        commands.entity(props.entity).despawn();

        idm.remove_id(props.id);
    }
}

pub fn shared(
    q: Query<Entity, Added<Hero>>,
    mut commands: Commands
){
    for ent in &q{
        commands.entity(ent)
        .insert(Actions::<Action>::new())
        .insert(CollisionGroups::new(0b01, 0b110));
    }
}


impl SpawnHeroEv {
    pub fn new(name: String, team: Team, position: Vec3) -> Self {
        Self{
            name, 
            team, 
            position,
        }
    }
}

impl DespawnHeroEv {
    pub fn new(entity: Entity, id: i32) -> Self {
        Self{
            entity,
            id,
        }
    }
}


pub mod heroes_structs {

    use super::*;

    pub struct SpawnHeroEv{

        pub name: String,
        pub team: Team,
        pub position: Vec3,
    }
    pub struct DespawnHeroEv{
        pub entity: Entity,
        pub id: i32
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
