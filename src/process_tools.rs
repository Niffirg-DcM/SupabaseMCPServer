use reqwest::{Response};
use serde::{Deserialize};
use serde_json::{Value};
use dotenvy::dotenv;
use std::{time::Duration};
#[derive(Deserialize)]
pub struct ToolRequest {
    name: String,
    args: Value,
}

//example response {"name":"Total_from_dates","args":{"end_date":"2026-05-12 14:00:00","start_date":"2024-01-01 13:00:00"}}

pub async fn process(resp: ToolRequest) -> Result<String,Box<dyn std::error::Error>> {
    // format json
    // grab pieces
    // send
    dotenv()?;

    //ensure api key is there
    let env_url = env::var("SUPABASE_URL")?;
    let env_sec = env::var("SUPABASE_ANON_KEY")?;
    let env_table = env::var("SUPABASE_MAIN_TABLE")?;
    let env_view = env::var("SUPABASE_MAIN_VIEW")?;

    //instantiate client request
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()?;
    //grabbing name as a starting point
    let name:String = resp.name;
    
    //payload for url, created based off required table and data requested by LLM.
    let payload = match name.as_str() {
        "First_Entry"=>format!("{}?select=created_at&order=created_at.asc&limit=1",env_table),
        "Last_Entry"=>format!("{}?select=created_at&order=created_at.desc&limit=1",env_table),
        "Range_of_Dates"=>format!("{}?select=first_date,last_date",env_view),
        "Total_Entries"=>format!("{}?select=id",env_table),
        "Total_from_dates"=> {let end_date = resp.args["end_date"].as_str().unwrap().to_string();
            let start_date = resp.args["start_date"].as_str().unwrap().to_string();
            format!("{}?created_at=gte.{}&created_at=lt.{}&select=id",env_table,start_date,end_date)},
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
        if (name.as_str() == "Total_Entries" || name.as_str() == "Total_from_dates") && let Some(range_header) = post_response.headers().get("Content-Range") {
            let range = range_header.to_str()?;
            if let Some(first_iter) = range.split('/').nth(0) {
                if let Some (count_str) = first_iter.split('-').nth(1) {
                    if count_str == "*" {
                        return Ok("0".to_string());
                    }
                    return Ok(count_str.to_string());
                }
            }
        }
        let out_bound_val = post_response.text().await?;
        println!("{}",out_bound_val);
        Ok(out_bound_val)
        //Ok(val.to_string())
        
    } else {
        //parse and return json error
        let error_body = post_response.text().await?;
        Err(error_body.into())
    }

}
