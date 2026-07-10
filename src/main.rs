//
//  main.rs
//  Supabase MCP Server
//
//  Created by Griffin McDonald on 2026-06-07.
//

use std::io::{self, BufRead};
mod offer_tools;
mod process_tools;
//this will be a child process for other projects so they can access a given supabase db.
//will potentially take in api key and endpoint to allow for customizability in its domain
fn main() {
    //println!("Hello, world!");

    //offer tools -> listen for inputs and handle them/
    offer_tools::offer();
    let stdin = io::stdin();
    let reader = stdin.lock();

    //need to handle aysnc response and print it to pipe.
    let rt = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()
    .unwrap();

    //grab input from reader.
    for result in reader.lines() {
        //parse reader line for valid input
        let line = match result {
            Ok(l)=>l,
            Err(_)=>continue,
        };
        //trim off excess
        let trimmed = line.trim();
        //if blank them move on to next input.
        if trimmed.is_empty() {
            continue;
        }
        //attempt to parse the trimmed value into input tool request struct.
        match serde_json::from_str::<process_tools::ToolRequest>(trimmed) {
            //if valid, then block and call process tools.
            Ok(valid_req)=>{
                rt.block_on(async {

                    match process_tools::process(valid_req).await {
                        Ok(e)=>println!("{}",e),
                        Err(err)=>eprintln!("Server failed to parse incoming data: {}", err)
                    }
            
                })
            },
            Err(e)=>{eprintln!("Server failed to parse incoming data: {}", e);}
        }
    }

}


