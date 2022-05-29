GriffsOrt Characters -> gocha

Managing everything related to characters.

Api this module provides:

    Events:
        *every of this events will be checked on the server*

        Spawn(Type, Team): Spawning character.
        Respawn(Id, Pos),
        Spawnpoint(Id, Pos),
        Despawn(Id): Despawning character.
        Kill(Id): Killing character.
        Select(Id): Bind inputs and camera to another char.
        ChangeGameMode(Mode{ NoClip, Player, Invincible }, Id),
        Teleport(Id, Vec3),


    Components:
        Read-And-Write:
            LocalRespawnPeriod(Option<f32>): Uses only for special chars. None by default.

    Resources:
        Read-Only:
            Chars: Vec bevy-entities of chars.
            Died: Vec of died chars with respawn timer.


        Read-And-Write:
            RespawnPeriod(f32): Global respawn period.
            IsActiveInputs(bool): pause inputs to chars.
            IsActiveSim(bool): pause of physic simulation.
        

        

