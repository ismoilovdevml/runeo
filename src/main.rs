use dotenv::dotenv;
use hyper::body::Buf;
use hyper::{header, Body, Client, Request};
use hyper_tls::HttpsConnector;
use serde_derive::{Deserialize, Serialize};
use spinners::{Spinner, Spinners};
use std::env;
use std::io::{stdin, stdout, Write};

#[derive(Deserialize, Debug)]

struct RuneoResponse{
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choises: Vec<RuneoChoise>,
}


#[derive(Deserialize, Debug)]

struct RuneoChoise {
    tetx: String,
    index: u8,
    logprobs: Option<u8>,
    finish_reason: String,
}

#[derive(Serialize, Debug)]

struct RuneoRequest {
    promt: String,
    max_tokens: u16,
}

#[tokio::main]

async fn main() -> Result<(), Box<dyn std::error::Error + send + Sync>> {
dotenv().ok();
}
