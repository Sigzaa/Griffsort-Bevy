use actions::Actions;
use bevy::prelude::*;
use heroes::*;
use keyframe::{ease, functions::*};
use std::collections::HashSet;

use crate::{characters_impl::Jacqueline, resources::Action};

use super::resources::*;

pub struct QAbil;

impl Plugin for QAbil {
    fn build(&self, app: &mut App) {
        app.add_system(r#use);
    }
}

fn r#use(
	mut jacq: Query<(&MarksLinks, &mut MarksCD, &Actions<Action>)>,
	mut markql: Query<&mut MarkState>,
	mut commands: Commands,
){
	for (marks_link, mut mcd, act) in &mut jacq{
		if act.just_pressed(Action::Abil2) {

		}
	}
}

fn boom(){

}

fn animate(){


}

