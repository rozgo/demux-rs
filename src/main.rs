#![feature(await_macro, async_await, futures_api)]

#[macro_use]
extern crate tokio;
// extern crate hyper;

use clap::{Arg, App};
use console::style;
use indicatif::{ProgressStyle, ProgressBar};
use tokio::prelude::*;
use hyper::Client;
use std::time::Duration;
use std::str;

fn req() -> impl std::future::Future<Output=()> {
    async {
        let client = Client::new();

        let bar = create_progress_bar(false, "connecting to api", None);

        let uri = "http://127.0.0.1:8888/v1/chain/get_info".parse().unwrap();

        let response = await!({
            client.get(uri)
                .timeout(Duration::from_secs(10))
        }).unwrap();

        println!("Response: {}", response.status());

        let mut body = response.into_body();

        while let Some(chunk) = await!(body.next()) {
            let chunk = chunk.unwrap();
            bar.inc(1);
            println!("chunk = {}", str::from_utf8(&chunk[..]).unwrap());
        }
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

fn main() {

    let matches =
        App::new("demux-rs")
            .version("0.1.0")
            .about("Demux is a backend infrastructure pattern for sourcing blockchain events to deterministically update queryable datastores and trigger side effects.")
            .author("Alex Rozgo <alex.rozgo@gmail.com>")
            .arg(Arg::with_name("api")
                .short("a")
                .long("api")
                // .value_name("FILE")
                .help("Sets api for EOSIO http_plugin")
                .takes_value(true))
            .get_matches();

    let api = matches.value_of("api").unwrap_or("http://127.0.0.1:8888/v1");
    println!("Using api: {}", style(api).green());

    // println!(
    //     "This is red on black: {:010x}",
    //     style(42).red().on_black().bold()
    // );
    // println!("This is reversed: [{}]", style("whatever").reverse());
    // println!("This is cyan: {}", style("whatever").cyan());

    // tokio::run_async(async {
    //     let client = Client::new();

    //     let uri = "http://httpbin.org/ip".parse().unwrap();

    //     let response = await!({
    //         client.get(uri)
    //             .timeout(Duration::from_secs(10))
    //     }).unwrap();

    //     println!("Response: {}", response.status());

    //     let mut body = response.into_body();

    //     while let Some(chunk) = await!(body.next()) {
    //         let chunk = chunk.unwrap();
    //         println!("chunk = {}", str::from_utf8(&chunk[..]).unwrap());
    //     }
    // });

    // tokio::run_async(async {
    //     println!("Hello");
    // });

    tokio::run_async(req());
}
