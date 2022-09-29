use super::super::*;
use super::resources::*;
use crate::Action;
use actions::Actions;
use bevy::prelude::*;

pub(crate) fn attack(
    char: Query<(&ShapeIntersections, &Team, &Actions<Action>), (With<Soul>, Without<Dead>)>,
    mut enemy: Query<(&mut Hp, &Team), (With<Hero>, Without<Dead>)>,
    _time: Res<Time>,
) {
    // for (pointing_on, team, act) in char.iter() {
    //     let en = enemy.get_mut(pointing_on.target);

    //     if en.is_ok() {
    //         let (mut hp, en_team) = en.unwrap();

    //         if act.pressed(Action::Shoot) && team != en_team {
    //             hp.0 -= 10;
    //         }
    //         //println!("enemy hp: {:?}", hp.0);
    //     }
    // }
}

pub(crate) fn place_n_get_shield(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut char: Query<(&ShieldUp, &ShieldPos, Entity, &Children), (With<Soul>, Changed<ShieldUp>)>,
    shield_q: Query<(Entity, &ShieldFather)>,
) {
    for (shield_up, shield_pos, entity, children) in char.iter_mut()
    {
        match shield_pos.0
        {
            Some(transform) =>
            {
                println!("place");
                let handle = ass.load("models/shield.gltf#Scene0");
                commands
                    .spawn_bundle(SceneBundle {
                        scene: handle.clone(),
                        transform: Transform {
                            translation: transform.forward() * 5. + transform.translation,
                            scale: Vec3::new(3., 3., 3.),
                            rotation: transform.rotation,
                        },
                        ..Default::default()
                    })
                    .insert(AsyncSceneCollider {
                        handle: handle,
                        shape: Some(ComputedColliderShape::TriMesh),
                        named_shapes: Default::default(),
                    })
                    .insert(RigidBody::Fixed)
                    .insert(ShieldFather(entity));
            }
            None =>
            {
                println!("get");
                for (ent, shield_father) in shield_q.iter()
                {
                    println!(
                        "entity: {entity:?}, father: {:?}, ent: {ent:?}",
                        shield_father.0
                    );
                    if entity == shield_father.0
                    {
                        commands.entity(ent).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub(crate) fn shield_toggler(
    mut char: Query<(&Actions<Action>, &mut ShieldUp, &mut ShieldPos, &Transform), With<Soul>>,
    mut ev_wr: EventWriter<ShieldEvent>,
    time: Res<Time>,

    _commands: Commands,
) {
    // for (
    //     _team,
    //     ginp,
    //     mut q_timer,
    //     mut pre_q_timer,
    //     mut shield_up,
    //     mut shield_pos,
    //     transform,
    //     mut limiter,
    // ) in char.iter_mut()
    // {
    //     if q_timer.0 >= 0. {
    //         limiter.0 = false;
    //         q_timer.0 -= time.delta_seconds();

    //         if q_timer.0 <= 0. {
    //             println!("shield is ready");
    //         }
    //     }
    //     if ginp.a_1 == 0 {
    //         limiter.0 = true;
    //     }
    //     if ginp.a_1 == 0 && pre_q_timer.0 < GET_SHIELD && shield_up.0 {
    //         // Interruption on getting shield
    //         limiter.0 = false;
    //         q_timer.0 = 1.5;
    //     }
    //     if ginp.a_1 == 0 && pre_q_timer.0 < PLACE_SHIELD && !shield_up.0 {
    //         // Interruption on placing shield
    //         // limiter.0 = false;
    //         // q_timer.0 = 1.5;
    //     }

    //     if ginp.a_1 == 1 && limiter.0 {
    //         if q_timer.0 <= 0. {
    //             // Placing or getting a shield

    //             if !shield_up.0 {
    //                 //Start shield up animation
    //                 shield_pos.0 = Some(*transform);
    //             } else {
    //                 //Start shield down animation
    //                 shield_pos.0 = None;
    //             }

    //             pre_q_timer.0 -= time.delta_seconds();

    //             if pre_q_timer.0 <= 0. {
    //                 q_timer.0 = SHIELD_COOLDOWN;
    //                 if !shield_up.0 {
    //                     println!("shield up");
    //                     // cooldown time
    //                     shield_up.0 = true;
    //                     //limiter.0 = false;
    //                 } else {
    //                     println!("shield down");
    //                     shield_up.0 = false;
    //                     limiter.0 = true;
    //                 }
    //             }
    //         } else {
    //             //println!("in cooldown");
    //         }
    //     } else {
    //         if shield_up.0 {
    //             pre_q_timer.0 = GET_SHIELD // Time to remove shield
    //         }
    //         if !shield_up.0 {
    //             pre_q_timer.0 = PLACE_SHIELD; // Time to place shield
    //         }
    //     }
    // }
}
pub fn sprint(
    mut q: Query<(&Actions<Action>, &mut EscCD, &mut ExternalForce, &Transform), With<Soul>>,
    time: Res<Time>,
) {
    for (ginp, mut cd, mut force, transform) in &mut q
    {
        cd.tick_timers(time.delta_seconds());

        if ginp.just_pressed(Action::Sprint) && cd.is_ready(0)
        {
            cd.cooldown(0, 0.5);
            cd.cooldown(1, 1.5);
            cd.add(-1);

            // let (right, forward) = (ginp.movement[0], ginp.movement[1]);

            // let direction = transform.forward() * forward + transform.right() * right + Vec3::new(0., 0.01, 0.);

            // force.force += direction * 50000.;
        }
        if !cd.is_cooldown(1) && cd.left() < 15
        {
            cd.add(1);
            cd.cooldown(1, 2.5);
        }
    }
}
