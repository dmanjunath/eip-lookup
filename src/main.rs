use std::fmt::format;

use reqwest::{self, StatusCode};
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide an eip_id. Usage `eip-lookup <eip_id>`");
        std::process::exit(1);
    }
    let eip_id = &args[1];

    // println!(" outputs {eip_id} ");

    let raw_url = make_raw_url(&eip_id);
    match make_req(&raw_url).await {
        Ok(()) => (),
        Err(e) => println!("Could not make a request because of {e}"),
    }

    // open url in browser
    let github_url = make_github_url(&eip_id);
    match open::that(&github_url) {
        Ok(()) => println!("Opened '{}' successfully.", &github_url),
        Err(err) => panic!("An error occurred when opening '{}': {}", &github_url, err),
    }
}

fn make_raw_url(eip_id: &str) -> String {
    let url = String::from(format!(
        "https://raw.githubusercontent.com/ethereum/EIPs/master/EIPS/eip-{}.md",
        eip_id
    ));
    url
}

fn make_github_url(eip_id: &str) -> String {
    let url = String::from(format!(
        "https://github.com/ethereum/EIPs/blob/master/EIPS/eip-{}.md",
        eip_id
    ));
    url
}

async fn make_req(url: &String) -> Result<(), reqwest::Error> {
    let response = reqwest::get(url).await?;
    // get status code from response
    let status = response.status();
    if !(status == StatusCode::from_u16(200).unwrap()) {
        eprintln!(
            "Status code is not 200, received {status}",
            status = status.to_string()
        );
        std::process::exit(1);
    }

    // get body from response
    let body = response.text().await?;

    println!("{body}");

    Ok(())
}
