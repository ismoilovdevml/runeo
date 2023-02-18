use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]

struct RuneoChoise {
    tetx: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

fn main(){
    println!("Runeo")
}