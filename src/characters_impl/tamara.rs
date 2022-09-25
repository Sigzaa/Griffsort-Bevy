use super::*;
use bevy::asset::LoadState;
use bevy::prelude::shape::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
use heroes::*;

#[derive(Component)]
struct Rocket;

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
struct ExpLifeTime(Timer);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum SpriteState {
    Setup,
    Finished,
}

#[derive(Default)]
struct ExplSpriteHandles {
    handles: Vec<HandleUntyped>,
}

impl Plugin for Tamara {
    fn build(&self, app: &mut App) {
        app
            // .add_system_set(
            //     SystemSet::on_update(GameState::InGame).with_system(Tamara::sync_rotation::<Tamara>)
            // )
            //.add_system(Tamara::move_player::<Tamara>)
            //.add_system(Tamara::jump::<Tamara>)
            .init_resource::<ExplSpriteHandles>()
            .add_state(SpriteState::Setup)
            .add_system_set(SystemSet::on_enter(SpriteState::Setup).with_system(load_textures))
            .add_system_set(SystemSet::on_update(SpriteState::Setup).with_system(check_textures))
            .add_system_set(SystemSet::on_enter(SpriteState::Finished).with_system(setup))
            // .add_system(Tamara::fly::<Tamara>)
            .add_system(Tamara::explode_rocket)
            // .add_system(Tamara::shoot::<Tamara>)
            .add_system(Tamara::animate_explosion);
    }
}

fn load_textures(
    mut rpg_sprite_handles: ResMut<ExplSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    rpg_sprite_handles.handles = asset_server
        .load_folder("sprites/pack1/PNG/Explosion")
        .unwrap();
}
fn setup(
    _commands: Commands,
    _rpg_sprite_handles: Res<ExplSpriteHandles>,
    _asset_server: Res<AssetServer>,
    _texture_atlases: ResMut<Assets<TextureAtlas>>,
    _textures: ResMut<Assets<Image>>,
) {

    // draw the atlas itself
    // commands.spawn_bundle(SpriteBundle {
    //     texture: texture_atlas_texture,
    //     transform: Transform::from_xyz(-300.0, 0.0, 0.0),
    //     ..default()
    // });
}
fn check_textures(
    mut state: ResMut<State<SpriteState>>,
    rpg_sprite_handles: ResMut<ExplSpriteHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(rpg_sprite_handles.handles.iter().map(|handle| handle.id))
    {
        state.set(SpriteState::Finished).unwrap();
    }
}

use bevy::input::mouse::MouseWheel;
impl Tamara {
    // fn fly<C: Component>(
    //     mut q_sel: Query<(&GoInputs, &mut ExternalForce, &MaxJump, &Transform), With<C>>,
    //     time: Res<Time>,
    // ) {
    //     for (inputs, mut force, max_jump, transform) in q_sel.iter_mut() {
    //         if inputs.jump == 1 {
    //             force.force = transform.up() * max_jump.0 * 700. * time.delta_seconds();
    //            //println!("force: {}", force.force);
    //         } else {
    //             force.force = Vec3::ZERO;
    //         }
    //     }
    // }

    // fn shoot<C: Component>(
    //     mut q_sel: Query<(&GoInputs, &Transform, &mut ShootTimer, &mut IsReadyShoot), With<C>>,
    //     mut commands: Commands,
    //     mut meshes: ResMut<Assets<Mesh>>,
    //     mut materials: ResMut<Assets<StandardMaterial>>,
    //     time: Res<Time>,
    // ) {

    //     for (ginp, transform, mut timer, mut can_shoot) in q_sel.iter_mut(){
    //         timer.0.tick(time.delta());
    //         if timer.0.finished(){
    //             can_shoot.0 = true;
    //         }
    //         if ginp.shoot == 1 && can_shoot.0{
    //             can_shoot.0 = false;
    //             commands
    //             .spawn_bundle(PbrBundle {
    //                 mesh: meshes.add(Mesh::from(Cube {
    //                     size: 0.05,
    //                 })),
    //                 material: materials.add(StandardMaterial {
    //                     base_color: Color::rgba(0.9, 0.2, 0.1, 0.5),
    //                     alpha_mode: AlphaMode::Blend,
    //                     ..Default::default()
    //                 }),
    //                 transform: Transform{
    //                     translation: transform.translation + transform.forward() + Vec3::new(0., 0.4, 0.),
    //                     ..Default::default()
    //                 },
    //                 ..Default::default()
    //             })
    //             .insert(Collider::ball(0.05))
    //             .insert(Velocity{
    //                 linvel: transform.forward() * 20.,
    //                 angvel: Vec3::ZERO,
    //             })

    //             .insert(GravityScale(1.))
    //             .insert(Damping {
    //                 linear_damping: 0.4,
    //                 angular_damping: 0.,
    //             })
    //             .insert(ColliderMassProperties::Density(2.5))
    //             .insert(Friction {
    //                 coefficient: 1.,
    //                 combine_rule: CoefficientCombineRule::Min,
    //             })
    //             .insert(RigidBody::Dynamic)
    //             .insert(ActiveEvents::COLLISION_EVENTS)
    //             .insert(Rocket)
    //             //.insert(Ccd::enabled())
    //             ;

    //         }
    //     }
    // }
    fn animate_explosion(
        time: Res<Time>,
        _texture_atlases: Res<Assets<TextureAtlas>>,
        mut query: Query<
            (
                Entity,
                &mut AnimationTimer,
                &mut ExpLifeTime,
                &mut Transform,
            ),
            Without<Selected>,
        >,
        sel: Query<&Transform, With<Selected>>,
        mut commands: Commands,
    ) {
        for (ent, mut timer, mut lifetime, mut transform) in query.iter_mut()
        {
            timer.tick(time.delta());
            lifetime.tick(time.delta());

            for sel_trans in sel.iter()
            {
                transform.look_at(sel_trans.translation, Vec3::Y);
            }

            if timer.just_finished()
            {
                // let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
                // sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
            }
            if lifetime.finished()
            {
                commands.entity(ent).despawn();
            }
        }
    }
    fn explode_rocket(
        mut collision_events: EventReader<CollisionEvent>,
        mut commands: Commands,
        q: Query<(Entity, &Transform), With<Rocket>>,
        asset_server: Res<AssetServer>,
        _texture_atlases: ResMut<Assets<TextureAtlas>>,
        _rpg_sprite_handles: Res<ExplSpriteHandles>,
        _textures: ResMut<Assets<Image>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        for collision_event in collision_events.iter()
        {
            println!("Received collision event: {:?}", collision_event);
            // if let Some(rocket) = collision_event.clone().0{
            for (ent, transform) in q.iter()
            {
                match collision_event
                {
                    CollisionEvent::Started(ent1, ent2, _flag) =>
                    {
                        if ent == *ent1 || ent == *ent2
                        {
                            let texture_handle =
                                asset_server.load("sprites/pack1/PNG/Explosion/Explosion6.png");
                            let material_handle = materials.add(StandardMaterial {
                                base_color_texture: Some(texture_handle.clone()),
                                alpha_mode: AlphaMode::Blend,
                                unlit: true,
                                ..default()
                            });
                            commands
                                .spawn_bundle(PbrBundle {
                                    mesh: meshes.add(Mesh::from(Box::new(2., 2., 0.))),
                                    material: material_handle,
                                    transform: Transform {
                                        translation: transform.translation + Vec3::new(0., 0.5, 0.),
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert_bundle(PointLightBundle {
                                    transform: Transform {
                                        translation: transform.translation + Vec3::new(0., 0.5, 0.),
                                        ..default()
                                    },
                                    point_light: PointLight {
                                        intensity: 600.0, // lumens - roughly a 100W non-halogen incandescent bulb
                                        color: Color::ORANGE,
                                        shadows_enabled: true,
                                        ..default()
                                    },
                                    ..default()
                                })
                                .insert(AnimationTimer(Timer::from_seconds(0.1, true)))
                                .insert(ExpLifeTime(Timer::from_seconds(0.7, false)));
                            // let texture_handle = asset_server.load("sprites/explosion.png");
                            // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(24.0, 24.0), 7, 1);
                            // let texture_atlas_handle = texture_atlases.add(texture_atlas);
                            // commands.spawn_bundle(OrthographicCameraBundle::new_2d());
                            // commands
                            //     .spawn_bundle(SpriteSheetBundle {
                            //         texture_atlas: texture_atlas_handle,
                            //         transform: Transform::from_scale(Vec3::splat(6.0)),
                            //         ..default()
                            //     })
                            //     .insert(AnimationTimer(Timer::from_seconds(0.5, true)))
                            //     .insert(ExpLifeTime(Timer::from_seconds(2.5, false)));
                            //
                            // let mut texture_atlas_builder = TextureAtlasBuilder::default();
                            // for handle in &rpg_sprite_handles.handles {
                            //     let texture = textures.get(handle).unwrap();
                            //     texture_atlas_builder.add_texture(handle.clone_weak().typed::<Image>(), texture);
                            // }

                            // let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
                            // let texture_atlas_texture = texture_atlas.texture.clone();
                            // let vendor_handle = asset_server.get_handle("sprites/pack1/PNG/Explosion/Explosion1.png");
                            // let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
                            // let atlas_handle = texture_atlases.add(texture_atlas);

                            // // set up a scene to display our texture atlas

                            // // draw a sprite from the atlas
                            // commands.spawn_bundle(SpriteSheetBundle {
                            //     transform: Transform {
                            //         translation: transform.translation,
                            //         scale: Vec3::splat(1.0),
                            //         ..default()
                            //     },
                            //     sprite: TextureAtlasSprite::new(vendor_index),
                            //     texture_atlas: atlas_handle,
                            //     ..default()
                            // })
                            // .insert(AnimationTimer(Timer::from_seconds(0.2, true)))
                            // .insert(ExpLifeTime(Timer::from_seconds(2.5, false)));;
                        }
                        commands.entity(ent).despawn();
                    }
                    _ =>
                    {}
                }
            }
            // }
        }
    }

    // fn sync_rotation<C: Component>(
    //     mut q_head: Query<(&Children, &mut Transform), (With<Head>, Without<Selected>)>,
    //     mut q_sel: Query<(&GoRot, &mut Transform, &Children, &GoInputs, &GlobalTransform), With<Selected>>,
    //     mut q_cam: Query<&mut Transform, (With<HeroCam>, Without<Selected>, Without<Head>)>,
    //     mut motion_evr: EventReader<MouseMotion>,
    //     time: Res<Time>,
    //     mut scroll_evr: EventReader<MouseWheel>,
    // ) {
    //     for (_gorot, mut body, children, ginp, _glob) in q_sel.iter_mut() {
    //         for &child in children.iter() {
    //             let (children, _head) = q_head.get_mut(child).unwrap();

    //             for &child in children.iter() {
    //                 let mut cam_transform = q_cam.get_mut(child).unwrap();
    //                 use bevy::math::EulerRot;
    //                 // body_transform.rotation = gorot.y;
    //                 // cam_transform.rotation = gorot.x;
    //                 for ev in motion_evr.iter() {
    //                     //body_transform.rotation *= Quat::from_rotation_y(-ev.delta.x * 0.002);
    //                     body.rotation *= Quat::from_rotation_z(-ev.delta.x * 0.004);
    //                     body.rotation *= Quat::from_rotation_x(-ev.delta.y * 0.004);
    //                     //head.rotation *= Quat::from_rotation_z(-ev.delta.x * 0.004);

    //                     //*gorot.z = Quat::from_rotation_z(-ev.delta.y * 0.004);
    //                     // body.rotation *= Quat::from_euler(EulerRot::XYZ, -ev.delta.x * 0.004, -ev.delta.y * 0.004, 0.);
    //                 }
    //                 body.rotation *= Quat::from_rotation_y(-ginp.movement[0] * 3. * time.delta_seconds());
    //                 //body.rotation *= Quat::from_rotation_y(0.01 );
    //                 //body.looking_at();
    //                 //body_transform.rotation = Quat::from_rotation_x(15.) + body_transform.rotation;
    //                 let (_t, _t1) = body.rotation.to_axis_angle();
    //                 let (mut z, _e2, _e3) = body.rotation.to_euler(EulerRot::ZYX);
    //                 let (y, _e2, _e3) = body.rotation.to_euler(EulerRot::YXZ);
    //                 //let (e1, e2, e3) = (e1, e2, e3);
    //                 let  y =  y * 1000./18.;
    //                 //body.rotation = Quat::from_euler(EulerRot::XYZ, -ev.delta.y * 0.004 + e1, e2, -ev.delta.x * 0.004 + e3);
    //                 let _rot = body.rotation.xyz()[2];
    //                 //println!("euler before: z {z:.3}, y {y:.3}");
    //                 if (y < -90. && y > -180.) || (y > 90. && y < 180.){
    //                     if z > 0.{
    //                         z -= 3.12;
    //                     } else if z < 0.{
    //                         z += 3.12;
    //                     }
    //                     z = -z;
    //                 }

    //                 let deadzone = 0.1;
    //                 if z > deadzone || z < -deadzone{
    //                     body.rotation *= Quat::from_rotation_y(z * time.delta_seconds() * 1.5);
    //                 }
    //                 //println!("euler after: z {z:.3}, y {y:.3}");

    //                 use bevy::input::mouse::MouseScrollUnit;
    //                 for ev in scroll_evr.iter() {
    //                     match ev.unit {
    //                         MouseScrollUnit::Line => {

    //                             cam_transform.rotation = Quat::from_rotation_x(ev.y * 0.5);
    //                         }
    //                         MouseScrollUnit::Pixel => {
    //                             //println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
    //                         }
    //                     }
    //                 }
    //                 //

    //                 //println!("gorot: {}, rb rotation: {}", gorot.x, body_transform.rotation);
    //             }
    //         }
    //     }
    // }
}
