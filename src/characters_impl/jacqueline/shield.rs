use std::collections::HashSet;

use super::resources::*;
use crate::*;
use bevy::prelude::*;
use keyframe::{ease, functions::EaseInQuint};

// Converting mark states from Idle to Shield
// Handling inputs and cooldowns
pub fn idle_to_shield(
    mut query: Query<(&MarksLinks, &mut ShieldState, &Actions<Action>, &mut MarksCD), With<Jacqueline>>,
    mut markq: Query<&mut MarkState, Without<Jacqueline>>,
    mut evwr: EventWriter<RecalkAnglesEv>,
    mut commands: Commands,

) {
    for (links, mut shield_state, act, mut mcd) in query.iter_mut() {
        // Detecting inputs
        if !act.pressed(Action::Abil1) {
            return;
        }
        if !mcd.is_ready(0){
            return;
        }

        // Time between use of marks
        mcd.cooldown(0, 0.5);

        mcd.add(-1);
        // Time to respawn mark
        mcd.cooldown(1, 2.);

        // Using first free mark to become a shield
        for mark in links.0.iter() {
            let mut state = markq.get_mut(*mark).unwrap();

            if let MarkState::Idle(_angle) = *state {
                // Changing a state of mark
                *state = MarkState::Shield;

                // Adding expiration timer
                commands.entity(*mark).insert(MarkDespawnTimer {
                    timer: Timer::from_seconds(1., false),
                });

                // Sending Event to reallocate angles of marks in new state
                evwr.send(RecalkAnglesEv);

                // Returning because we need only one mark
                return;
            }
        }
    }
}

// Transforming Mark to Shield
pub fn mark_to_shield(
    mut query: Query<(&mut ShieldState, &mut MarksLinks, &Transform), With<Jacqueline>>,
    mut markq: Query<(&MarkState, &mut Transform, &mut MarkDespawnTimer), Without<Jacqueline>>,
    mut shieldq: Query<&mut Transform, (With<Shield>, Without<Jacqueline>, Without<MarkState>)>,
    time: Res<Time>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut shield_state, mut links, &transform) in query.iter_mut() {
        // Expecting position of the shield
        let mut final_transform = transform.clone();

        final_transform.translation += transform.forward() * 2.;

        // We will use it later
        let shield_exists = shield_state.link.is_some();

        // We have to move a shield at his proper position (if shield exists)
        if shield_exists {
            // Its safe
            let mut shield_transform = shieldq.get_mut(shield_state.link.unwrap()).unwrap();

            for i in 0..3 {
                shield_transform.translation[i] = ease(
                    EaseInQuint,
                    shield_transform.translation[i],
                    final_transform.translation[i],
                    0.5,
                );
            }
        }

        // We need this, to push there all marks which should be removed from MarksLinks, and then remove it
        // We can remove it in the loop because of ownership problems
        let mut to_remove = HashSet::new();

        // Iterating over marks to find marks with with shield state
        for mark in links.0.iter() {
            let res = markq.get_mut(*mark);

            // Skipping this iteration of loop if mark has non-shield state or doesnt has despawn timer
            if res.is_err() {
                continue;
            }

            // We are safe here to unwrap.
            let (state, mut mark_transform, mut desp_timer) = res.unwrap();

            if let MarkState::Shield = *state {
                desp_timer.timer.tick(time.delta());

                // We have to move mark to this position

                for i in 0..3 {
                    mark_transform.translation[i] = ease(
                        EaseInQuint,
                        mark_transform.translation[i],
                        final_transform.translation[i],
                        0.5,
                    );
                }

                if desp_timer.timer.just_finished() {
                    // Time to despawn a mark.

                    shield_state.duration += 1.;

                    if !shield_exists {
                        // Creating a shield if it does not exist

                        // Position on front of the character
                        let sh_ent = commands
                            .spawn_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Box::new(1., 1., 0.1))),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::rgba(0.2, 0.9, 0.1, 0.5),
                                    alpha_mode: AlphaMode::Blend,
                                    ..Default::default()
                                }),
                                transform: final_transform,
                                ..default()
                            })
                            .insert(Shield)
                            .id();

                        // Linking created entity to Jacqueline
                        // It helps us to access shield very easily
                        shield_state.link = Some(sh_ent);
                    }

                    /*
                    we cant do this:
                        links.0.remove(mark);

                    links are borrowed at the first loop
                    we must wait until the end of the loop

                    */
                    // Instead we do this:
                    to_remove.insert(*mark);

                    // Killing the mark;
                    commands.entity(*mark).despawn();
                }
            }
        }

        // Finaly removing marks from Links
        for mark_ent in to_remove {
            links.0.remove(&mark_ent);
        }
    }
}


pub fn shield_handler(
    mut query: Query<(&mut ShieldState, &MarksLinks, &Transform), With<Jacqueline>>,
    time: Res<Time>,
    mut commands: Commands,
) {
    for (mut shield_state, _links, _tr) in &mut query {

        if shield_state.link.is_none() {
            return;
        }
        let mark_ent = shield_state.link.unwrap();

        if shield_state.duration <= 0. {
            commands.entity(mark_ent).despawn();
            shield_state.link = None;
        } else {
            shield_state.duration -= time.delta_seconds();
        }
    }
}

pub fn shield_follow_hero() {}

pub fn setup_shield(
    query: Query<(Entity, &Transform), (With<Jacqueline>, Added<Transform>)>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (ent, transform) in &query {}
}

pub fn animation_handler() {
    todo!();
}
