use actions::Actions;
use bevy::prelude::*;
use heroes::*;
use keyframe::{ease, functions::*};
use std::collections::HashSet;

use crate::{characters_impl::Jacqueline, resources::Action};

use super::resources::*;

pub struct ChaseAbil;

impl Plugin for ChaseAbil {
    fn build(&self, app: &mut App) {
        app.add_system(idle_to_chase)
            .add_system(move_marks)
            .add_system(chase)
            .add_system(detect_jump)
            .add_system(expire_marks)
            .add_system(move_jacqueline);
    }
}

fn detect_jump(
    rapier_context: Res<RapierContext>,
    mut jacquelineq: Query<(&CameraLink, &Actions<Action>, Entity, &MarksLinks, &mut MarksCD), With<Jacqueline>>,
    cameraq: Query<&GlobalTransform>,
    mut markq: Query<&mut MarkState, Without<Hero>>,
    mut commands: Commands,
    conf: Res<JacquelineConfig>,
) {
    for (camera_link, actions, jacqueline_entity, marks_links, mut mcd) in &mut jacquelineq
    {
        //Detecting inputs
        if !actions.just_released(Action::Sprint)
        {
           continue;
        }
        if !mcd.is_ready(0)
        {
            continue;
        }

        let global_transform = cameraq.get(camera_link.0).unwrap();
        let (_, c_rotation, _) = global_transform.to_scale_rotation_translation();

        let props = conf.props().intersections_shape.clone();

        rapier_context.intersections_with_shape(
            global_transform.translation()
                + global_transform.forward() * (props.source_distance),
            c_rotation,
            &Collider::cuboid(props.radius, props.radius, props.toi),
            QueryFilter::new(),
            |entity| {
                //println!("ent {entity:?}");
                if marks_links.0.contains(&entity)
                {
                    let mut changed = false;
                    if let Ok(mark_state) = markq.get_mut(entity){
                        if let MarkState::ReadyToJump
                        {
                            target: chased_entity,
                            expire_time: exp_time,

                        } = *mark_state{



                            commands
                                .entity(jacqueline_entity)
                                .insert(JumpingTo(entity))
                                .insert(RigidBody::Fixed);

                            // Prevent looping of marks teleporting
                            if chased_entity.is_some(){
                                changed = true;
                            }
                        }
                    }
                    //
                    if changed {

                        for mark_link in marks_links.0.iter()
                        {


                            let mut mark_state = markq.get_mut(*mark_link).unwrap();


                            if let MarkState::Idle(_angle) = *mark_state {
                                    *mark_state = MarkState::ReadyToJump{
                                        target: None,
                                        expire_time: 10.,

                                    };

                                    mcd.add(-1);
                                    // Time to respawn mark
                                    mcd.cooldown(1, 2.);
                                    break;
                            }
                        }
                        return false;
                    
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
    mark_target: Query<(Entity, &Transform), With<MarkState>>,
    mut commands: Commands,
) {
    for (jac_ent, mut jac_transform, jumping_to, mut marks_links) in &mut jacq
    {
        if let Ok((mark_entity, to_mark_transform)) = mark_target.get(jumping_to.0)
        {
            if marks_links.0.contains(&mark_entity)
            {
                for i in 0..3
                {
                    jac_transform.translation[i] = ease(
                        EaseInOutQuad,
                        jac_transform.translation[i],
                        to_mark_transform.translation[i],
                        0.3,
                    );

                    if to_mark_transform
                        .translation
                        .distance(jac_transform.translation)
                        < 1.5
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
                *mark_state = MarkState::ReadyToJump{
                    target: Some(entity),
                    expire_time: 10.,
                };
            }
        }
    }
}
fn expire_marks(
    mut jacq: Query<(Entity, &mut MarksLinks), Without<MarkState>>,
    mut markq: Query<(&mut MarkState, Entity), Without<Hero>>,
    time: Res<Time>,
    mut commands: Commands,
    )
{
    for (ent, mut marks_links) in &mut jacq{

        let mut to_remove = HashSet::new();

        for mut mark_link in marks_links.0.iter(){

            if let Ok((mut mark_state, mark_entity)) = markq.get_mut(*mark_link){

            if let MarkState::ReadyToJump{target: target_entity, expire_time: mut exp_time} = *mark_state{
                exp_time -= time.delta_seconds();

                *mark_state = MarkState::ReadyToJump{target: target_entity, expire_time: exp_time};

                if exp_time <= 0.{
                    to_remove.insert(mark_entity);
                    commands.entity(mark_entity).despawn_recursive();
                }

            }
            }
        }
        for mark_ent in to_remove
        {
            marks_links.0.remove(&mark_ent);
        }

    }

}

fn chase(
    mut markq: Query<(&mut MarkState, &mut Transform, Entity), Without<Hero>>,
    enemyq: Query<&Transform, With<Hero>>,

) {
    for (mut mark_state, mut mark_transform, mark_entity) in &mut markq
    {
        if let MarkState::Chasing(target_entity) = *mark_state
        {
            let target_transform = enemyq.get(target_entity).unwrap();

            let to = target_transform.translation + target_transform.back() * 1.5;

            for i in 0..3
            {
                mark_transform.translation[i] = ease(
                    EaseInOutQuad,
                    mark_transform.translation[i],
                    to[i],
                    0.1,
                );
            }
        }
        if let MarkState::ReadyToJump{target: Some(target_entity), expire_time: mut exp_time}= *mark_state
        {
            let target_transform = enemyq.get(target_entity).unwrap();




            let to = target_transform.translation + target_transform.back() * 1.5;
            
            for i in 0..3
            {
                mark_transform.translation[i] = ease(
                    EaseInOutQuad,
                    mark_transform.translation[i],
                    to[i],
                    0.05,
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
                    // return;
                }

                for mark in links.0.iter()
                {
                    // We cant doublesend marks to same character
                    if match *markq.get_mut(*mark).unwrap()
                    {
                        MarkState::Chasing(chased_entity) => Some(chased_entity),
                        MarkState::ReadyToJump{
                            target: chased_entity,
                            expire_time: exp_time,
                        } => chased_entity,
                        _=> None,
                        
                    } == Some(entity)
                    {
                        return;  
                    }

                }
                for mark in links.0.iter()
                {
                    let mut state = markq.get_mut(*mark).unwrap();

                    if let MarkState::Idle(_angle) = *state
                    {

                        // Time between use of marks
                        mcd.cooldown(0, 0.5);

                        mcd.add(-1);
                        // Time to respawn mark
                        mcd.cooldown(1, 2.);

                        // Changing a state of mark
                        *state = MarkState::Chasing(entity);
                        return;
                    }
                }
            }
        }
    }
}
