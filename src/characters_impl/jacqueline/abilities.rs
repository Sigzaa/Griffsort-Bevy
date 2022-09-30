use super::{
    *,
};

pub(crate) fn attack(
    char: Query<(&RayPointingOn, &Team, &Actions<Action>), (With<Jacqueline>, Without<Dead>)>,
    mut enemy: Query<(&mut Hp, &Team), (With<Hero>, Without<Dead>)>,
    _time: Res<Time>,
) {
    for (pointing_on, team, act) in char.iter()
    {
        if let Some((entity, _toi)) = pointing_on.0
        {
            let en = enemy.get_mut(entity);
            if en.is_ok()
            {
                let (mut hp, en_team) = en.unwrap();

                if act.just_pressed(Action::Shoot) && team != en_team
                {
                    hp.0 -= 50;
                }
            }
        }
    }
}
