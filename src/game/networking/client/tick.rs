use bevy_simple_networking::Transport;
use super::components::*;
use crate::game::components::{filters::*, player_data::*, *};
use crate::game::networking::additional::*;
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_rapier3d::prelude::*;

pub fn send_message(
    time: Res<Time>,
    mut transport: ResMut<Transport>,
    mut q_selected: Query<(&Id, &mut Control, &mut Transform, &mut HeadRotation), With<Selected>>,
    serv_addr: ResMut<ServerAddr>,
    my_id: ResMut<MyId>,
    mut inp_his: ResMut<InputHistory>,
    mut tick_counter: ResMut<TickCounter>,
) {
    for (_id, ctrl, transform, head_rotation) in q_selected.iter_mut() {
        //
        let msg = msg_structure {
            ctrl: ctrl.clone(),
            rotation: transform.rotation,
            head_rotation: head_rotation.0,
            id: my_id.0,
            tick: tick_counter.0,
        }
        .pack();
        //println!("msg: {}", msg);

        // msg_pack : msg_struct -> string to send.
        // msg_unpack : received string -> msg_struct.

        //let msg = msg_s.pack(); // Can also use msg_pack(msg_s)

        // Filling the history buffer.
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

#[derive(Default)]
pub struct IsStarted(pub bool);

pub fn predict_sys(
    mut q_core: Query<
        (
            &Id,
            &mut Control,
            &mut Transform,
            &mut HeadRotation,
            &ColliderShapeComponent,
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<Selected>,
    >,
    mut tick_counter: ResMut<TickCounter>,
    mut is_started: ResMut<IsStarted>, // Will be replaced with reconsiliation system
    
) {
    //println!("tick {}:", tick_counter.0);
    for (id, mut ctrl, mut transform, mut head_rotation, collider, rb_position, mut rb_velocity) in
        q_core.iter_mut()
    {
        transform.translation = rb_position.position.translation.into();

        // Simulating -->
        let mut rb_vel = Vec3::ZERO;

        // Uses to sync initial conditions
        // Remove this after creating reconciliation system -->
        if !is_started.0 && ctrl.velocity == Vec3::ZERO {
        } else {
            is_started.0 = true;
            rb_vel = Vec3::new(0., rb_velocity.linvel[1], 0.);
        } //<--
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
            &ColliderShapeComponent,
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<Selected>,
    >,
) {
    for (mut ctrl, mut transform, mut head_rotation, collider, rb_position, mut rb_velocity) in
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
pub fn update_tick(mut s_tick: ResMut<TickCounter>) {
    s_tick.0 += 1;
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
            &ColliderShapeComponent,
            &RigidBodyPositionComponent,
            &mut RigidBodyVelocityComponent,
        ),
        With<Selected>,
    >,
    mut tick_counter: ResMut<TickCounter>,
){
    if q_selected.is_empty(){
        return;
    }
    let (mut ctrl, mut transform, mut head_rotation, collider, rb_position, mut rb_velocity) =
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
