use crate::*;
use bevy::prelude::*;

pub fn cast_shape(
    rapier_context: &RapierContext,
    shape_pos: Vect,
    shape_rot: Rot,
    shape_vel: Vect,
    shape: &Collider,
    max_toi: Real,
    filter: QueryFilter<'_>,
    conf: &HeroesConfig,
) -> Option<(Entity, Toi)> {
    rapier_context.cast_shape(shape_pos, shape_rot, shape_vel, shape, max_toi, filter)
}

pub fn intersections_with_shape(
    rapier_context: &RapierContext,
    shape_pos: Vect,
    shape_rot: Rot,
    shape: &Collider,
    filter: QueryFilter<'_>,
    conf: HeroesConfig,
    callback: impl FnMut(Entity) -> bool,
) {
    rapier_context.intersections_with_shape(shape_pos, shape_rot, shape, filter, callback);
}
