use bevy::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use crate::Sensitivity;

pub use super::resources::*;
use iyes_loopless::prelude::*;
pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(ClientConfig::default())
        .insert_resource(InputMap::default())
        .add_plugin(RonAssetPlugin::<ClientConfig>::new(&["ron"]))
        .add_plugin(RonAssetPlugin::<InputMap>::new(&["ron"]))
        .add_event::<ReloadRequest>()
        .add_startup_system(load_config)
        .add_system(apply_config.run_if_resource_exists::<Handle<ClientConfig>>())
        .add_system(apply_keycode_map.run_if_resource_exists::<Handle<InputMap>>())
        //.insert_resource(Msaa { samples: 16 })
        ;
    }
}
fn load_config(mut commands: Commands, asset_server: Res<AssetServer>)
{

    asset_server.watch_for_changes().unwrap();

    let handle1: Handle<ClientConfig> = asset_server.load("../config/config.ron");
    commands.insert_resource(handle1);

    let handle2: Handle<InputMap> = asset_server.load("../config/keycode-map.ron");
    commands.insert_resource(handle2);
}

fn apply_keycode_map(
    handle: Res<Handle<InputMap>>,
    mut config: ResMut<Assets<InputMap>>,
    mut keycode_map: ResMut<InputMap>,
){
    if let Some(file_map) = config.get_mut(&handle){
        *keycode_map = file_map.clone();
    } 
}

fn apply_config(    
    handle: Res<Handle<ClientConfig>>,
    mut config: ResMut<Assets<ClientConfig>>,
    mut client_config: ResMut<ClientConfig>,
    mut sens: ResMut<Sensitivity>
){
    if let Some(conf) = config.remove(handle.id){
        *client_config = conf;
        sens.0 = client_config.mouse_sensitivity.clone();
    } 
}
