use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct OutTools {
    #[serde(rename = "functionDeclarations", alias = "function_declarations")]
    function_declarations: Vec<OutDeclarations>,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutDeclarations {
    name: String,
    description: String,
    parameters: OutParams,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutParams {
    r#type: String,
    properties: OutProps
}

#[derive(Debug, Serialize, Deserialize)]
struct OutProps {
    location: OutLocation,
}

#[derive(Debug, Serialize, Deserialize)]
struct OutLocation {
    r#type: String,
}


pub fn offer() {
    // format json
    // grab pieces
    // send
    let outgoing_tools = vec![ OutTools {
                function_declarations: vec![ OutDeclarations {
                    name: String::from("First_Entry"),
                    description: String::from("gets the time of the very first Entry from the SupabaseDB."),
                    parameters: OutParams {
                            r#type: String::from("OBJECT"), 
                            properties: OutProps { 
                                location:  OutLocation {
                                    r#type: String::from("STRING")
                                } 
                            }
                        }
                    }]
            }];

        println!("{}",serde_json::to_string(&outgoing_tools).unwrap());

}