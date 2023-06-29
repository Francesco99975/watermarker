use std::{fs, time::Duration};

use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

pub fn get_dirs_images_paths(dirs: Vec<&String>) -> Vec<String> {
    let mut image_paths: Vec<String> = Vec::new();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::with_template("{spinner:.blue} {msg}")
            .unwrap()
            .tick_strings(&[
                "▹▹▹▹▹",
                "▸▹▹▹▹",
                "▹▸▹▹▹",
                "▹▹▸▹▹",
                "▹▹▹▸▹",
                "▹▹▹▹▸",
                "▪▪▪▪▪",
            ]),
    );
    pb.set_message(format!("{}", "Loading Images...".blue().italic()));

    for dir in dirs {
        let paths = fs::read_dir(dir).expect("Could not get file paths");
        for path in paths {
            let clean_path = path.expect("Path retreival Error").path();

            if clean_path.is_file() {
                image_paths.push(
                    clean_path
                        .to_str()
                        .expect("File Path Parsing error")
                        .to_string(),
                );
            }
        }
    }

    pb.finish_with_message(format!(
        "{} {} {}",
        "Loaded".yellow().bold(),
        image_paths.len(),
        "Images".yellow().bold(),
    ));

    image_paths
}
