use futures::prelude::*;
use std::time::Duration;
use structopt::StructOpt;
use sonor::Speaker;

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
        None => Ok(()),
    }
}

async fn play(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.play().await,
        None => Ok(()),
    }
}

async fn pause(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.pause().await,
        None => Ok(()),
    }
}

async fn next(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.next().await,
        None => Ok(()),
    }
}

async fn previous(name: String) -> Result<(), sonor::Error> {
    return match get_speaker(name).await {
        Some(speaker) => speaker.previous().await,
        None => Ok(()),
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
        None => Ok(()),
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

async fn mute(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    match speaker.mute().await {
        Ok(is_muted) => {
            return speaker.set_mute(!is_muted).await;
        },
        Err(_) => println!("Error: unable to mute"),
    }
    Ok(())
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

async fn get_speaker(name: String) -> Option<Speaker> {
    match sonor::find(&name, Duration::from_secs(2)).await {
        Ok(opt) => return opt,
        Err(_) => {
            println!("Unable to find Speaker `{}, try using the `info` commmand to list devices", name);
            return None;
        },
    }
}