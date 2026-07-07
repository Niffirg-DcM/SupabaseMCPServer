use reqwest::{Response};
use serde::{Deserialize, Serialize};
use serde_json::{Value};
use dotenvy::dotenv;
use std::{time::Duration};


#[derive(Debug,Serialize,Deserialize)]
struct ClientResponse {
    name: String,
    args: Vec<Value>,
}


//example response {"name":"Total_from_dates","args":{"end_date":"2026-05-12 14:00:00","start_date":"2024-01-01 13:00:00"}}

pub async fn process(input: &str) -> Result<String,Box<dyn std::error::Error>> {
    // format json
    // grab pieces
    // send
    dotenv().expect(".env file not found");


    //ensure api key is there
    let env_url = env::var("SUPABASE_URL")?;
    let env_sec = env::var("SUPABASE_ANON_KEY")?;
    let env_table = env::var("SUPABASE_MAIN_TABLE")?;
    let env_view = env::var("SUPABASE_MAIN_VIEW")?;


    //instantiate client request
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;

    //Grabbing response json from string received from client.
    let resp: ClientResponse = serde_json::from_str(input).expect("darn");

    //grabbing name as a starting point
    let name = resp.name;

    //payload for url, created based off required table and data requested by LLM.
    let payload = match name.as_str() {
        "First_Entry"=>format!("{}?select=created_at&order=created_at.asc&limit=1",env_table),
        "Last_Entry"=>format!("{}?select=created_at&order=created_at.desc&limit=1",env_table),
        "Range_of_Dates"=>format!("{}?select=first_date,last_date",env_view),
        "Total_Entries"=>format!("{}?select=*&limit=0",env_table),
        "Total_from_dates"=> {let end_date = resp.args[0]["end_date"].to_string();
            let start_date = resp.args[1]["start_date"].to_string();
            format!("{}?created_at=gte.{}&created_at=lt.{}&select=id&limit=1",env_table,start_date,end_date)},
        _=> {format!("{}{}",env_table,name)},
    };

    //formatting url with payload and base from env.
    let url = format!("{}{}",env_url, payload);

    //send get and gather response
    let post_response:Response = client.get(url)
        .header("apikey", &env_sec)
        .header("Authorization", format!("Bearer {}",&env_sec))
        .header("Content-Type", "application/json")
        .header("Prefer", "Count=exact")
        .send()
        .await?;


    //if successful post then parse response to send back to main
    if post_response.status().is_success() {

        //parse and return Json response
        let out_bound_val = post_response.text().await?;
        Ok(out_bound_val.into())
        
    } else {
        //parse and return json error
        let error_body = post_response.text().await?;
        Err(error_body.into())
    }

}
