use clap::Parser;
use reqwest::{self, StatusCode};
use tokio;

#[derive(Parser, Default, Debug)]
#[clap(author = "dmanjunath", version = "0.0.1", about = "eip-lookup")]
struct Arguments {
    eip_id: String,

    #[arg(short, long)]
    browser: bool,
}

#[tokio::main]
async fn main() {
    let args = parse_clap_args();
    let eip_id = args.eip_id;
    let browser = args.browser;

    let raw_url = make_raw_url(&eip_id);

    match check_eip_exists(&raw_url).await {
        Ok(true) => (),
        Ok(false) => {
            eprintln!("EIP {} does not exist", eip_id);
            std::process::exit(1);
        }
        Err(e) => println!("Could not make a request because of {e}"),
    }

    // open url in browser
    if browser {
        let github_url = make_github_url(&eip_id);
        open_in_browser(&github_url);
    } else {
        match fetch_body(&raw_url).await {
            Ok((body, _)) => println!("{body}", body = body),
            Err(e) => println!("Could not make a request because of {e}"),
        }
    }
}

fn parse_clap_args() -> Arguments {
    let args = Arguments::parse();
    // println!("Clap arguments: {:?}", args);
    args
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

fn open_in_browser(url: &String) {
    match open::that(url) {
        Ok(()) => println!("Opened '{}' successfully.", url),
        Err(err) => panic!("An error occurred when opening '{}': {}", url, err),
    }
}

async fn check_eip_exists(url: &String) -> Result<bool, reqwest::Error> {
    let client = reqwest::Client::new();
    let response = client.head(url).send().await?;

    // get status code from response
    let status = response.status();
    if status == StatusCode::from_u16(200).unwrap() {
        return Ok(true);
    } else {
        return Ok(false);
    }
}

async fn fetch_body(url: &String) -> Result<(String, StatusCode), reqwest::Error> {
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

    Ok((body, status))
}
