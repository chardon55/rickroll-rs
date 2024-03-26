use std::{fs::File, io::Write, path::Path};

use curl::easy::Easy;

fn main() {
    let mut client = Easy::new();
    let target = Path::new("./rick-roll.mp4");

    if target.is_dir() {
        // Why did you create a directory ahead of me?!
        std::fs::remove_dir_all(target)
            .expect_err("Failed to remove obstructed directory.");
    }

    if !target.exists() {
        client.url("https://archive.org/download/rick-roll/Rick%20Roll.mp4").unwrap();
        client.write_function(move |data| {
            let mut file = File::create(target).unwrap();

            file.write_all(data).unwrap();

            Ok(data.len())
        }).unwrap();

        client.perform().unwrap();
    }
}
