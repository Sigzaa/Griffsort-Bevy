use bevy::prelude::*;
use crate::game::components::{ * , player_states::*, filters::*};
use hl2_console::*;
mod hl2_console;

pub struct Console;
impl Plugin for Console {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(ConsolePlugin)
        .insert_resource(ConsoleConfiguration {
            ..Default::default()
        })
        .add_system(console_config)
        .add_system(listen_to_console_events)
        ;
    }
}

fn console_config(

    mut config: ResMut<ConsoleConfiguration>,
    mut windows: ResMut<Windows>
){

    let window = windows.get_primary_mut().unwrap();
        
    config.height = window.height() * 0.75;
    config.width = window.width() * 0.75;
    config.left_pos = window.width() * 0.125;
    config.top_pos = window.height() * 0.125;
}

fn listen_to_console_events(
    mut console_events: EventReader<ConsoleCommandEntered>,
    mut console_line: EventWriter<PrintConsoleLine>,
    mut writer: EventWriter<SpawnCharacter>,
    q_players: Query<(Entity, &Id, &Hp, &CharName, Option<&Selected>), With<Core>>,
    mut binded_id: ResMut<BindedId>,
) {
    for event in console_events.iter() {
        println!{"com: {}, argue: {}", event.command, event.args}
        // event has 2 fields, commands and args
        if event.command == "spawn"{
            //println!("args: {}", event.args);
            if event.args == "root"{
                
                writer.send(SpawnCharacter("Root", 3,1));
                console_line.send(PrintConsoleLine::new("Spawning Cover".to_string()));
            }
        }
        if event.command == "print"{
            for (entity, id, hp, name ,selected) in q_players.iter(){
                let mut sel = "";
                if let Some(_selected) = selected{
                    sel = "Selected";  
                }
                let output = format!("Entity Id: {}, Player Id:{}, Name: {}, Hp: {}  {}", entity.to_bits(), id.0, name.0, hp.0, sel);
                console_line.send(PrintConsoleLine::new(output.to_string()));
            }
        }
        if event.command == "select"{
            binded_id.0 = event.args.parse().unwrap();
        }
    }
}