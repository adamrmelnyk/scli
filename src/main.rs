use futures::prelude::*;
use std::time::Duration;
use structopt::StructOpt;
use sonor::Speaker;
use sonor::RepeatMode;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Command {
    #[structopt(
        about = "Displays info about all rooms",
        help = "USAGE: info"
    )]
    Info,
    #[structopt(
        about = "Stops specified room",
        help = "USAGE: stop MyRoomName"
    )]
    Stop {
        name: String,
    },
    #[structopt(
        about = "Plays specified room",
        help = "USAGE: play MyRoomName"
    )]
    Play {
        name: String,
    },
    #[structopt(
        about = "Pauses a specified room",
        help = "USAGE: volume MyRoomName"
    )]
    Pause {
        name: String,
    },
    #[structopt(
        about = "Skips to the next track for a specified room",
        help = "USAGE: next MyRoomName"
    )]
    Next {
        name: String,
    },
    #[structopt(
        about = "skips back to the previous track for a specified room",
        help = "USAGE: previous MyRoomName")]
    Previous {
        name: String,
    },
    #[structopt(
        about = "Displays the track for a specified room",
        help = "USAGE: track MyRoomName"
    )]
    Track {
        name: String,
    },
    #[structopt(
        about = "Displays the volume for a specified room",
        help = "USAGE: volume MyRoomName"
    )]
    Volume {
        name: String,
    },
    #[structopt(
        about = "Sets the volume for a specified room",
        help = "USAGE: set-volume MyRoomName 11"
    )]
    SetVolume {
        name: String,
        volume: u16,
    },
    #[structopt(
        about = "Displays the currently queued tracks",
        help = "USAGE: queue MyRoomName"
    )]
    Queue {
        name: String,
    },
    #[structopt(
        about = "Mutes speaker if unmuted, Unmutes if muted",
        help = "USAGE: mute MyRoomName"
    )]
    Mute {
        name: String,
    },
    #[structopt(
        about = "Displays bass level, optionaly sets bass level",
        help = "USAGE: bass MyRoomName OR bass MyRoomName bass 1"
    )]
    Bass {
        name: String,
        opt: Option<i8>,
    },
    #[structopt(
        about = "Displays treble level, optionaly sets treble level",
        help = "USAGE: treble MyRoomName OR treble MyRoomName bass 1"
    )]
    Treble {
        name: String,
        opt: Option<i8>,
    },
    #[structopt(
        about = "Sets loudness on if off, Loudness off if on",
        help = "USAGE: loudness MyRoomName",
    )]
    Loudness {
        name: String,
    },
    #[structopt(
        about = "Removes specified track in queue (Tracks are zero-indexed)",
        help = "USAGE: remove-track MyRoomName 10",
    )]
    RemoveTrack {
        name: String,
        track_no: u32,
    },
    #[structopt(
        about = "Queues a new track at the top of the queue",
        help = "USAGE queue-next MyRoomName myuri metadata",
    )]
    QueueNext {
        name: String,
        uri: String,
        metadata: String,
    },
    #[structopt(
        about = "Queues a new track at the end of the queue",
        help = "USAGE: queue-end MyRoomName myuri metadata",
    )]
    QueueEnd {
        name: String,
        uri: String,
        metadata: String
    },
    #[structopt(
        about= "Clears the queue",
        help = "USAGE: clear-queue MyRoomName myuri metadata",
    )]
    ClearQueue {
        name: String,
    },
    #[structopt(
        about = "Sets shuffle on if off, off if on",
        help = "USAGE: shuffle MyRoomName",
    )]
    Shuffle {
        name: String,
    },
    #[structopt(
        about = "Sets repeat all on (repeats queue)",
        help = "USAGE: repeat-all MyRoomName",
    )]
    RepeatAll {
        name: String,
    },
    #[structopt(
        about = "Sets repeat one on (repeats one track)",
        help = "USAGE: repeat-one MyRoomName",
    )]
    RepeatOne {
        name: String,
    },
    #[structopt(
        about = "Sets repeat off",
        help = "USAGE: repeat-off MyRoomName",
    )]
    RepeatOff {
        name: String,
    },
    #[structopt(
        about = "Joins one speaker to another speaker group",
        help = "USAGE: join MyRoomName RoomToJoin",
    )]
    Join {
        name: String,
        speaker_to_join: String,
    },
    #[structopt(
        about = "Specified speaker will leave any group it's currently in",
        help = "USAGE: leave MyRoomName",
    )]
    Leave {
        name: String,
    },
    #[structopt(
        about = "Skips the current playing track a specified number of seconds",
        help = "USAGE: skip MyRoomName 10"
    )]
    Skip {
        name: String,
        seconds: i32,
    },
    #[structopt(
        about = "Skips the current playing track to a specified time",
        help = "USAGE: skip-to MyRoomName 30",
    )]
    SkipTo {
        name: String,
        seconds: u32,
    }
}

#[tokio::main]
async fn main() -> Result<(), sonor::Error> {
    let args = Command::from_args();
    return match args {
        Command::Info => info().await,
        Command::Stop { name } => stop(name).await,
        Command::Play { name } => play(name).await,
        Command::Pause { name } => pause(name).await,
        Command::Next { name } => next(name).await,
        Command::Previous { name } => previous(name).await,
        Command::Track { name } => track(name).await,
        Command::Volume { name } => volume(name).await,
        Command::SetVolume { name, volume} => set_volume(name, volume).await,
        Command::Queue { name } => queue(name).await,
        Command::Mute { name } => mute(name).await,
        Command::Bass { name, opt } => bass(name, opt).await,
        Command::Treble { name, opt } => treble(name, opt).await,
        Command::Loudness { name } => loudness(name).await,
        Command::RemoveTrack { name, track_no } => remove_track(name, track_no).await,
        Command::QueueNext { name, uri, metadata } => queue_next(name, uri, metadata).await,
        Command::QueueEnd {name, uri, metadata } => queue_end(name, uri, metadata).await,
        Command::ClearQueue { name } => clear_queue(name).await,
        Command::Shuffle { name } => shuffle(name).await,
        Command::RepeatAll { name } => repeat_all(name).await,
        Command::RepeatOne { name } => repeat_one(name).await,
        Command::RepeatOff { name } => repeat_off(name).await,
        Command::Join { name, speaker_to_join } => join(name, speaker_to_join).await,
        Command::Leave { name } => leave(name).await,
        Command::Skip { name, seconds } => skip(name, seconds).await,
        Command::SkipTo { name, seconds } => skip_to(name, seconds).await,
    }
}

async fn info() -> Result<(), sonor::Error> {
    let mut devices = sonor::discover(Duration::from_secs(2)).await?;

    while let Some(device) = devices.try_next().await? {
        let name = device.name().await?;
        let speaker = sonor::find(&name, Duration::from_secs(2)).await?
            .expect("room exists");
        match speaker.track().await? {
            Some(track_info) => {
                println!("Room: {}", name);
                println!("Volume: {}", speaker.volume().await?);
                println!("Track: {}", track_info.track());
            }
            None => {
                println!("Room: {}", name);
                println!("Volume: {}", speaker.volume().await?);
            }
        }
        println!("----------");
    }

    Ok(())
}

async fn stop(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.stop().await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn play(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.play().await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn pause(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.pause().await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn next(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.next().await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn previous(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.previous().await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn track(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    match speaker.track().await? {
        Some(track_info) => println!("- Currently playing '{} on '{}", track_info.track(), name),
        None => println!("- No track currently playing on {}", name),
    }
    Ok(())
}

async fn volume(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    match speaker.volume().await {
        Ok(vol) => println!("The volume is currently at {} on {}", vol, name),
        Err(_) => println!("Error"),
    }
    Ok(())
}

async fn set_volume(name: String, volume: u16) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.set_volume(volume).await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn queue(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    match speaker.queue().await {
        Ok(q) => for (i,t) in q.iter().enumerate() { println!("{}. {}", i, t.title()) },
        Err(_) => println!("Empty Queue"),
    }
    Ok(())
}

async fn remove_track(name: String, track_no: u32) -> Result<(), sonor::Error>{
    return match get_speaker(name).await {
        Some(speaker) => speaker.remove_track(track_no).await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn queue_next(name: String, uri: String, metadata: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.queue_next(Box::leak(uri.into_boxed_str()), Box::leak(metadata.into_boxed_str())).await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn queue_end(name: String, uri: String, metadata: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.queue_end(Box::leak(uri.into_boxed_str()), Box::leak(metadata.into_boxed_str())).await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn clear_queue(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.clear_queue().await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn mute(name: String) -> Result<(), sonor::Error> {
    match get_speaker(name).await {
        Some(speaker) => match speaker.mute().await {
            Ok(is_muted) => {
                return speaker.set_mute(!is_muted).await;
            },
            Err(_) => { println!("Error: unable to check if muted"); Ok(()) },
        },
        None => { speaker_not_found(); Ok(()) },
    }
}

async fn bass(name: String, opt: Option<i8>) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    match opt {
        Some(opt) => match speaker.set_bass(opt).await {
            Ok(_) => println!("bass now at: {}", opt),
            Err(_) => println!("Error: could not set bass"),
        }
        None => match speaker.bass().await {
            Ok(bass) => println!("The bass is currently set at {} on {}", bass, name),
            Err(_) => println!("Error: could not get bass"),
        }
    }
    Ok(())
}

async fn treble(name: String, opt: Option<i8>) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    match opt {
        Some(opt) => match speaker.set_treble(opt).await {
            Ok(_) => println!("treble now at: {}", opt),
            Err(_) => println!("Error: could not set treble"),
        }
        None => match speaker.treble().await {
            Ok(bass) => println!("The treble is currently set at {} on {}", bass, name),
            Err(_) => println!("Error: could not get treble"),
        }
    }
    Ok(())
}

async fn loudness(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    match speaker.loudness().await {
        Ok(is_loudness) => {
            return speaker.set_loudness(!is_loudness).await;
        },
        Err(_) => println!("Error: unable to get loudness"),
    }
    Ok(())
}

async fn shuffle(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    match speaker.shuffle().await {
        Ok(on_shuffle) => return speaker.set_shuffle(!on_shuffle).await,
        Err(_) => println!("Error: unable to get shuffle mode"),
    }
    Ok(())
}

async fn repeat_all(name: String) -> Result<(), sonor::Error> {
    match get_speaker(name).await {
        Some(speaker) => speaker.set_repeat_mode(RepeatMode::All).await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn repeat_one(name: String) -> Result<(), sonor::Error> {
    match get_speaker(name).await {
        Some(speaker) => speaker.set_repeat_mode(RepeatMode::One).await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn repeat_off(name: String) -> Result<(), sonor::Error> {
    match get_speaker(name).await {
        Some(speaker) => speaker.set_repeat_mode(RepeatMode::None).await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn join(name: String, speaker_to_join: String) -> Result<(), sonor::Error> {
    match get_speaker(name).await {
        Some(speaker) => match speaker.join(&speaker_to_join).await {
            Ok(joined) => {
                println!("joined {}: {}", speaker_to_join, joined);
                return Ok(());
            }
            Err(_) => Ok(()),
        },
        None => Ok(()),
    }
}

async fn leave(name: String) -> Result<(), sonor::Error> {
    match get_speaker(name).await {
        Some(speaker) => speaker.leave().await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn skip(name: String, seconds: i32) -> Result<(), sonor::Error> {
    match get_speaker(name).await {
        Some(speaker) => speaker.skip_by(seconds).await,
        None => { speaker_not_found(); Ok(())},
    }
}

async fn skip_to(name: String, seconds: u32) -> Result<(), sonor::Error> {
    match get_speaker(name).await {
        Some(speaker) => speaker.skip_to(seconds).await,
        None => { speaker_not_found(); Ok(()) }
    }
}

fn speaker_not_found() {
    println!("No speaker found with that name\ntry using the `info` command to list discoverable devices")
}

async fn get_speaker(name: String) -> Option<Speaker> {
    match sonor::find(&name, Duration::from_secs(2)).await {
        Ok(opt) => return opt,
        Err(err) => {
            eprintln!("Error: {}", err);
            return None;
        },
    }
}