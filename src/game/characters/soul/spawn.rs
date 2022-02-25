use bevy::{  prelude::*, };
use heron::prelude::*;

use crate::game::components::{*, player_states::*, filters::*};
pub fn spawn(
    mut spawn_reader: EventReader<SpawnCharacter>,
    mut extend_writer: EventWriter<ExtendCharacter>,
    mut meshes: ResMut<super::Assets<Mesh>>,
    mut materials: ResMut<super::Assets<StandardMaterial>>,
    mut commands: Commands
) {

    for spawn in spawn_reader.iter() {
        if spawn.0 == "Soul"{
            let id = spawn.1;
            let team = spawn.2;
            let entity_id = commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube { size: 1. })),
                material: materials.add(StandardMaterial {
                    base_color: Color::rgba(0.7, 0.2, 0.3, 0.5),
                    ..Default::default()
                }),
                transform: Transform::from_xyz(id as f32 * 2.0, 0.5, 15.0),
                ..Default::default()
            })          
            .insert(Spawn{ respawn_coords: Vec3::new(id as f32 * 2.0, 0.5, 15.0)})
            .insert_bundle(
                States{
                    character_name: CharName("Soul"),
                    team: Team(team as i16),
                    id: Id(id),
                    hor_vel: Speed(4.4),
                    hp: Hp(1000),
                    ..Default::default()
                    
                }
            )
            .insert(Control{
               //lmb: true,
                ..Default::default()})
            .insert(SoulFilter)
            .insert(Core)
            .insert(CollisionShape::Cuboid { half_extends: Vec3::new(0.5,0.5,0.5) , border_radius: None})
            .id();
            extend_writer.send(ExtendCharacter(entity_id, id, team));
        }     
    }
}
