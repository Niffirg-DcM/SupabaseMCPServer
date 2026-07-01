//
//  main.rs
//  Supabase MCP Server
//
//  Created by Griffin McDonald on 2026-06-07.
//
mod offer_tools;
mod process_tools;
//this will be a child process for other projects so they can access a given supabase db.
//will potentially take in api key and endpoint to allow for customizability in its domain
fn main() {
    //println!("Hello, world!");

    //offer tools -> listen for inputs and handle them/
    offer_tools::offer();

    // Must offer tools for supabase
    // tools are total entries, last entry, entries from date-date, range of dates.
    // loop {
    //     println!("OUTPUT!!!!!!");
    // }





    // must handle reqests to said tools

    // this will have us looping on stdin to find pipelined inputs

}


