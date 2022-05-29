use bevy::prelude::*;

#[derive(Default)]
pub struct CharPlugin<C: 'static> {
    pub char_type: C,
}
impl<C: Character + Send + Sync + Copy> CharPlugin<C> {
    pub fn new(char_type: C) -> Self {
        Self { char_type }
    }
}

impl<C: Character + Send + Sync + Copy> Plugin for CharPlugin<C> {
    fn build(&self, app: &mut App) {
        app.add_plugin(self.char_type);
    }
}

pub trait Character : Plugin{
    fn movement() {}
    fn shoot() {}
    fn spawn() {}
    fn hit() {}
}


// fn spawn(
//     mut spawn_reader: EventReader<SpawnCharacter>,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut commands: Commands,
//     mut char_list: ResMut<CharList>,
// ) {
//     for spawn in spawn_reader.iter() {
//             let team = spawn.1;
//             let id = char_list.0.len();
//             let entity_id = commands
//                 .spawn()
//                 .insert_bundle(PbrBundle {
//                     mesh: meshes.add(Mesh::from(bevy::prelude::shape::Cube { size: 0.5 })),
//                     material: materials.add(StandardMaterial {
//                         base_color: Color::rgba(0.7, 0.2, 0.3, 0.5),
//                         ..Default::default()
//                     }),
//                     transform: Transform::from_xyz(id as f32 * 2.0, 40.5, 15.0),
//                     ..Default::default()
//                 })
//                 .insert(Spawn {
//                     respawn_coords: Vec3::new(id as f32 * 2.0, 40.5, 15.0),
//                 })
//                 .insert_bundle(State {
//                     character_name: CharName("Soul"),
//                     team: Team(team as i16),
//                     id: Id(id),
//                     hor_vel: Speed(4.4),
//                     hp: Hp(1000),
//                     ..Default::default()
//                 })
//                 .insert(RigidBody::Dynamic)
//                 .insert(Velocity {
//                     linvel: Vec3::ZERO,
//                     angvel: Vec3::ZERO,
//                 })
//                 .insert(LockedAxes::ROTATION_LOCKED)
//                 .insert(Collider::ball(0.5))
//                 .insert(GravityScale(0.))
//                 .insert(GoInputs {
//                     //forward: true,
//                     ..Default::default()
//                 })
//                 .insert(Soul) // Change to Enum
//                 .insert(Core)
//                 .insert(ToExtend)
//                 .id();
//             char_list.0.push(entity_id);
//         }
// }
