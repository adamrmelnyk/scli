use futures::prelude::*;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
enum Cli {
    Info,
    Stop {
        name: String,
    },
    Play {
        name: String,
    },
    Pause {
        name: String,
    },
    Next {
        name: String,
    },
    Previous {
        name: String
    },
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
        Cli::Previous { name }=> previous(name).await,
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
    speaker.stop().await?;

    Ok(())
}

async fn play(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
    .expect("room exists");
    speaker.play().await?;
    Ok(())
}

async fn pause(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
    .expect("room exists");
    speaker.pause().await?;
    Ok(())
}

async fn next(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
    .expect("room exists");
    speaker.next().await?;
    Ok(())
}

async fn previous(name: String) -> Result<(), sonor::Error> {
    let speaker = sonor::find(&name, Duration::from_secs(2)).await?
    .expect("room exists");
    speaker.previous().await?;
    Ok(())
}