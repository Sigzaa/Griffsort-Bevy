use super::*;

pub(crate) fn attack(
    char: Query<(&RayPointingOn, &Team, &Actions<Action>), (With<Jacqueline>, Without<Dead>)>,
    mut enemy: Query<(&mut Hp, &Team), (With<Hero>, Without<Dead>)>,
    _time: Res<Time>,
) {
    for (pointing_on, team, act) in char.iter() {
        if let Some((entity, toi)) = pointing_on.0{
            println!("entity: {entity:?}, toi: {toi}");
        }
        // let en = enemy.get_mut(pointing_on.target);

        // if en.is_ok() {
        //     let (mut hp, en_team) = en.unwrap();

        //     if act.pressed(Action::Shoot) && team != en_team {
        //         hp.0 -= 10;
        //     }
        //     //println!("enemy hp: {:?}", hp.0);
        // }
    }
}