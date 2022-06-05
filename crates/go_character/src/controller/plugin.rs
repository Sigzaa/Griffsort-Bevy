use super::resources::*;
use bevy::prelude::{*, shape::*};
use bevy::render::camera::{ActiveCamera, CameraTypePlugin};
pub use bevy_rapier3d::prelude::*;
use go_core::{Character::*, GoInputs, GoRot};
use bevy::render::camera::Camera3d;

impl<T: Character<T> + Send + Sync + Copy + Component> Plugin for Controller<T> {
    fn build(&self, app: &mut App) {
        app.add_plugin(self.char_type)
            .add_plugin(CameraTypePlugin::<CharacterCamera>::default())
            .add_event::<SpawnChar>()
            .add_system(T::spawn)
            .add_system(T::extend::<T>)
            .add_system(T::sync_components)
            .add_system(T::sync_camera)
            .add_system(T::movement::<T>)
            .add_system(T::sync_rotation::<T>)
            ;
    }
}

impl<T: Character<T> + Send + Sync + Copy> Controller<T> {
    pub fn new(char_type: T) -> Self {
        Self { char_type }
    }
}

pub trait Character<T: Character<T>>: Plugin {
    fn movement<C: Component>(
        mut q_sel: Query<(&GoInputs, &mut Velocity, &Transform), With<Selected>>,
        time: Res<Time>,
    ){
        const MAX_SPEED: f32 = 9.;
        for (inputs, mut velocity, transform) in q_sel.iter_mut(){

            let coef = time.delta_seconds() * 15.;

            if inputs.forward == 1{
                velocity.linvel += transform.forward() * coef;
            }
            if inputs.back == 1{
                velocity.linvel += transform.back() * coef;
            }
            if inputs.left == 1 {
                velocity.linvel += transform.left() * coef;
            }
            if inputs.right == 1 {
                velocity.linvel += transform.right() * coef;
            }
        }
    }
    fn jump() {}
    fn slab_handle() {}
    fn sprint() {}

    fn extend<C: Component>(
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        mut commands: Commands,
        query: Query<(Entity, &Id), Added<C>>,
    ) {
        for (entity, id) in query.iter() {
            println!("automatic extention has been worked");

            commands
                .entity(entity)
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(Capsule {
                        radius: 0.3,
                        ..Default::default()
                    })),
                    material: materials.add(StandardMaterial {
                        base_color: Color::rgba(0.9, 0.2, 0.1, 0.5),
                        alpha_mode: AlphaMode::Blend, 
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(2.0, 5.5, -id.0 as f32 * 1.5),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(
                            PbrBundle {
                                mesh: meshes.add(Mesh::from(Cube {
                                    size: 0.2,
                                })),
                                material: materials.add(StandardMaterial {
                                    base_color: Color::DARK_GRAY,
                                    metallic: 0.1,
                                    ..Default::default()
                                }),
                                transform: Transform::from_xyz(0., 0.4, 0.),
                                ..Default::default()
                            }
                        )
                        .insert(ZHead)
                        .with_children(|parent| {
                            parent
                                .spawn_bundle(PerspectiveCameraBundle {
                                    transform: Transform::from_xyz(0., 0., 0.),
                                    perspective_projection: PerspectiveProjection {
                                        fov: 1.5,
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                })
                                .insert(CharacterCamera)
                                
                                ;
                        });
                })
                ;

            commands
                .entity(entity)
                .insert(Collider::capsule(
                    Vec3::new(0., -0.4, 0.),
                    Vec3::new(0., 0.4, 0.),
                    0.4,
                ))
                .insert(Velocity::default())
                .insert(ExternalForce::default())
                .insert(ExternalImpulse::default())
                .insert(GravityScale(2.))
                .insert(LockedAxes::ROTATION_LOCKED)
                .insert(RigidBody::Dynamic);

            commands
                .entity(entity)
                .insert_bundle(Config::default())
                .insert(GoRot::default())
                .insert(GoInputs::new())
                .insert(ChCore);
        }
    }
    fn sync_rotation<C: Component>(
        mut q_head: Query<(&Children, &mut Transform), (With<ZHead>, Without<C>)>,
        mut q_sel: Query<(&GoRot, &mut Transform, &Children), With<C>>,
        mut q_cam: Query<&mut Transform, (With<CharacterCamera>, Without<C>, Without<ZHead>)>,
    ){
        for (gorot, mut body_transform, children) in q_sel.iter_mut(){
            for &child in children.iter(){

                let (children, mut head_transform) = q_head.get_mut(child).unwrap();

                for &child in children.iter(){

                    let mut cam_transform = q_cam.get_mut(child).unwrap();
                    body_transform.rotation = gorot.x;
                    cam_transform.rotation = gorot.y;
                    head_transform.rotation = gorot.z;
                } 
            }
        }
    }

    fn sync_camera(
        selected_id: Res<SelectedId>,
        q_camera: Query<Entity, With<CharacterCamera>>,
        q_head: Query<&Children, With<ZHead>>,
        q_core: Query<(&Id, &Children), (With<ChCore>, Without<Killed>, Without<Selected>)>,
        mut active_camera: ResMut<ActiveCamera<Camera3d>>,
    ) {
        for (id, children) in q_core.iter() {

            if Some(id.0) == selected_id.0 {

                for &child in children.iter() {

                    let children = q_head.get(child).unwrap();

                    for &child in children.iter(){

                        let cam_ent = q_camera.get(child).unwrap();
                        active_camera.set(cam_ent);
                    }
                    
                }
            }
        }
    }

    fn sync_components(
        q_core: Query<(&Id, Entity), (With<ChCore>, Without<Killed>)>,
        selected_id: Res<SelectedId>,
        mut commands: Commands,
    ) {
        for (id, ent) in q_core.iter(){  
            if Some(id.0) != selected_id.0{
                commands.entity(ent).remove::<Selected>(); 
            } else {
                commands.entity(ent).insert(Selected);
            }
        }
    }
    fn spawn(spawn_request: EventReader<SpawnChar>, commands: Commands);
}
