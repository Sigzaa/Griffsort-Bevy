use actions::Actions;
use bevy::prelude::*;
use heroes::*;
use keyframe::{ease, functions::*};

use crate::{characters_impl::Jacqueline, resources::Action};

use super::resources::*;

pub struct ChaseAbil;

impl Plugin for ChaseAbil {
    fn build(&self, app: &mut App) {
        app.add_system(idle_to_chase)
            .add_system(move_marks)
            .add_system(chase)
            .add_system(detect_jump)
            .add_system(move_jacqueline);
    }
}

fn detect_jump(
    rapier_context: Res<RapierContext>,
    mut jacquelineq: Query<(&CameraLink, &Actions<Action>, Entity, &MarksLinks), With<Jacqueline>>,
    cameraq: Query<&GlobalTransform>,
    mut markq: Query<(&mut MarkState, Entity), Without<Hero>>,
    mut commands: Commands,
    conf: Res<JacquelineConfig>,
) {
    for (camera_link, actions, jacqueline_entity, marks_links) in &mut jacquelineq
    {
        //Detecting inputs
        if !actions.pressed(Action::Sprint)
        {
            return;
        }

        let global_transform = cameraq.get(camera_link.0).unwrap();
        let (_, c_rotation, _) = global_transform.to_scale_rotation_translation();

        let props = conf.props().intersections_shape.clone();

        rapier_context.intersections_with_shape(
            global_transform.translation()
                + global_transform.forward() * (props.source_distance + props.toi),
            c_rotation,
            &Collider::cylinder(10., 3.),
            QueryFilter::new(),
            |entity| {
                //

                if let Ok((mark_state, mark_entity)) = markq.get_mut(entity)
                {
                    if marks_links.0.contains(&mark_entity)
                    {
                        if let MarkState::ReadyToJump(_chased_entity) = *mark_state
                        {
                            println!("ent {entity:?}");

                            commands
                                .entity(jacqueline_entity)
                                .insert(JumpingTo(mark_entity))
                                .insert(RigidBody::Fixed);

                            // vel.angvel = Vec3::ZERO;
                            // vel.linvel = Vec3::ZERO;
                            // coll_groups.memberships = Group::NONE;

                            return false;
                        }
                    }
                }
                true
            },
        );

        println!();
    }
}

fn move_jacqueline(
    mut jacq: Query<(Entity, &mut Transform, &JumpingTo, &mut MarksLinks), Without<MarkState>>,
    target: Query<(Entity, &Transform), With<MarkState>>,
    mut commands: Commands,
) {
    for (jac_ent, mut jac_transform, jumping_to, mut marks_links) in &mut jacq
    {
        if let Ok((mark_entity, to_mark_transform)) = target.get(jumping_to.0)
        {
            if marks_links.0.contains(&mark_entity)
            {
                for i in 0..3
                {
                    jac_transform.translation[i] = ease(
                        EaseInOutQuad,
                        jac_transform.translation[i],
                        to_mark_transform.translation[i],
                        0.1,
                    );

                    if to_mark_transform
                        .translation
                        .distance(jac_transform.translation)
                        < 0.5
                    {
                        commands.entity(jac_ent).insert(RigidBody::Dynamic);
                        commands.entity(mark_entity).despawn_recursive();
                        marks_links.0.remove(&mark_entity);
                    }
                }
            }
        }
    }
}

fn move_marks(
    mut markq: Query<(&mut MarkState, &mut Transform), Without<Hero>>,
    enemyq: Query<&Transform, With<Hero>>,
) {
    for (mut mark_state, mark_transform) in &mut markq
    {
        if let MarkState::Chasing(entity) = *mark_state
        {
            let target_transform = enemyq.get(entity).unwrap();

            if target_transform
                .translation
                .distance(mark_transform.translation)
                < 5.
            {
                *mark_state = MarkState::ReadyToJump(entity);
            }
        }
    }
}

fn chase(
    mut markq: Query<(&mut MarkState, &mut Transform), Without<Hero>>,
    enemyq: Query<&Transform, With<Hero>>,
) {
    for (mark_state, mut mark_transform) in &mut markq
    {
        if let MarkState::Chasing(entity) = *mark_state
        {
            let target_transform = enemyq.get(entity).unwrap();

            for i in 0..3
            {
                mark_transform.translation[i] = ease(
                    EaseInOutQuad,
                    mark_transform.translation[i],
                    target_transform.translation[i],
                    0.1,
                );
            }
        }
    }
}

fn idle_to_chase(
    mut query: Query<
        (
            &MarksLinks,
            &Actions<Action>,
            &mut MarksCD,
            &RayPointingOn,
            &Team,
        ),
        With<Jacqueline>,
    >,
    aiming_on_q: Query<&Team, With<Hero>>,
    mut markq: Query<&mut MarkState, Without<Jacqueline>>,
) {
    for (links, act, mut mcd, pointing_on, team) in query.iter_mut()
    {
        if let Some((entity, _toi)) = pointing_on.0
        {
            if let Ok(aiming_on_team) = aiming_on_q.get(entity)
            {
                // Detecting inputs
                if !act.pressed(Action::Sprint)
                {
                    return;
                }
                if !mcd.is_ready(0)
                {
                    return;
                }
                if aiming_on_team == team
                {
                    // Uncomment if you want Jacqueline not to teleport to teammates
                    //return;
                }

                // Time between use of marks
                mcd.cooldown(0, 0.5);

                mcd.add(-1);
                // Time to respawn mark
                mcd.cooldown(1, 2.);

                for mark in links.0.iter()
                {
                    let state = markq.get_mut(*mark).unwrap();

                    if let MarkState::Chasing(chasing_entity) = *state
                    {
                        // We cant send marks to same character
                        if chasing_entity == entity
                        {
                            return;
                        }
                    }
                    if let MarkState::ReadyToJump(chasing_entity) = *state
                    {
                        // We cant send marks to same character
                        if chasing_entity == entity
                        {
                            return;
                        }
                    }
                }
                for mark in links.0.iter()
                {
                    let mut state = markq.get_mut(*mark).unwrap();

                    if let MarkState::Idle(_angle) = *state
                    {
                        // Changing a state of mark
                        *state = MarkState::Chasing(entity);
                        return;
                    }
                }
            }
        }
    }
}
