use super::*;
use bevy::ecs::query;
use bevy::prelude::*;
use corgee::character::*;
use corgee::*;
use go_character::*;
use iyes_loopless::prelude::*;

impl Plugin for Soul {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::InGame).with_system(crosshair_setup))
            .add_event::<ShieldEvent>()
            .add_system_set(
                ConditionSet::new()
                    .with_system(walk::<Soul>)
                    .with_system(look::<Soul>)
                    .with_system(is_grounded::<Soul>)
                    .with_system(jump::<Soul>)
                    .with_system(shield_toggler)
                    .with_system(place_shield)
                    .with_system(is_pointing)
                    .with_system(crosshair)
                    .with_system(crosshair_2)
                    .with_system(attack)
                    .into(),
            );
    }
}

const PLACE_SHIELD: f32 = 3.;
const GET_SHIELD: f32 = 5.;
const SHIELD_COOLDOWN: f32 = 2.;

fn is_pointing(
    rapier_context: Res<RapierContext>,
    char: Query<(&Children, &Transform, &Team, &GoInputs, Entity), (With<Soul>, Without<Killed>)>,
    q_head: Query<&Children, With<ZHead>>,
    q_cam: Query<(&Camera, &GlobalTransform)>,
    mut enemy: Query<&Team, (With<ChCore>, Without<Killed>)>,
    show_ray: Res<ShowRay>,
    mut lines: ResMut<DebugLines>,
    mut commands: Commands,
) {
    for (children, transform, team, ginp, ch_entity) in char.iter() {
        for &child in children.iter() {
            let head_children = q_head.get(child).unwrap();
            for &child in head_children.iter() {
                let (cam, cam_transform) = q_cam.get(child).unwrap();
                //println!("rotation: {}", transform.rotation);

                let (c_translation, c_rotation, c_scale) =
                    cam_transform.to_scale_rotation_translation();

                let shape = Collider::ball(1.5);
                let shape_pos = cam_transform.translation() + cam_transform.forward() * 2.5;
                let shape_rot = c_rotation;
                let shape_vel = cam_transform.forward() * 5.;
                let max_toi = 10.;
                let groups = InteractionGroups::default().into();

                commands.entity(ch_entity).remove::<PointingOn>();

                if let Some((entity, hit)) = rapier_context
                    .cast_shape(shape_pos, shape_rot, shape_vel, &shape, max_toi, groups)
                {

                    let en = enemy.get_mut(entity);

                    if en.is_ok() {
                        let en_team = en.unwrap();

                        if team.0 != en_team.0 {
                            commands.entity(ch_entity).insert(PointingOn {
                                target: entity,
                                hit: hit
                            });
                        }
                    }
                    

                    if show_ray.0 {
                        lines.line_colored(
                            shape_pos + shape_vel * hit.toi,
                            shape_pos + shape_vel * max_toi,
                            0.0,
                            Color::BLUE,
                        );
                    }
                }
                if show_ray.0 {
                    lines.line_colored(
                        shape_pos,
                        shape_pos + shape_vel * max_toi,
                        0.0,
                        Color::CYAN,
                    );
                }
            }
        }
    }
}

fn crosshair(
    mut is_pointing: Query<(Option<&PointingOn>, &GoInputs, &mut Crosshair), (With<Selected>, Without<Killed>)>,
) {
        for (pointing_on, ginp, mut crosshair) in is_pointing.iter_mut() {

            crosshair.0 = 100.;
            if let Some(pointing_on) = pointing_on{
                println!("pointing");
                if ginp.fire == 1 {
                    crosshair.0 = 70.;
                } else {
                    crosshair.0 = 120.;
                }
            }

        }
    
}
fn crosshair_2(
    mut crosshair_box: Query<&mut Style, With<Crosshair>>,
    q_val: Query<&Crosshair>,
){
    for val in q_val.iter(){
        for mut style in crosshair_box.iter_mut(){
            style.size = Size::new(Val::Px(val.0), Val::Px(val.0));
            println!("cross val: {}", val.0);
            if val.0 > 200.{

            }
        }
    }

}

fn attack(
    char: Query<(&PointingOn, &Team, &GoInputs), (With<Soul>, Without<Killed>)>,
    mut enemy: Query<(&mut Hp, &Team), (With<ChCore>, Without<Killed>)>,
    time: Res<Time>,
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

fn place_shield(mut commands: Commands, mut evr: EventReader<ShieldEvent>, ass: Res<AssetServer>) {
    for ev in evr.iter() {
        match ev {
            ShieldEvent::Up => {
                println!("Shield Up");

                // commands
                //     .spawn_bundle(SceneBundle {
                //         scene: ass.load("models/shield.gltf#Scene0"),
                //         transform: Transform {
                //             translation: transform.forward() * 20. + transform.translation,
                //             scale: Vec3::new(3., 3., 3.),
                //             rotation: transform.rotation,
                //         },
                //         ..Default::default()
                //     })
                //     .insert(AsyncSceneCollider {
                //         handle: handle,
                //         shape: Some(ComputedColliderShape::TriMesh),
                //         named_shapes: Default::default(),
                //     })
                //     .insert(RigidBody::Fixed);
            }
            ShieldEvent::Down => {
                println!("Shield Down")
            }
        }
    }
}

fn shield_toggler(
    mut char: Query<
        (
            &Team,
            &GoInputs,
            &mut QTimer,
            &mut PreQTimer,
            &mut ShieldUp,
            &Transform,
            &mut QLimiter,
        ),
        (With<Soul>, Without<Killed>),
    >,
    mut ev_wr: EventWriter<ShieldEvent>,
    time: Res<Time>,

    mut commands: Commands,
) {
    for (team, ginp, mut q_timer, mut pre_q_timer, mut shield_up, transform, mut limiter) in
        char.iter_mut()
    {
        if q_timer.0 >= 0. {
            limiter.0 = false;
            q_timer.0 -= time.delta_seconds();
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
            limiter.0 = false;
            q_timer.0 = 1.5;
        }

        if ginp.a_1 == 1 && limiter.0 {
            if q_timer.0 <= 0. {
                pre_q_timer.0 -= time.delta_seconds();

                if pre_q_timer.0 <= 0. {
                    q_timer.0 = SHIELD_COOLDOWN; // cooldown time

                    if !shield_up.0 {
                        ev_wr.send(ShieldEvent::Up);
                    } else {
                        ev_wr.send(ShieldEvent::Down);
                    }

                    shield_up.0 = !shield_up.0;
                    limiter.0 = false;
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
fn crosshair_setup(mut commands: Commands) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(100.), Val::Px(100.)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            color: Color::rgba(0.4, 0.4, 1.0, 0.).into(),
            visibility: Visibility { is_visible: true },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        //left: Val::Px(0.),
                        bottom: Val::Percent(100.0),
                        ..default()
                    },
                    size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),

                    ..default()
                },
                color: Color::BISQUE.into(),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        //left: Val::Px(0.),
                        left: Val::Percent(100.0),
                        ..default()
                    },
                    size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),

                    ..default()
                },
                color: Color::BISQUE.into(),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        //left: Val::Px(0.),
                        right: Val::Percent(100.0),
                        ..default()
                    },
                    size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),

                    ..default()
                },
                color: Color::BISQUE.into(),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        //left: Val::Px(0.),
                        top: Val::Percent(100.0),
                        ..default()
                    },
                    size: Size::new(Val::Percent(10.0), Val::Percent(10.0)),

                    ..default()
                },
                color: Color::BISQUE.into(),
                ..default()
            });
        })
        .insert(Crosshair(100.));
}

impl Character<Soul> for Soul {
    fn spawn(mut spawn_request: EventReader<SpawnChar>, mut commands: Commands) {
        for spawn_request in spawn_request.iter() {
            if spawn_request.0 == "Soul" {
                commands
                    .spawn()
                    .insert(Soul)
                    .insert(ShieldUp(false))
                    .insert(PreQTimer(PLACE_SHIELD))
                    .insert(QLimiter(true))
                    .insert_bundle(Config {
                        ..Default::default()
                    })
                    .insert_bundle(States {
                        id: Id(spawn_request.2),
                        team: Team(spawn_request.1 as i16),
                        ..Default::default()
                    });
            }
        }
    }
}

#[derive(Component)]
struct PreQTimer(f32);
#[derive(Component)]
struct ShieldUp(bool);
enum ShieldEvent {
    Up,
    Down,
}

#[derive(Component)]
struct QLimiter(bool);

#[derive(Component)]
struct Crosshair(f32);

#[derive(Component)]
struct PointingOn {
    target: Entity,
    hit: Toi,
}
