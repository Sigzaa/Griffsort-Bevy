use crate::{id_manager::IdManager, *};

use super::body_parts::{body, camera, head};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn insert_body<T: Component>(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
    query: Query<(Entity, Option<&Transform>, Option<&Id>), Added<T>>,
) {
    for (entity, transform, _id) in query.iter()
    {
        let head = head(&mut commands, &mut meshes, &mut materials);

        let camera = camera(&mut commands);

        let body = body(&mut commands, &mut meshes, &mut materials, entity.clone());

        let mut ent_com = commands.entity(body);

        ent_com
            .push_children(&[head])
            .insert(HeadLink(head))
            .insert(CameraLink(camera));

        //let arr:[Option<(dyn Component + 'static)>; 2] = [transform, id];

        //for component in arr{ }

        if let Some(transform) = transform
        {
            ent_com.insert(*transform);
        }

        commands.entity(head).push_children(&[camera]);
    }
}
pub fn insert_physics<T: Component>(
    mut commands: Commands,
    query: Query<(Entity, Option<&Transform>), Added<T>>,
) {
    for (entity, _transform) in query.iter()
    {
        commands
            .entity(entity)
            .insert_bundle(bevy_mod_picking::PickableBundle::default())
            .insert(Collider::capsule(
                Vec3::new(0., -0.4, 0.),
                Vec3::new(0., 0.4, 0.),
                0.4,
            ))
            .insert(Velocity::default())
            .insert(ExternalForce::default())
            .insert(ExternalImpulse::default())
            .insert(GravityScale(2.))
            .insert(LockedAxes::ROTATION_LOCKED)
            .insert(Damping {
                linear_damping: 1.5,
                angular_damping: 0.,
            })
            .insert(ColliderMassProperties::Density(1.0))
            .insert(Friction {
                coefficient: 1.,
                combine_rule: CoefficientCombineRule::Min,
            })
            .insert(RigidBody::Dynamic);
    }
}
pub fn insert_rest<T: Component, Conf: ConfigProps + Send + Sync + 'static>(
    mut commands: Commands,
    query: Query<(Entity, Option<&Team>, Option<&Id>), Added<T>>,
    mut idm: ResMut<IdManager>,
    conf: Res<Conf>,
) {
    for (entity, team, id) in query.iter()
    {
        let mut ec = commands.entity(entity);

        ec.insert(Hero)
            .insert(ShapeIntersections::default())
            .insert(RayPointingOn::default())
            
            .insert_bundle(DynamicHeroesBundle {
                hp: Hp(conf.props().max_hp),
                ..Default::default()
            })

            .insert_bundle(StaticHeroesBundle {
                id: idm.alloc_id(),
                team: Team::Dark,
                ..Default::default()
            });
            
        if let Some(team) = team
        {
            ec.insert(team.clone());
        };
        if let Some(id) = id
        {
            ec.insert(id.clone());
        };
    }
}
