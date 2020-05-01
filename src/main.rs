use futures::prelude::*;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(rename_all = "kebab-case")]
enum Cli {
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
        about = "Displays the volume for a specified room",
        help = "USAGE: volume MyRoomName"
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
    }
}

#[tokio::main]
async fn main() -> Result<(), sonor::Error> {
    let args = Cli::from_args();
    return match args {
        Cli::Info => info().await,
        Cli::Stop { name } => stop(name).await,
        Cli::Play { name } => play(name).await,
        Cli::Pause { name } => pause(name).await,
        Cli::Next { name } => next(name).await,
        Cli::Previous { name } => previous(name).await,
        Cli::Track { name } => track(name).await,
        Cli::Volume { name } => volume(name).await,
        Cli::SetVolume { name, volume} => set_volume(name, volume).await,
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
                println!("The volume is currently at {} on {}", speaker.volume().await?, name);
                println!("- Currently playing '{} on '{}", track_info.track(), name);
            }
            None => println!("- No track currently playing on {}", name),
        }
    }

    Ok(())
}

async fn stop(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
            .expect("room exists");
    return speaker.stop().await;
}

async fn play(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    return speaker.play().await;
}

async fn pause(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    return speaker.pause().await;
}

async fn next(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    return speaker.next().await;
}

async fn previous(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    return speaker.previous().await;
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
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
        .expect("room exists");
    return speaker.set_volume(volume).await;
}