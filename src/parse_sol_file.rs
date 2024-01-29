use crate::find_selector::find_selector;
use crate::handle_parse_all_files::ABIInput;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io;

#[derive(Serialize, Deserialize, Clone)]
pub struct SolidityMessageAndArgs {
    #[serde(rename = "errorMessage")]
    pub error_message: String,
    pub args: Vec<ABIInput>,
}
fn cleanup_text_input(input: &str) -> String {
    let m = input.split_whitespace().collect::<Vec<&str>>().join(" ");
    let m = m.replace(", ", ",");
    let m = m.replace("( ", "(");
    let m = m.replace(" )", ")");

    let m = m.as_str().trim().replace(";", "").replace("error ", "");

    return m;
}

pub struct Temp {
    pub cleaned_error: String,
    pub identifier: String,
}
pub fn parse_sol_file(
    file_path: &str,
) -> Result<
    (
        HashMap<String, SolidityMessageAndArgs>,
        HashMap<String, Temp>,
    ),
    io::Error,
> {
    let registry_path = "./contracts/Registry.sol";
    let is_registry = file_path == registry_path;
    let file_contents = fs::read_to_string(file_path)?;

    //get all lines by iterating over the lines
    let lines: Vec<String> = file_contents
        .lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>();
    let mut identifier_to_selector: HashMap<String, Temp> = HashMap::new();

    let error_pattern = r"error\s+\w+\s*\([^;]*\);";
    let re = Regex::new(error_pattern).expect("Failed to compile regex");
    let line_indicators = re
        .captures_iter(&file_contents)
        .filter_map(|cap| cap.get(0))
        .map(|mat| {
            let mat = cleanup_text_input(mat.clone().as_str());
            let identifier = mat.split("(").collect::<Vec<&str>>()[0];
            // println!("identifier = {}", identifier);
            //insert identifier to selector
            let selector = find_selector(identifier);
            identifier_to_selector.insert(
                identifier.to_string(),
                Temp {
                    cleaned_error: mat.clone(),
                    identifier: selector.clone(),
                },
            );

            //insert it into the map
            // println!("identifier = {}", identifier);
            return (identifier.to_string(), mat.clone());
        })
        .collect::<HashMap<String, String>>();

    // //print all line indicators
    // for (key, val) in line_indicators.iter() {
    //     println!("{} -> {}", key, val);
    // }
    //print cleaned
    let mut error_to_error_message: HashMap<String, SolidityMessageAndArgs> = HashMap::new();

    //do the above, but use indexes so we can get line above
    for (i, line) in lines.iter().enumerate() {
        let cleaned_line = cleanup_text_input(line.as_str());

        let parts: Vec<&str> = cleaned_line.split("(").collect();
        if let Some(indicator) = parts.get(0) {
            let in_map = line_indicators.get(indicator.clone());
            // Rest of your logic...
            let mut past_line_counter = 0;

            if in_map.is_some() && i > 0 {
                let past_line = lines.get(i - 1);
                match past_line {
                    None => {}
                    _ => {}
                }

                if is_comment_line(past_line.as_ref().unwrap()) {
                    while past_line_counter < 10 {
                        past_line_counter += 1;
                        if i - past_line_counter == 0 {
                            break;
                        }
                        let past_line = lines.get(i - past_line_counter);
                        match past_line {
                            None => {}
                            _ => {}
                        }
                        let past_line = past_line.unwrap();
                        let (is_message, message) = is_message_line(past_line);
                        if is_message {
                            //Add it to the map
                            let key = indicator.to_string();
                            let val_in_map = line_indicators.get(key.as_str());
                            let cleaned_error = val_in_map.unwrap().clone();
                            let args = parse_args_from_cleaned_error(cleaned_error);
                            error_to_error_message.insert(
                                indicator.to_string(),
                                SolidityMessageAndArgs {
                                    error_message: message.unwrap(),
                                    args: args,
                                },
                            );
                            break;
                        }
                    }
                } else {
                    //insert it with an empty message
                    let key = indicator.to_string();
                    let val_in_map = line_indicators.get(key.as_str());
                    let cleaned_error = match val_in_map {
                        Some(val) => val.clone(),
                        None => {
                            println!("Could not find error {}", key);
                            "".to_string()
                        }
                    };
                    let args = parse_args_from_cleaned_error(cleaned_error);
                    error_to_error_message.insert(
                        indicator.to_string(),
                        SolidityMessageAndArgs {
                            error_message: "".to_string(),
                            args: args,
                        },
                    );
                }
            }
        }
    }
    //Print out the entire error_to_error_message_map
    // for (error, error_message) in error_to_error_message.iter() {
    //     println!("{} -> {:?}", error, error_message);
    // }

    Ok((error_to_error_message, identifier_to_selector))
}

fn is_comment_line(line: &str) -> bool {
    let line = line.trim();
    if line.starts_with("//") {
        return true;
    }
    if (line.contains("*")) {
        return true;
    }
    return false;
}

fn is_message_line(line: &str) -> (bool, Option<String>) {
    let line = line.trim();
    let lower_line = line.to_lowercase();
    if lower_line.starts_with("//#message:") || lower_line.starts_with("//# message:") {
        let message_without_comment = line.replace("//", "");
        let message_without_comment = message_without_comment.replace("*", "");
        let message_without_comment = message_without_comment
            .replace("#message: ", "")
            .replace("#message: ", "");
        let message_without_comment = message_without_comment.replace("#Message: ", "");
        let message_without_comment = message_without_comment.replace("# Message: ", "");
        let message_without_comment = message_without_comment.trim();
        return (true, Some(message_without_comment.to_string()));
    }
    return (false, None);
}

pub fn parse_args_from_cleaned_error(line: String) -> Vec<ABIInput> {
    let split = line.split("(").collect::<Vec<&str>>();

    let line = split[1].replace(")", "");
    // println!("line = {}", line);

    if line.len() == 0 {
        return vec![];
    }

    //split by comma to find all the args
    let solidity_args = line.split(",").collect::<Vec<&str>>();
    let mut args: Vec<ABIInput> = Vec::new();
    for arg in solidity_args.into_iter() {
        let arg = arg.trim();
        let arg_parts = arg.split(" ").collect::<Vec<&str>>();
        let arg_type = arg_parts.get(0).unwrap();
        let arg_name = arg_parts.get(1).unwrap_or(&"");
        let solidity_arg = ABIInput {
            _type: Some(arg_type.to_string()),
            name: Some(arg_name.to_string()),
            internal_type: Some(arg_type.to_string()),
        };
        args.push(solidity_arg);
    }

    return args;
}
