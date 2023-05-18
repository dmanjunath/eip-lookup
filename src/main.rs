use reqwest::{self};
use tokio;

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let eip_id = args.get(1).ok_or("No EIP id passed in").unwrap();

    // println!(" outputs {eip_id} ");

    match make_req(&eip_id).await {
        Ok(()) => (),
        Err(e) => println!("Could not make a request because of {e}")
    }
}

async fn make_req(eip_id: &&String) -> Result<(),reqwest::Error> {
    let mut url = String::from("https://raw.githubusercontent.com/ethereum/EIPs/master/EIPS/eip-");
    url.push_str(eip_id);
    url.push_str(".md");
    
    // println!("url {url}");
    let response = reqwest::get(url).await?;
    let body = &response.text().await?;

    println!("{body}");

    Ok(())
}
