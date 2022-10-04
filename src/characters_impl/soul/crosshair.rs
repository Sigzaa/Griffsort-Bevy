use crate::Action;
use actions::Actions;
use bevy::prelude::*;
use keyframe::ease;
use keyframe::functions::*;

use super::super::*;
use super::resources::*;

pub(crate) fn crosshair(
    is_pointing: Query<(&ShapeIntersections, &Actions<Action>), (With<Selected>, Without<Dead>)>,
    mut crosshair_val: Query<&mut CrosshairValue>,
    mut crosshair_box: Query<&mut Style, With<Crosshair>>,
    conf: Res<SoulConfig>,
) {
    for (pointing_on, ginp) in is_pointing.iter()
    {
        for mut crosshair in crosshair_val.iter_mut()
        {
            if pointing_on.0.len() > 0
            {
                if ginp.pressed(Action::Shoot)
                {
                    // Attacking
                    crosshair.0 = ease(
                        EaseInQuint,
                        crosshair.0,
                        150.,
                        conf.crosshair.to_attack_duration,
                    );
                }
                else
                {
                    // Pointing
                    crosshair.0 = ease(
                        EaseInQuint,
                        crosshair.0,
                        70.,
                        conf.crosshair.to_pointing_duration,
                    );
                }
            }
            else
            {
                // Not Pointing
                crosshair.0 = ease(
                    EaseInQuint,
                    crosshair.0,
                    100.,
                    conf.crosshair.to_idle_duration,
                );
            }

            for mut style in crosshair_box.iter_mut()
            {
                style.size = Size::new(Val::Px(crosshair.0), Val::Px(crosshair.0));
                if crosshair.0 > 200.
                {}
            }
        }
    }
}
pub(crate) fn crosshair_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
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
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(70.0 / 3.), Val::Px(40.0 / 3.)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        //left: Val::Px(10.),
                        left: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                image: asset_server.load("sprites/soul-crosshair.png").into(),
                transform: Transform::from_rotation(Quat::from_rotation_z(
                    -std::f32::consts::PI / 2.,
                )),
                //transform: Transform::from_xyz(100., 10., 10.),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(70.0 / 3.), Val::Px(40.0 / 3.)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        //left: Val::Px(10.),
                        bottom: Val::Percent(100.),

                        ..default()
                    },
                    ..default()
                },
                image: asset_server.load("sprites/soul-crosshair.png").into(),
                //transform: Transform::from_rotation(Quat::from_rotation_z(-std::f32::consts::PI/2.)),
                //transform: Transform::from_xyz(100., 10., 10.),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(70.0 / 3.), Val::Px(40.0 / 3.)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        //left: Val::Px(10.),
                        right: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                image: asset_server.load("sprites/soul-crosshair.png").into(),
                transform: Transform::from_rotation(Quat::from_rotation_z(
                    std::f32::consts::PI / 2.,
                )),
                //transform: Transform::from_xyz(100., 10., 10.),
                ..default()
            });
        })
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Px(70.0 / 3.), Val::Px(40.0 / 3.)),
                    position_type: PositionType::Absolute,
                    position: UiRect {
                        //left: Val::Px(10.),
                        top: Val::Percent(100.),
                        ..default()
                    },
                    ..default()
                },
                image: asset_server.load("sprites/soul-crosshair.png").into(),
                transform: Transform::from_rotation(Quat::from_rotation_z(-std::f32::consts::PI)),
                //transform: Transform::from_xyz(100., 10., 10.),
                ..default()
            });
        })
        .insert(Crosshair)
        .insert(CrosshairValue(100.));
}
