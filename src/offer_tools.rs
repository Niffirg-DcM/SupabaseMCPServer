use serde::{Deserialize, Serialize};
use serde_json::{json,Value};

#[derive(Debug, Serialize, Deserialize)]
struct OutTools {
    #[serde(rename = "functionDeclarations", alias = "function_declarations")]
    function_declarations: Vec<OutDeclarations>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutDeclarations {
    name: String,
    description: String,
    parameters: Value
}



pub fn offer() {
    // format json
    // grab pieces
    // send
    let outgoing_tools = vec![ OutTools {
                function_declarations: vec![ OutDeclarations {
                    name: String::from("First_Entry"),
                    description: String::from("gets the very first entry from the SupabaseDB and the time it was added."),
                    parameters: json!({
                            "type": "Object", 
                            "properties": {},
                        })
                    },
                    OutDeclarations {
                        name: String::from("Last_Entry"),
                        description: String::from("gets the very last entry from the SupabaseDB and the time it was added."),
                        parameters: json!({
                            "type": "Object", 
                            "properties": {},
                        })
                    },
                    OutDeclarations {
                        name: String::from("Range_of_Dates"),
                        description: String::from("gets the range of date between the first and last entries from the SupabaseDB."),
                        parameters: json!({
                            "type": "Object", 
                            "properties": {},
                        })
                    },
                    OutDeclarations {
                        name: String::from("Total_Entries"),
                        description: String::from("gets the number of total entries in the SupabaseDB across all times. Also known as the total number of rows in the db."),
                        parameters: json!({
                            "type": "Object", 
                            "properties": {},
                        })
                    },
                    OutDeclarations {
                        name: String::from("Total_from_dates"),
                        description: String::from("gets the number of entries or rows in the SupabaseDB from datetime x to datetime y."),
                        parameters: json!({
                            "type": "Object", 
                            "properties": {
                                "start_date": {"type": "String", "description": "Starting datetime for the range asked for by the client. Format: YYYY-MM-DDTHH:MM:SS"},
                                "end_date": {"type": "String", "description": "Ending datetime for the range asked for by the client. Format: YYYY-MM-DDTHH:MM:SS"}
                            },
                            "required": ["start_date","end_date"]
                        })
                    }]
            }];

        println!("{}",serde_json::to_string(&outgoing_tools).unwrap());

}