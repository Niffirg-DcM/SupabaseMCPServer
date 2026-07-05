use serde::{Deserialize, Serialize};
use serde_json::{json,Value};

#[derive(Debug,Serialize,Deserialize)]
struct Response {
    name: String,
    args: Vec<Value>,
}


//example response {"name":"Total_from_dates","args":{"end_date":"2026-05-12 14:00:00","start_date":"2024-01-01 13:00:00"}}

pub fn process(input: &str) {
    // format json
    // grab pieces
    // send

    //Grabbing response json from string received from client.
    let resp: Response = serde_json::from_str(input).expect("darn");

    //outbound val instantiated for later.
    let out_bound_val:String = String::new();

    //grabbing name as a starting point
    let name = resp.name;

    let out_bound_val = match name.as_str() {
        "First_Entry"=>first_entry(),
        "Last_Entry"=>last_entry(),
        "Range_of_Dates"=>range_of_dates(),
        "Total_Entries"=>total_entries(),
        "Total_from_dates"=> {let end_date = resp.args[0]["end_date"].to_string();
            let start_date = resp.args[1]["start_date"].to_string();
            total_from_dates(start_date,end_date)},
        _=> {println!("Received unknown tool call: {}", name);
            String::from("Error: Unknown tool request.")},
    };



        println!("{}",serde_json::to_string(&out_bound_val).unwrap());

}

fn total_from_dates(start_date:String,end_date:String) -> String {
    let mut api_response = String::new();
    //call api
    //parse response to string
    return api_response;
}

fn first_entry() -> String{
    let mut api_response = String::new();
    //call api
    //parse response to string
    return api_response;
}

fn last_entry() -> String{
    let mut api_response = String::new();
    //call api
    //parse response to string
    return api_response;
}

fn range_of_dates() -> String{
    let mut api_response = String::new();
    //call api
    //parse response to string
    return api_response;
}

fn total_entries() -> String{
    let mut api_response = String::new();
    //call api
    //parse response to string
    return api_response;
}