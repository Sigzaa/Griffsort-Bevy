use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_common_assets::ron::RonAssetPlugin;
use crate::Sensitivity;

use super::resources::*;
use iyes_loopless::prelude::*;
pub struct Config;
impl Plugin for Config {
    fn build(&self, app: &mut App) {
        app


        .insert_resource(ClientConfig::default())

        .add_plugin(RonAssetPlugin::<ClientConfig>::new(&["ron"]))
        .add_event::<ReloadRequest>()

        .add_startup_system(load_config)
        .add_system(reload_config)
        .add_system(apply_config.run_if_resource_exists::<Handle<ClientConfig>>())
        //.insert_resource(Msaa { samples: 16 })
        ;
    }
}
fn load_config(mut commands: Commands, asset_server: Res<AssetServer>){
    let handle: Handle<ClientConfig> = asset_server.load("../config/config.ron");
    println!("load");
    commands.insert_resource(handle);
}

fn reload_config(mut commands: Commands, asset_server: Res<AssetServer>, mut request: EventReader<ReloadRequest>){
    for req in request.iter(){
        let handle: Handle<ClientConfig> = asset_server.load("../config/config.ron");
        println!("load");
        commands.insert_resource(handle);
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
