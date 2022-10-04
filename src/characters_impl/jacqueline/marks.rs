use bevy::prelude::*;
use bevy_rapier3d::geometry::Sensor;
use bevy_rapier3d::prelude::*;
use keyframe::{ease, functions::EaseInQuint};
use std::{collections::HashSet, f32::consts::PI};

use crate::characters_impl::{jacqueline::resources::MarksLinks, Jacqueline};

use heroes::*;

use super::resources::*;

pub fn respawn_marks(
    mut query: Query<(&mut MarksCD, Entity), With<Jacqueline>>,
    conf: Res<JacquelineConfig>,
    mut spawn: EventWriter<SpawnMarkEv>,
    time: Res<Time>,
) {
    for (mut mcd, entity) in &mut query
    {
        mcd.tick_timers(time.delta_seconds());

        if !mcd.is_cooldown(1) && conf.marks.max_amount > mcd.left() as usize
        {
            spawn.send(SpawnMarkEv {
                amount: 1,
                owner: entity,
            });

            mcd.add(1);

            mcd.cooldown(1, 2.)
        }
    }
}

pub fn follow_hero(
    query: Query<(&Transform, &MarksLinks), With<Jacqueline>>,
    conf: Res<JacquelineConfig>,
    mut markq: Query<(&mut Transform, &MarkState), Without<Jacqueline>>,
) {
    for (transform, links) in query.iter()
    {
        for mark in links.0.iter()
        {
            if let Ok((mut mark_transform, state)) = markq.get_mut(*mark)
            {
                if let MarkState::Idle(angle) = &state
                {
                    let back = transform.right();

                    let mut offset =
                        Vec3::new(angle.sin() * back[0], angle.cos(), angle.sin() * back[2]);

                    offset += Vec3::new(0., 0.7, 0.);

                    offset *= conf.marks.distance;

                    offset += transform.translation * transform.back() * 0.1;

                    let to = transform.translation + offset;

                    for i in 0..3
                    {
                        mark_transform.translation[i] = ease(
                            EaseInQuint,
                            mark_transform.translation[i],
                            to[i],
                            conf.marks.animation_delay,
                        );
                    }
                }
            }
        }
    }
}

pub fn insert_marks(
    query: Query<Entity, (With<Jacqueline>, Added<Transform>)>,
    _evwr: EventWriter<SpawnMarkEv>,
    mut commands: Commands,
    _conf: Res<JacquelineConfig>,
) {
    for entity in query.iter()
    {
        // evwr.send(SpawnMarkEv {
        //     amount: conf.marks.max_amount,
        //     owner: entity,
        // });
        commands.entity(entity).insert(MarksLinks(HashSet::new()));
    }
}

pub fn rearrange_angles(
    query: Query<(Entity, &MarksLinks), With<Jacqueline>>,
    mut markq: Query<&mut MarkState, Without<Jacqueline>>,
    mut ev: EventReader<RecalkAnglesEv>,
) {
    for _i in ev.iter()
    {
        for (_eni, links) in query.iter()
        {
            // Amount of marks in idle
            let mut in_idle = 0;

            // Naive impl of counting
            for mark in links.0.iter()
            {
                let res = markq.get_mut(*mark);
                if let Ok(state) = res
                {
                    if let MarkState::Idle(_angle) = *state
                    {
                        in_idle += 1;
                    }
                }
            }

            let mut index = 0;

            for mark in links.0.iter()
            {
                if let Ok(mut state) = markq.get_mut(*mark)
                {
                    if let MarkState::Idle(_angle) = *state
                    {
                        let rotation_offset = match in_idle
                        {
                            4 => 45.,
                            2 => -90.,
                            _ => 0.,
                        };

                        let angle = if in_idle == 1
                        {
                            0.
                        }
                        else
                        {
                            calculate_angle(in_idle, index, rotation_offset)
                        };

                        index += 1;

                        *state = MarkState::Idle(angle);
                    }
                }
            }
        }
    }
}

pub fn spawn_mark(
    mut ev: EventReader<SpawnMarkEv>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(&mut MarksLinks, &Transform), With<Jacqueline>>,
    conf: Res<JacquelineConfig>,
    mut rearrange: EventWriter<RecalkAnglesEv>,
) {
    for spawn in ev.iter()
    {
        let sided = conf.marks.max_amount;

        for i in 0..spawn.amount
        {
            let angle = calculate_angle(sided, i, 0.);

            let offset = Vec3::new(angle.cos(), angle.sin(), 0.);

            if let Ok((mut marks_links, transform)) = query.get_mut(spawn.owner)
            {
                //println!("owner{:?}, links {:?}", spawn.owner, marks_links.0);

                let mark = commands
                    .spawn_bundle(PbrBundle {
                        mesh: meshes.add(Mesh::from(shape::Icosphere {
                            radius: 0.05,
                            subdivisions: 12,
                        })),
                        material: materials.add(StandardMaterial {
                            base_color: Color::hex("ffd891").unwrap(),
                            // vary key PBR parameters on a grid of spheres to show the effect
                            ..default()
                        }),
                        transform: Transform::from_translation(transform.translation + offset),
                        ..default()
                    })
                    .insert(MarkState::Idle(angle))
                    .insert(Collider::ball(0.5))
                    .insert(Sensor)
                    .id();

                marks_links.0.insert(mark);
            }
        }
        rearrange.send(RecalkAnglesEv);
    }
}

// Returning an Angle in Radians

fn calculate_angle(sided: usize, index: usize, rotation_offset: f32) -> f32 {
    let complete_angle = 180 * (sided - 2);

    let step = 180. - (complete_angle / sided) as f32;

    (index as f32 * step + rotation_offset) * PI / 180.
}
