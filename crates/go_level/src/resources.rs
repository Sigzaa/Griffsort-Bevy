use bevy::prelude::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
pub struct LoadedMap {
    scene_handle: Handle<Scene>,
}

// impl AssociatedAsset for Map {
//     fn asset_path(&self) -> &str {
//         match self {
//             Map::TestingPolygon => "maps/sky_roof.glb#Scene0",
//         }
//     }
// }
trait AssociatedAsset {
    /// Returns path to associated asset
    fn asset_path(&self) -> &str;
}
