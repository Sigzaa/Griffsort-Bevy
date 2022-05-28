use bevy::{prelude::*};
use crate::shared::resources::*;
use super::resources::ToExtend;

pub fn master(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    q_core: Query<(Entity, &Id, &Team), (With<Core>, With<ToExtend>)>,
    //mut snapshot_id_provider: ResMut<SnapshotIdProvider<SnapShot>>
) {
    for (ent, id, team) in q_core.iter() {
        let id = id.0;
        let team = team.0;


        let core = commands
            .entity(ent)
            //.insert(snapshot_id_provider.next())
            //.insert(RigidBody::Dynamic)
            //.insert(RotationConstraints::lock())
            .remove::<ToExtend>()
            .insert(GoRot::default())
            .id();

        let head = commands
            .spawn()
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.5 })),
                material: materials.add(StandardMaterial {
                    //base_color: Color::BLACK,
                    ..Default::default()
                }),
                transform: Transform::from_xyz(0.0, 1.2, -0.5),
                ..Default::default()
            })
            .insert(Head)
            .insert(Team(team as i16))
            .id();
        commands.entity(core).push_children(&[head]);
    }
}
