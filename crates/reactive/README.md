Implementation of Authoritative dedicated server networking using Bevy engine.

Current features:

    Clients prediction,


Road map:

    Clients reconciliation (Rollback)
    Clients extrapolation,
    Chat,
    Commands forward,
    Lag compensation,



Usage:

    Tag Entities/Resources/Components which should be synced with bevy reflect.

    Set fixed timestep in physic engine.

    Pass "simulation step system" into crate stage.

    Pass all systems for synced Entities into crate stage.

Use limitations:

    Systems for synced Entities should be single threaded


It Depends on Corgee-rs

How does it work:

    Client sends keyboard input and mouse rotation to the server.

    Every 
