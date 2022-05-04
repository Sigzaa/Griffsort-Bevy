use bevy::{ prelude::*};
use crate::game::components::{ filters::*, player_data::*, };
pub fn name_and_hp_tags_update_system(
    q_camera: Query<(&Camera, &GlobalTransform), With<ThreeDCam>>,
    mut text: Query<(&mut Style, &CalculatedSize, &mut Text), (With<HpText>, Without<Selected>)>,
    mut q_player: Query<
        (Entity, &GlobalTransform, &Hp, Option<&Team>),
        (With<Hp>, Without<Killed>, Without<Selected>),
    >,
    windows: Res<Windows>,
) {
//     let mut counter = 0;
//     let mut limiter = 0;
//     for (camera, camera_transform) in q_camera.iter() {
//         for (_player, player_transform, hp, team) in q_player.iter_mut() {
//             for (mut style, calculated, mut text) in text.iter_mut() {
//                 if let Some(_team) = team {
                    // if counter == limiter {
                    //     match camera.world_to_screen(
                    //         &windows,
                    //         camera_transform,
                    //         player_transform.translation + Vec3::new(0., 1.5, 0.),
                    //     ) {
                    //         Some(coords) => {
                    //             style.position.left =
                    //                 Val::Px(coords.x - calculated.size.width / 2.0);
                    //             style.position.bottom =
                    //                 Val::Px(coords.y - calculated.size.height / 2.0);
                    //             text.sections[0].value = hp.0.to_string();
                    //         }
                    //         None => {
                    //             style.position.bottom = Val::Px(-1000.0);
                    //         }
                    //     }
                    //     limiter += 1;
                    //     counter = 0;
                    //     break;
                    // } else {
                    //     counter += 1;
                    // }
        //         } else {
        //             style.position.bottom = Val::Px(-1000.0);
        //         }
        //     }
        // }
    // }
}
