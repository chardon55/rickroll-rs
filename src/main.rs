#[cfg(feature = "offline")]
use std::env;

use clap::Parser;
use tokio::{fs, process::Command};
#[cfg(feature = "offline")]
use tokio::{fs::File, io::AsyncWriteExt};
#[cfg(feature = "offline")]
use uuid::Uuid;

const ONLINE_URL: &str = "https://archive.org/download/rick-roll/Rick%20Roll.mp4";

#[derive(Parser)]
#[clap(version, about)]
struct Cli {
    #[cfg(feature = "offline")]
    #[arg(short, long, help = "Use online resources (namely from the Internet Archive)")]
    online: bool,
}

#[tokio::main]
async fn main() -> tokio::io::Result<()> {

    let _args = Cli::parse();

    #[cfg(not(feature = "offline"))]
    let path_str: String = ONLINE_URL.to_string();

    #[cfg(feature = "offline")]
    let path_str: String = if _args.online {
        println!("[info] Using online resources.");
        "https://archive.org/download/rick-roll/Rick%20Roll.mp4".to_string()
    } else {
        print!("[info] Generating random file name... ");

        let file_name = Uuid::new_v4();

        let mut output_path = env::temp_dir();
        output_path.push(format!("{}.mp4", file_name.to_string()));


        println!("done");

        print!("[info] Releasing video... ");

        // Transient video data embedded in the binary
        let video_file: &[u8] = include_bytes!("../rick-roll.mp4");

        let mut file = File::create(&output_path).await?;
        file.write_all(video_file).await?;

        println!("done");

        output_path.to_str()
            .unwrap()
            .to_string()
    };


    println!("Calling mpv to roll!");

    Command::new("mpv")
        .args(&["--fs", path_str.as_str()])
        .spawn()
        .unwrap()
        .wait()
        .await?;

    fs::remove_file(path_str).await?;

    Ok(())
}

