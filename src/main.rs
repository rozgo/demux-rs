#![feature(await_macro, async_await, futures_api)]

use clap::{App, Arg};
use console::style;
// use indicatif::{ProgressBar, ProgressStyle};

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate tokio;

use tokio::prelude::*;
use hyper::Client;

mod eosio;
mod action;

#[derive(Debug)]
enum FetchError {
    Http(hyper::Error),
    Json(serde_json::Error),
}

impl From<hyper::Error> for FetchError {
    fn from(err: hyper::Error) -> FetchError {
        FetchError::Http(err)
    }
}

impl From<serde_json::Error> for FetchError {
    fn from(err: serde_json::Error) -> FetchError {
        FetchError::Json(err)
    }
}

// fn create_progress_bar(quiet_mode: bool, msg: &str, length: Option<u64>) -> ProgressBar {
//     let bar = match quiet_mode {
//         true => ProgressBar::hidden(),
//         false => {
//             match length {
//                 Some(len) => ProgressBar::new(len),
//                 None => ProgressBar::new_spinner(),
//             }
//         }
//     };

//     bar.set_message(msg);
//     match length.is_some() {
//         true => bar
//             .set_style(ProgressStyle::default_bar()
//                 .template("{msg} {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} eta: {eta}")
//                 .progress_chars("=> ")),
//         false => bar.set_style(ProgressStyle::default_spinner()),
//     };

//     bar
// }

async fn get_json<T>(url: String) -> Result<T, FetchError>
where T: std::fmt::Debug,
 T: serde::de::DeserializeOwned {
    let client = Client::new();
    let uri = url.parse().unwrap();
    await!(
        client.get(uri)
            .and_then(|res| res.into_body().concat2())
            .from_err()
            .and_then(|body| {
                let json = serde_json::from_slice::<T>(&body)?;
                Ok(json)
            })
    )
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

    let api = matches
        .value_of("api")
        .unwrap_or("http://127.0.0.1:8888/v1");
    println!("Using api: {}", style(api).green());

    let url = api.to_owned() + "/chain/get_info";

    let chain = async {
        let chain_info = await!(get_json::<eosio::ChainInfo>(url));
        println!("{:#?}", chain_info);
    };

    tokio::run_async(chain);
}
