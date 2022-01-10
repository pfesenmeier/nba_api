use reqwest::Error;
use serde::Deserialize;
mod today;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Links {
    league_schedule: String,
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let request_url = "http://data.nba.net/10s/prod/v1/today.json";
    println!("{}", request_url);
    let response = reqwest::get(request_url).await?.json::<today::Today>().await?;

    println!("{:?}", response);
    Ok(())
}
