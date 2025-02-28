extern crate clap;
extern crate indicatif;
extern crate reqwest;
extern crate tokio;

use clap::{Arg, App};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::error::Error;
use tokio::fs::File;
use tokio::io::{AsyncWriteExt};


#[tokio::main]
async fn main() {
    let matches = App::new("Ru_get")
        .version("0.1.0")
        .author("Vivek Nair <vivekmanju53@gmail.com>")
        .about("wget but written in Rust")
        .arg(Arg::with_name("URL")
            .short("u")
            .long("url")
            .required(true)
            .takes_value(true)
            .help("URL to be fetched!")
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();

    if let Err(e) = download(url).await {
        eprintln!("Error: {}", e);
    }

}

fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
    let bar = match quiet_mode {
        true => ProgressBar::hidden(),
        false => {
            match length {
                Some(len) => ProgressBar::new(len),
                None => ProgressBar::new_spinner(),
            }
        }
    };

    bar.set_message(msg);
    match length.is_some() {
        true => bar
            .set_style(ProgressStyle::default_bar()
                .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
                .progress_chars("=> ")),
        false => bar.set_style(ProgressStyle::default_spinner()),
    };
    bar
}

async fn download(url: &str) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(url).send().await?;
    
    println!("Status: {}", response.status());

    let content_length = response.content_length();
    let quiet_mode = false;
    let bar = create_progress_bar(quiet_mode, "Downloading...", content_length);

    // Set filename
    let mut filename = url.split('/').last().unwrap_or("index.html").to_string();
    if !filename.ends_with(".html") {
        filename.push_str(".html");
    }

    let mut file = File::create(&filename).await?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        bar.set_position(downloaded);
    }

    bar.finish_with_message("Download complete!");
    println!("File Saved: {}", filename);
    
    Ok(())
}