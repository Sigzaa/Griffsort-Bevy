    // fn float<C: Component>(
    //     mut q_sel: Query<(&mut ExternalForce, &Transform, &RideHeight, &Velocity), With<C>>,
    //     rapier_context: Res<RapierContext>,
    // ) {
    //     for (mut spring, transform, ride_height, velocity) in q_sel.iter_mut() {
    //         let ray_pos = transform.translation;
    //         let ray_dir = Vec3::new(0., -4., 0.);
    //         let max_toi = 0.5;
    //         let solid = false;
    //         let groups = InteractionGroups::all();
    //         let filter = None;

    //         if let Some((entity, toi)) =
    //             rapier_context.cast_ray(ray_pos, ray_dir, max_toi, solid, groups, filter)
    //         {
    //             println!(
    //                 "ride height: {}, toi: {}, spring force: {}",
    //                 ride_height.0, toi, spring.force[1]
    //             );

    //             let ray_dir_vel = ray_dir.dot(velocity.linvel);
    //             let other_dir_vel = ray_dir.dot(Vec3::ZERO);
    //             let rel_vel = ray_dir_vel - other_dir_vel;
    //             let x = (toi - ride_height.0) * 20.;

    //             let force = (120. * x) - (rel_vel * 2.0);
    //             spring.force = ray_dir * force;

    //         } else {
    //             spring.force = Vec3::ZERO;
    //         }
    //     }
    // }