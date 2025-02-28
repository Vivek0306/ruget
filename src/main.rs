extern crate clap;
extern crate indicatif;
extern crate reqwest;
extern crate tokio;

use clap::{Arg, App};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::error::Error;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;


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

    let response = client.get(url).send().await?;

    println!("Status: {}", response.status());
    if let Some(content_length) = response.content_length(){
        println!("Content-Length: {} bytes", content_length);
    }else{
        println!("Content-Length: Unknown");
    }
    // dbg!(response);

    let body = response.text().await?;
    
    let mut filename = url.split('/').last().unwrap_or("index.html").to_string();

    if !filename.ends_with(".html") {
        filename.push_str(".html");
    }

    let mut file = File::create(filename.clone()).await?;
    file.write_all(body.as_bytes()).await?;
    println!("File Saved: {}", filename);

    Ok(())    
}