//
//  main.rs
//  Supabase MCP Server
//
//  Created by Griffin McDonald on 2026-06-07.
//
mod offer_tools;
mod process_tools;
use std::io::{self, Read, Write};
//this will be a child process for other projects so they can access a given supabase db.
//will potentially take in api key and endpoint to allow for customizability in its domain
fn main() {
    //println!("Hello, world!");

    //offer tools -> listen for inputs and handle them/
    offer_tools::offer();

    // Must offer tools for supabase
    // tools are total entries, last entry, entries from date-date, range of dates.
    let stdin = io::stdin();
    let mut input: String = String::new();
    loop {
        stdin.read_line(&mut input).expect("failed");

        let trimmed = input.trim();
        

        process_tools::process(trimmed);


    }







    // must handle reqests to said tools

    // this will have us looping on stdin to find pipelined inputs

}


