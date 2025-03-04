extern crate clap;
extern crate indicatif;
extern crate reqwest;
extern crate tokio;

use clap::{Arg, App};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::error::Error;
use std::path::Path;
use tokio::fs;
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
            .help("URL to be fetched.")
        )
        .arg(Arg::with_name("OUTPUT")
            .short("o")
            .long("output")
            .required(false)
            .takes_value(true)
            .help("Specify output filename.")
        )
        .arg(Arg::with_name("QUIET")
            .short("q")
            .long("quiet")
            .required(false)
            .takes_value(false)
            .help("Silence the download notifications.")
        )
        .get_matches();

    let url = matches.value_of("URL").unwrap();

    let mut filename = matches.value_of("OUTPUT").map(String::from).unwrap_or_else(|| {
        let name = url.trim_end_matches('/').split('/').last().unwrap_or("index.html").to_string();
        name
    });


    if let Some(output) = matches.value_of("OUTPUT"){
        if !output.contains("."){
            if let Some(ext) = url.trim_end_matches("/").split('.').last(){
                filename.push_str(&format!(".{}", ext));
            }
        }else if !filename.contains('.'){
            filename.push_str(".html")
        }
    }

    let quiet_mode = matches.is_present("QUIET");

    if let Err(e) = download(url, &filename, quiet_mode).await {
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

async fn download(url: &str, filename: &str, quiet: bool) -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    let mut response = client.get(url).send().await?;
    
    println!("Status: {}", response.status());

    let content_length = response.content_length();
    let quiet_mode = quiet;
    let bar = create_progress_bar(quiet_mode, "Downloading...", content_length);

    // Ensure 'files' directory exists
    let folder_path = "files";
    if !Path::new(folder_path).exists() {
        fs::create_dir_all(folder_path).await?;
    }

    // Prepend 'files/' to filename
    let file_path = format!("{}/{}", folder_path, filename);
    
    let mut file = File::create(&file_path).await?;
    let mut downloaded: u64 = 0;

    while let Some(chunk) = response.chunk().await? {
        file.write_all(&chunk).await?;
        downloaded += chunk.len() as u64;
        bar.set_position(downloaded);
    }

    bar.finish_with_message("Download complete!");
    println!("File Saved: {}", file_path);
    
    Ok(())
}