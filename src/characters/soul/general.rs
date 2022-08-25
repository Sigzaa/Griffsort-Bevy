use bevy::prelude::*;
use corgee::*;
use super::super::*;
use super::resources::*;

pub(crate) fn is_pointing(
    // To dirty. I hate working with parenting :/
    rapier_context: Res<RapierContext>,
    char: Query<(&Children, &Team, Entity), (With<Soul>, Without<Killed>)>,
    q_head: Query<&Children, With<ZHead>>,
    q_cam: Query<(&Camera, &GlobalTransform)>,
    mut enemy: Query<&Team, (With<ChCore>, Without<Killed>)>,
    show_ray: Res<ShowRay>,
    mut lines: ResMut<DebugLines>,
    mut commands: Commands,
) {
    for (children, team, ch_entity) in char.iter() {
        for &child in children.iter() {
            let head_children = q_head.get(child);
            if head_children.is_ok() {
                let head_children = head_children.unwrap();

                for &child in head_children.iter() {
                    let (_cam, cam_transform) = q_cam.get(child).unwrap();
                    //println!("rotation: {}", transform.rotation);

                    let (_c_translation, c_rotation, _c_scale) =
                        cam_transform.to_scale_rotation_translation();

                    let shape = Collider::ball(1.5);
                    let shape_pos = cam_transform.translation() + cam_transform.forward() * 2.5;
                    let shape_rot = c_rotation;
                    let shape_vel = cam_transform.forward() * 5.;
                    let max_toi = 10.;
                    //let groups = InteractionGroups::new(0b100, 0b10).exclude_sensors().into();
                    let groups = QueryFilter::exclude_fixed();

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
                                    hit: hit,
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
}
