mod characters_impl;
mod commands_impl;
mod resources;
mod temp;
pub mod heroes_mapping;

use actions::{ActionsPlugin, update_inputs};
use characters_impl::Characters;
use commands_impl::ConsoleCommands;
use go_level::plugin::Level;
use gs_inspector::Inspector;
use gs_states::StatesPlugin;
use gs_ui::UI;
use heroes::{Selected, Heroes};
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

        // Bevy default plugins
        .add_plugins(DefaultPlugins)


        // Handling states of this game
        .add_plugin(StatesPlugin)


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
        .add_system(update_inputs::<Selected, Action>)


        // Debugging and Hardcode 
        .add_plugin(TempPlugin)
        

        // Character controller
        .add_plugin(Heroes)


        // Implementation of heroes using HeroesPlugin APIs
        .add_plugin(Characters)


        .add_plugin(Level)


        .add_plugin(UI)


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