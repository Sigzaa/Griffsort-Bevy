use bevy::prelude::*;
use corgee::*;
use super::super::*;
use super::resources::*;

pub(crate) fn attack(
    char: Query<(&PointingOn, &Team, &GoInputs), (With<Soul>, Without<Killed>)>,
    mut enemy: Query<(&mut Hp, &Team), (With<ChCore>, Without<Killed>)>,
    _time: Res<Time>,
) {
    for (pointing_on, team, ginp) in char.iter() {
        let en = enemy.get_mut(pointing_on.target);

        if en.is_ok() {
            let (mut hp, en_team) = en.unwrap();

            if ginp.fire == 1 && team.0 != en_team.0 {
                hp.0 -= 10;
            }
            //println!("enemy hp: {:?}", hp.0);
        }
    }
}

pub(crate) fn place_n_get_shield(
    mut commands: Commands,
    ass: Res<AssetServer>,
    mut char: Query<(&ShieldUp, &ShieldPos, Entity, &Children), (With<Soul>, Changed<ShieldUp>)>,
    shield_q: Query<(Entity, &ShieldFather)>,
) {
    for (shield_up, shield_pos, entity, children) in char.iter_mut() {
        match shield_pos.0 {
            Some(transform) => {
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
            None => {
                println!("get");
                for (ent, shield_father) in shield_q.iter() {
                    println!(
                        "entity: {entity:?}, father: {:?}, ent: {ent:?}",
                        shield_father.0
                    );
                    if entity == shield_father.0 {
                        commands.entity(ent).despawn_recursive();
                    }
                }
            }
        }
    }
}

pub(crate) fn shield_toggler(
    mut char: Query<
        (
            &Team,
            &GoInputs,
            &mut QTimer,
            &mut PreQTimer,
            &mut ShieldUp,
            &mut ShieldPos,
            &Transform,
            &mut QLimiter,
        ),
        (With<Soul>, Without<Killed>),
    >,
    mut ev_wr: EventWriter<ShieldEvent>,
    time: Res<Time>,

    _commands: Commands,
) {
    for (
        _team,
        ginp,
        mut q_timer,
        mut pre_q_timer,
        mut shield_up,
        mut shield_pos,
        transform,
        mut limiter,
    ) in char.iter_mut()
    {
        if q_timer.0 >= 0. {
            limiter.0 = false;
            q_timer.0 -= time.delta_seconds();

            if q_timer.0 <= 0. {
                println!("shield is ready");
            }
        }
        if ginp.a_1 == 0 {
            limiter.0 = true;
        }
        if ginp.a_1 == 0 && pre_q_timer.0 < GET_SHIELD && shield_up.0 {
            // Interruption on getting shield
            limiter.0 = false;
            q_timer.0 = 1.5;
        }
        if ginp.a_1 == 0 && pre_q_timer.0 < PLACE_SHIELD && !shield_up.0 {
            // Interruption on placing shield
            // limiter.0 = false;
            // q_timer.0 = 1.5;
        }

        if ginp.a_1 == 1 && limiter.0 {
            if q_timer.0 <= 0. {
                // Placing or getting a shield

                if !shield_up.0 {
                    //Start shield up animation
                    shield_pos.0 = Some(*transform);
                } else {
                    //Start shield down animation
                    shield_pos.0 = None;
                }

                pre_q_timer.0 -= time.delta_seconds();

                if pre_q_timer.0 <= 0. {
                    q_timer.0 = SHIELD_COOLDOWN;
                    if !shield_up.0 {
                        println!("shield up");
                        // cooldown time
                        shield_up.0 = true;
                        //limiter.0 = false;
                    } else {
                        println!("shield down");
                        shield_up.0 = false;
                        limiter.0 = true;
                    }
                }
            } else {
                //println!("in cooldown");
            }
        } else {
            if shield_up.0 {
                pre_q_timer.0 = GET_SHIELD // Time to remove shield
            }
            if !shield_up.0 {
                pre_q_timer.0 = PLACE_SHIELD; // Time to place shield
            }
        }

        // println!(
        //     "q_time: {:.1}, pre_q: {:.1}, is up: {}",
        //     q_timer.0, pre_q_timer.0, shield_up.0
        // );
    }
}