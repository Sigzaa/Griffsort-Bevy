use std::{fs, path::Path};

use bevy::prelude::*;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};

use crate::Synx;

pub(crate) fn watch_for_changes<T: Default + Sync + Send + 'static + Serialize>(
    config: Res<T>,
    path: Res<Synx<T>>,
) {
    if config.is_changed()
    {
        let pretty = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let s = to_string_pretty(&*config, pretty).expect("Serialization failed");

        fs::write(path.path, s).unwrap();
    }
}

pub(crate) fn load_config<
    T: for<'de> Deserialize<'de> + Default + Sync + Send + 'static + Serialize,
>(
    mut config: ResMut<T>,
    path: Res<Synx<T>>,
) {
    if !Path::new(path.path).is_file()
    {
        info!(
            "There is no config file at the path {}, creating new one",
            path.path
        );

        let pretty = PrettyConfig::new()
            .depth_limit(2)
            .separate_tuple_members(true)
            .enumerate_arrays(true);
        let s = to_string_pretty(&T::default(), pretty).expect("Serialization failed");

        fs::write(path.path, s).unwrap();
    }
    else
    {
        let config_str = fs::read_to_string(path.path).unwrap();
        *config = ron::from_str(&config_str).unwrap();
    }
}
