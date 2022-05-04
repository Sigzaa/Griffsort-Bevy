use bevy_simple_networking::Transport;
use super::components::*;
use crate::game::components::{filters::*, player_data::*, *};
use crate::game::networking::shared::additional::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;
use bevy::ecs::schedule::{ShouldRun::*, ShouldRun};

pub fn send_message(
    time: Res<Time>,
    mut transport: ResMut<Transport>,
    mut q_selected: Query<(&Id, &mut Control, &mut Transform, &mut HeadRotation), With<Selected>>,
    serv_addr: ResMut<ServerAddr>,
    my_id: ResMut<MyId>,
    mut inp_his: ResMut<InputHistory>,
    tick_counter: ResMut<Tick>,
) {
    for (_id, ctrl, transform, head_rotation) in q_selected.iter_mut() {
        //
        //println!("send");
        let msg = msg_structure {
            ctrl: ctrl.clone(),
            rotation: transform.rotation,
            head_rotation: head_rotation.0,
            id: my_id.0,
            tick: tick_counter.0,
        }
        .pack();


        // msg_pack : msg_struct -> string to send.
        // msg_unpack : received string -> msg_struct.

        //let msg = msg_s.pack(); // Can also use msg_pack(msg_s)

        // Filling the history buffer. AKA Snapshot
        inp_his.0.push(HisPack {
            tick: tick_counter.0,
            input: ctrl.clone(),
            delta_seconds: time.delta_seconds(), // Remove it
            rotation: transform.rotation,
            translation: transform.translation, // Uses for less calculation
            st_tr: Vec3::new(0., 0., 0.),
        });
        // Limiting buffer.
        if inp_his.0.len() > 10000 {
            inp_his.0.remove(0); // !Change to pop or something. Or prevent pushing to the his.
        }
        if serv_addr.0.len() >= 1 {
            // Check for avaliable server in another place
            // Sending via UDP.
            transport.send("main", serv_addr.0[0], &msg);
        }
    }
}


pub fn predict_sys(
    mut q_core: Query<
        (
            &Id,
            &mut Control,
            &mut Transform,
            &mut HeadRotation,
            &mut Velocity,
        ),
        With<Core>,
    >,
    mut tick_counter: ResMut<Tick>,
    mut is_started: ResMut<IsStarted>, // Will be replaced with reconsiliation system
    
) {
    //println!("tick {}:", tick_counter.0);
     //println!("predict");
    // println!();
    for (id, mut ctrl, mut transform, mut head_rotation, mut rb_velocity) in
        q_core.iter_mut()
    {

        // Simulating -->
        let mut rb_vel = Vec3::ZERO;

        // Uses to sync initial conditions
        // Remove this after creating reconciliation system -->
        if !is_started.0 && ctrl.velocity == Vec3::ZERO {
        } else {
            is_started.0 = true;
            rb_vel = Vec3::new(0., rb_velocity.linvel[1], 0.);
        } //<--
        //println!("{}{}", ctrl.velocity, rb_vel);
        rb_velocity.linvel = (ctrl.velocity + rb_vel).into();
        //println!("pos: {}", rb_position.position.translation);

        //println!("rot: {} {}", transform.rotation, ctrl.forward);
        //println!("pos: {:?} + {}", rb_position.position.translation, ctrl.velocity);

    }
}

pub fn camera_movement(
    mut motion_evr: EventReader<MouseMotion>,
    mut q_camera: Query<&mut Transform, (With<ThreeDCam>, Without<Selected>)>,
    mut q_selected: Query<
        (
            &mut Control,
            &mut Transform,
            &mut HeadRotation,
            &mut Velocity,
        ),
        With<Selected>,
    >,
) {
    for (mut ctrl, mut transform, mut head_rotation, mut rb_velocity) in
        q_selected.iter_mut()
    {
        for mut camera in q_camera.iter_mut() {
            let mut delta_x = 0.;
            //ctrl.delta_x = 1.;
            for ev in motion_evr.iter() {
                delta_x = -ev.delta.x;
                //transform.rotation *= Quat::from_rotation_y( -ev.delta.x * SENSITIVITY);
                //camera.rotation *= Quat::from_rotation_x(-ev.delta.y * SENSITIVITY);
                //head_rotation.0 = camera.rotation;
            }
            //let ang_vel = Vec3::new(0., delta_x, 0.);
            //rb_velocity.angvel = ang_vel.into();
        }
    }
}
pub fn update_tick(mut s_tick: ResMut<Tick>) {
    s_tick.0 += 1;
    //println!("tick end");
    //println!();
    //println!("server tick end: {}", s_tick.0);
}

pub fn fill_his_sys(
    mut inp_his: ResMut<InputHistory>,
    mut q_selected: Query<
        (
            &mut Control,
            &mut Transform,
            &mut HeadRotation,
            &mut Velocity,
        ),
        With<Selected>,
    >,
    mut tick_counter: ResMut<Tick>,
){
    if q_selected.is_empty(){
        return;
    }
    let (mut ctrl, mut transform, mut head_rotation, mut rb_velocity) =
    q_selected.single_mut();


    inp_his.0.push(HisPack {
        tick: tick_counter.0 - 1,
        input: ctrl.clone(),
        delta_seconds: 0.,
        rotation: transform.rotation,
        translation: transform.translation,
        st_tr: Vec3::ZERO,
    });
    // Limiting buffer.
    if inp_his.0.len() > 1000 {
        inp_his.0.remove(0); // !Change to pop or something.
    }
}

pub fn should_tick(time: Res<Time>, mut timer: ResMut<TPS>, mut iter: ResMut<iter_count>) -> ShouldRun {
    if iter.0 == 0 {
        return ShouldRun::Yes;
    }
    iter.0 -= 1;
    return ShouldRun::YesAndCheckAgain;
}

pub fn tick_more_then_zero(tick_counter: Res<Tick>, mut inp_buf: ResMut<InpBuf>) -> ShouldRun {
    if tick_counter.0 > 0 && inp_buf.tick > 0 {
        return ShouldRun::Yes;
    }
    ShouldRun::No
}
pub fn rollback(is_rollback: Res<IsRollback>) -> ShouldRun {
    if is_rollback.0 {
        return Yes;
    } else {
        return No;
    }
}

pub fn prepare_rollback(mut iter: ResMut<iter_count>, mut is_rollback: ResMut<IsRollback>) {
    if iter.0 > 0 {
        is_rollback.0 = true;
    } else {
        is_rollback.0 = false;
    }
}
pub fn check_for_desync_sys(
    inp_his: ResMut<InputHistory>,
    tick_counter: Res<Tick>,
    mut inp_buf: ResMut<InpBuf>,
    mut iter: ResMut<iter_count>,
) {
    let tick = inp_buf.tick as usize;

    let client_tick = inp_his.0[tick].tick as usize;
    let client_pos = inp_his.0[2 * tick - client_tick].translation;
    let client_tick = inp_his.0[2 * tick - client_tick].tick as usize;

    let server_pos = inp_buf.pos;
    if client_pos != server_pos {
        // Time travelling ->
        //println!("desync on {}: server: {} client on {}: {}, w: {}", tick ,server_pos, client_tick, client_pos, client_forward );
        iter.0 = tick_counter.0 - client_tick as i32;
    }
}

pub fn setup_client(mut commands: Commands, mut q_selected: Query<&mut Transform, With<Selected>>) {
    for mut transform in q_selected.iter_mut() {
        // let locked_dofs = RigidBodyMassPropsFlags::ROTATION_LOCKED_X
        //     | RigidBodyMassPropsFlags::ROTATION_LOCKED_Z
        //     | RigidBodyMassPropsFlags::ROTATION_LOCKED_Y;

        //     let rigid_body = RigidBodyBundle {
        //         //body_type: RigidBodyTypeComponent(RigidBodyType::KinematicVelocityBased),
        //         position: transform.translation.into(),
        //         velocity: RigidBodyVelocity {
        //             linvel: Vec3::new(0.0, 0.0, -2.0).into(),
        //             angvel: Vec3::new(0.0, 0.0, 0.0).into(),
        //         }
        //         .into(),
        //         forces: RigidBodyForces {
        //             gravity_scale: 2.0,
        //             ..Default::default()
        //         }
        //         .into(),
        //         activation: RigidBodyActivation::cannot_sleep().into(),
        //         ccd: RigidBodyCcd {
        //             ccd_enabled: true,
        //             ..Default::default()
        //         }
        //         .into(),
        //         mass_properties: locked_dofs.into(),
        //         ..Default::default()
        //     };
        //     let collider = ColliderBundle {
        //         shape: ColliderShape::ball(0.5).into(),
        //         material: ColliderMaterial {
        //             restitution: 0.,
        //             ..Default::default()
        //         }
        //         .into(),
        //         ..Default::default()
        //     };
    }
}
