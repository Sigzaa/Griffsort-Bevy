How to spawn character?

fn spawn_character(
    mut spawn_request: EventWriter<SpawnCharacterRequest>,
){
    spawn_request.send(SpawnCharacterRequest{
        team: 1,
        character: 1
    });
}

Implementation guide:

    ATTENTION!!!
    You should add tag to each character.