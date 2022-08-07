use super::*;
use bevy::prelude::*;
use go_character::{*};
use corgee::character::*;
use corgee::*;
use iyes_loopless::prelude::*;

impl Plugin for Soul {
    fn build(&self, app: &mut App) {
        app.add_system_set(ConditionSet::new()
        .with_system(walk::<Soul>)
        .with_system(look::<Soul>)
        .with_system(is_grounded::<Soul>)
        .with_system(jump::<Soul>)
        .with_system(main_shield)
        .into());
    }
}

fn main_shield(
    mut char: Query<(&Team, &GoInputs, &mut QTimer, &mut PreQTimer, &mut ShieldUp, &Transform), (With<Soul>, Without<Killed>)>,
    time: Res<Time>,
    ass: Res<AssetServer>,
    mut commands: Commands,
){
    for (team, ginp, mut q_timer, mut pre_q_timer, mut shield_up, transform) in char.iter_mut(){

        if q_timer.0 >= 0.{
            q_timer.0 -= time.delta_seconds();
            println!("couldown: {}", q_timer.0);
        }

        if ginp.a_1 == 1{
            if q_timer.0 <= 0.{

                pre_q_timer.0 -= time.delta_seconds();



                println!("time remaining: {}", pre_q_timer.0);

                if pre_q_timer.0 <= 0. {
                    println!("shield up");

                    if !shield_up.0{
                    commands.spawn_bundle(SceneBundle {
                        scene: ass.load("models/shield.gltf#Scene0"),
                        transform: Transform{
                            translation: transform.forward() * 20. + transform.translation, 
                            scale: Vec3::new(3.,3.,3.),
                            rotation: transform.rotation
                        },
                        ..Default::default()
                    })
                    .insert(Collider::cuboid(0.5, 2.0, 0.5))
                    .insert(RigidBody::Fixed)
                    ;} else {

                    }

                    shield_up.0 =! shield_up.0;
                    q_timer.0 = 1.;
                }

            } else {
                println!("in couldown");
            }
        } else {
            pre_q_timer.0 = 0.5;
        }
    }
}
#[derive(Component)]

struct PreQTimer(f32);

#[derive(Component)]

struct ShieldUp(bool);



impl Character<Soul> for Soul {
    fn spawn(mut spawn_request: EventReader<SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter() {
            if spawn_request.0 == "Soul" {

                commands
                    .spawn()
                    .insert(Soul)
                    .insert(ShieldUp(false))
                    .insert(PreQTimer(0.5))
                    .insert_bundle(Config {
                        ..Default::default()
                    })
                    .insert_bundle(States {
                        id: Id(spawn_request.2),
                        ..Default::default()
                    });
            }
        }
    }
}
