# SCLI

A CLI tool for messing around with my sonos speakers

```sh
scli 0.1.0

USAGE:
    scli <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    bass            Displays bass level, optionaly sets bass level
    clear-queue     Clears the queue
    help            Prints this message or the help of the given subcommand(s)
    info            Displays info about all rooms
    join            Joins one speaker to another speaker group
    leave           Specified speaker will leave any group it is currently in
    loudness        Sets loudness on if off, Loudness off if on
    mute            Mutes speaker if unmuted, Unmutes if muted
    next            Skips to the next track for a specified room
    pause           Pauses a specified room
    play            Plays specified room
    previous        skips back to the previous track for a specified room
    queue           Displays the currently queued tracks
    queue-end       Queues a new track at the end of the queue
    queue-next      Queues a new track at the top of the queue
    remove-track    Removes specified track in queue (Tracks are zero-indexed)
    repeat-all      Sets repeat all on (repeats queue)
    repeat-off      Sets repeat off
    repeat-one      Sets repeat one on (repeats one track)
    set-volume      Sets the volume for a specified room
    shuffle         Sets shuffle on if off, off if on
    stop            Stops specified room
    track           Displays the track for a specified room
    treble          Displays treble level, optionaly sets treble level
    volume          Displays the volume for a specified room
```
