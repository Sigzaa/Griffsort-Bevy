mod characters_impl;
mod commands_impl;
pub mod heroes_mapping;
mod resources;
mod temp;
mod debugger;

use actions::{update_inputs, ActionsPlugin};
use bevy_debug_text_overlay::OverlayPlugin;
use bevy_draw_debug_tools::DebugLinesPlugin;
use characters_impl::Characters;
use commands_impl::ConsoleCommands;
use debugger::GSDebugger;
use go_level::plugin::Level;
use gs_inspector::Inspector;
use gs_states::{StatesPlugin, IntoConditionalSystem};
use gs_ui::UI;
use heroes::{Heroes, Selected};
use query_to_screen::QueryToScreenPlugin;
use resources::*;

use bevy::prelude::*;
use temp::TempPlugin;

/*
_____/\\\\\\\\\\\\_        ____/\\\\\\\\\_____        __/\\\\\\\\\\\_        __/\\\\\\\\\\\\\\\_        __/\\\\\\\\\\\\\\\_
 ___/\\\//////////__        __/\\\///////\\\___        _\/////\\\///__        _\/\\\///////////__        _\/\\\///////////__
  __/\\\_____________        _\/\\\_____\/\\\___        _____\/\\\_____        _\/\\\_____________        _\/\\\_____________
   _\/\\\____/\\\\\\\_        _\/\\\\\\\\\\\/____        _____\/\\\_____        _\/\\\\\\\\\\\_____        _\/\\\\\\\\\\\_____
    _\/\\\___\/////\\\_        _\/\\\//////\\\____        _____\/\\\_____        _\/\\\///////______        _\/\\\///////______
     _\/\\\_______\/\\\_        _\/\\\____\//\\\___        _____\/\\\_____        _\/\\\_____________        _\/\\\_____________
      _\/\\\_______\/\\\_        _\/\\\_____\//\\\__        _____\/\\\_____        _\/\\\_____________        _\/\\\_____________
       _\//\\\\\\\\\\\\/__        _\/\\\______\//\\\_        __/\\\\\\\\\\\_        _\/\\\_____________        _\/\\\_____________
        __\////////////____        _\///________\///__        _\///////////__        _\///______________        _\///______________
___________________        _____/\\\\\\\\\\\___        _______/\\\\\______        ____/\\\\\\\\\_____        __/\\\\\\\\\\\\\\\_
 ___________________        ___/\\\/////////\\\_        _____/\\\///\\\____        __/\\\///////\\\___        _\///////\\\/////__
  ___________________        __\//\\\______\///__        ___/\\\/__\///\\\__        _\/\\\_____\/\\\___        _______\/\\\_______
   ___________________        ___\////\\\_________        __/\\\______\//\\\_        _\/\\\\\\\\\\\/____        _______\/\\\_______
    ___________________        ______\////\\\______        _\/\\\_______\/\\\_        _\/\\\//////\\\____        _______\/\\\_______
     ___________________        _________\////\\\___        _\//\\\______/\\\__        _\/\\\____\//\\\___        _______\/\\\_______
      ___________________        __/\\\______\//\\\__        __\///\\\__/\\\____        _\/\\\_____\//\\\__        _______\/\\\_______
       ___________________        _\///\\\\\\\\\\\/___        ____\///\\\\\/_____        _\/\\\______\//\\\_        _______\/\\\_______
        ___________________         ___\///////////_____        ______\/////_______        _\///________\///__        _______\///________
*/

fn main() {
    App::new()
        
        // Handling states of this game
        .add_plugin(StatesPlugin)

        // Debugging and Hardcode
        .add_plugin(TempPlugin)
        
        // Bevy default plugins
        .add_plugins(DefaultPlugins)

        // For screen printing
        .add_plugin(OverlayPlugin { font_size: 24.0, fallback_color: Color::AZURE, ..default() })
        
        .add_plugin(DebugLinesPlugin::default())

        // Console + Configs
        .add_plugin(Inspector::new(env!("CARGO_PKG_VERSION")))

        // Implementation of console commands for Inspector plugin
        .add_plugin(ConsoleCommands)

        // Inputs Handler. List of Actions (Inputs) in resources/action
        .add_plugin(ActionsPlugin::<Action, Selected>::new(
            "./config/conf.ron",
            "./config/def.ron",
        ))

        // Using ActionsPlugin API to work it properly
        //.add_system_to_stage(CoreStage::First, update_inputs::<Selected, Action>)

        // Character controller
        .add_plugin(Heroes)

        // Implementation of heroes using HeroesPlugin APIs
        .add_plugin(Characters)

        .add_plugin(GSDebugger)

        .add_plugin(Level)

        .add_plugin(UI)

        .add_plugin(QueryToScreenPlugin)

        .run();
}

/*

                               ****   *****   ***,
                        ****, ********************. *****
                  ***   *********************************   ***
                  ********************    ,********************
            ***,  ***************  *****,****   ***************  ,***
            ***************           *****           ***************
             **********                                  ***********
       *************                                        .*************
        **********************************************         **********
      ****************************************************      ,**********
    ********************************************************     ************
     ********************************************************   ***********,
    ******    ***,   *************            ***************  ***.    ******
 **********. *****   *************             *************   ****. .**********
    ***********      *************************************       ,***********
   *********         ***********************************             *********
 **********          ************************************            ***********
   *********         *************************************        ************
   .********         *************           **************       ***********
 ***********.        *************            **************    ,***************
    *************************************.    *******************************
     ************************************.     *****************************
    *************************************.      *****************************
       ,*********************************.        ************************.
        ***********    ***                             **     ***********
       ********************                          *********************
             ********    ***                         ***    ********
            **********  ****                        ****   **********
            **,   ******************         ******************   ,*,
                  *********************************************
                  **    *********************************    **
                        ****. ,*******************  .****
                               ,***   ****,   ***,
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
...............................@@@@@@@@@@@..................................................................................................
..............................@@@@@/@@@@@,..............@@@@@@@@@@@@@....&@@@@@@@@@@@@.@@@@@........@@@@@..@@@@*......@@@@@.................
....................@@@@@@@@@@@@@@@@@@@@######..........@@@@@@@@@@@@@@@.,@@@@@@@@@@@@@.%@@@@@......%@@@@@*.@@@@@@....@@@@@@.................
.......................,@@@@@@@@@@@@@@@@@(#######.......@@@@@.....@@@@@.,@@@@@..........@@@@@@.....@@@@@@...@@@@@@..@@@@@@..................
.....................@@@@@@@@@@@@@@@@@@@@@.##(..........@@@@@@@@@@@@@@..,@@@@@@@@@@@@....@@@@@&...@@@@@@.....*@@@@@@@@@@@...................
.................@@@@@@@@@@@@@@@@@@@@@@@@@@##,////......@@@@@@@@@@@@@@@.,@@@@@@@@@@@@*....@@@@@...@@@@@........@@@@@@@@.....................
..............@@@@@@%##%@@@@@@@@@@@@@@@@@@.##,/.//......@@@@@.....@@@@@@,@@@@@.............@@@@@.@@@@@/.........%@@@@@......................
...............................@@@@@@@@@@.##.//////.....@@@@@....&@@@@@@,@@@@@@@@@@@@@.....,@@@@@@@@@#..........%@@@@@......................
..............................@@@@@@@@@.##.///..........@@@@@@@@@@@@@@&..@@@@@@@@@@@@@&......@@@@@@@,...........(@@@@@......................
.........................#@@@@@@@@@@.///////................................................................................................
....................@@@@@@@&................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
............................................................................................................................................
*/
