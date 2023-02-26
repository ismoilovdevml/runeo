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

    let https = HttpsConnector::new();

    let client = Client::builder().build(https);

    let uri = "https://api.openai.com/v1/engines/davinci-codex/completions";

    let preamble = "Salom! Men sizga qanday yordam bera olaman.";

    let token: String = env::var("TOKEN").unwrap();

    let auth_header_val = format!("Token: {}", token);

    println!("{esc}c", esc = 27 as char)

    loop {
        print!{">"};
        stdout().flush().unwrap();
        let mut user_text = String::new();

        stdin()
            .read_line(&mut user_text)
            .expect("Xatolik");
            println!("");

        let sp = Spinner::new(&Spinners::Dots12, "\t\tAqqli terminalchaman".into());
        let runeo_request = RuneoRequest{
            promt: format!("{} {}", preamble, user_text),
            max_tokens: 1000,
        };

        

    }

    Ok(())

}
