use futures::prelude::*;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    command: String
}

#[tokio::main]
async fn main() -> Result<(), sonor::Error> {
    let args = Cli::from_args();
    match args.command.as_str() {
         "info" => {
            //  TODO: Split this into it's own method
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

        },
        _ => {
            println!("Command was wrong");
            Ok(())
        }
    }
}
