use crate::config::{parse_abi_file, JesperConfigOutput};
use crate::constants;
use crate::find_selector;
use crate::parse_sol_file::{parse_sol_file, SolidityMessageAndArgs, Temp};
use clap::{arg, command, value_parser, ArgAction, Command, Parser};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize, Clone)]
pub struct ABIInput {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub _type: Option<String>,
    #[serde(rename = "internalType")]
    pub internal_type: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SolidityABI {
    #[serde(rename = "type")]
    _type: String,
    name: Option<String>,
    inputs: Option<Vec<ABIInput>>,
}

#[derive(Serialize, Deserialize, Clone)]
struct Output {
    #[serde(rename = "errorName")]
    error_name: String,
    #[serde(rename = "solidityMessageAndArgs")]
    solidity_message_and_args: SolidityMessageAndArgs,
}
pub fn prepare_error_from_abi_input(input: &SolidityABI) -> String {
    let mut error = String::new();
    //Name(input[0].type, input[1].type, ...)
    error.push_str(&input.name.clone().unwrap());
    error.push('(');
    let mut input_types = String::new();
    for input in input.inputs.clone().unwrap() {
        input_types.push_str(&input._type.unwrap());
        input_types.push(',');
    }
    //remove the last comma
    input_types.pop();
    error.push_str(&input_types);
    error.push(')');
    return error;
}
pub fn handle_parse_all_files(
    abi_files: Vec<String>,
    solidity_files: Vec<String>,
    config: &JesperConfigOutput,
) {
    let mut error_name_to_inputs_hashmap: HashMap<String, Vec<ABIInput>> = HashMap::new();
    let (selectors, prepared_errors) =
        get_prepared_errors_from_abi_files(abi_files, config, &mut error_name_to_inputs_hashmap);

    // For each solidity file,aggregate the solidity file
    // let mut identifier_to_error_map: HashMap<String, String> = HashMap::new();
    let (error_to_message_map, identifier_to_error_map): (
        HashMap<String, SolidityMessageAndArgs>,
        HashMap<String, Temp>,
    ) = solidity_files
        .par_iter()
        .map(|sol_file_path| {
            // Assuming parse_sol_file returns Result<(HashMap<String, SolidityMessageAndArgs>, HashMap<String, String>), io::Error>
            parse_sol_file(sol_file_path)
                .unwrap_or_else(|_| panic!("Could not parse the solidity file"))
        })
        .fold(
            || (HashMap::new(), HashMap::new()),
            |mut acc, (err_to_msg, id_to_err)| {
                acc.0.extend(err_to_msg);
                acc.1.extend(id_to_err);
                acc
            },
        )
        .reduce(
            || (HashMap::new(), HashMap::new()),
            |mut acc, next| {
                acc.0.extend(next.0);
                acc.1.extend(next.1);
                acc
            },
        );

    // .reduce(HashMap::new, |mut acc, elem| {
    //     acc.extend(elem);
    //     return acc;
    // });
    // let error_to_error_message_map:HashMap<String,SolidityMessageAndArgs> =

    //Loop over all the selectors and prepared errors
    let mut output: HashMap<String, Output> = selectors
        .into_iter()
        .zip(prepared_errors.into_iter())
        .map(|(selector, error)| {
            let key_in_error_to_message_map = split_on_first_parenthesis(&error);
            let error_message = match error_to_message_map.get(key_in_error_to_message_map) {
                Some(message) => {
                    // Deserialize the JSON string to a regular Rust string
                    let cleaned_error_message =
                        serde_json::from_str::<String>(message.error_message.as_str())
                            .unwrap_or_else(|_| "".to_string());
                    let args = if message.args.len() > 0 {
                        message.args.clone()
                    } else {
                        error_name_to_inputs_hashmap
                            .get(key_in_error_to_message_map)
                            .unwrap_or_else(|| {
                                panic!("Could not find the error {} in the hashmap", error)
                            })
                            .clone()
                            .to_owned()
                    };
                    SolidityMessageAndArgs {
                        error_message: cleaned_error_message,
                        args: args,
                    }
                }
                None => {
                    let key_in_error_to_message_map = split_on_first_parenthesis(&error);
                    let args = error_name_to_inputs_hashmap
                        .get(key_in_error_to_message_map)
                        .unwrap_or_else(|| {
                            panic!("Could not find the error {} in the hashmap", error)
                        })
                        .clone()
                        .to_owned();
                    SolidityMessageAndArgs {
                        //Find in hashmap
                        error_message: "".to_owned(),
                        args: args,
                    }
                }
            };

            let output = Output {
                error_name: error,
                solidity_message_and_args: error_message,
            };

            (selector, output)
        })
        .collect::<HashMap<String, Output>>();

    //For every key,value in the error_to_message_map, if the key is not in the output hashmap, add it
    for (key, value) in error_to_message_map {
        let key = split_on_first_parenthesis(&key).to_string();
        let selector = identifier_to_error_map.get(&key).unwrap_or_else(|| {
            panic!(
                "Could not find the error {} in the identifier_to_error_map",
                key
            )
        });
        let identifier = selector.identifier.clone();
        if !output.contains_key(&identifier) {
            let args = value.args.clone();
            let cleaned_message_with_serde = serde_json::from_str::<String>(&value.error_message)
                .unwrap_or_else(|_| "".to_string());
            let output_value = Output {
                error_name: selector.cleaned_error.clone(),
                solidity_message_and_args: SolidityMessageAndArgs {
                    error_message: cleaned_message_with_serde,
                    args: args,
                },
            };
            output.insert(selector.identifier.clone(), output_value);
        }
    }

    //Save the map
    let map_json = serde_json::to_string_pretty(&output.clone()).unwrap();

    let typescript_prefix = "export const errors:JesperObject = ";
    let typescript_suffix = ";";
    let typescript =
        constants::PREFIX_TEXT.to_owned() + typescript_prefix + &map_json + typescript_suffix;
    let save_path = config.output_folder.clone() + "/jesper-bindings.ts";
    let save_path = save_path.replace("//", "/");
    std::fs::write(save_path, typescript).unwrap();
}

pub fn get_prepared_errors_from_abi_files(
    abi_files: Vec<String>,
    config: &JesperConfigOutput,
    error_name_to_inputs_hashmap: &mut HashMap<String, Vec<ABIInput>>,
) -> (Vec<String>, Vec<String>) {
    let mut prepared_errors: Vec<String> = Vec::new();
    let mut selectors: Vec<String> = Vec::new();

    for (i, abi_fn) in abi_files.iter().enumerate() {
        //read in the abi file
        let abi = parse_abi_file(abi_fn, config);
        match abi {
            None => {
                continue;
            }
            _ => {}
        }
        let abi = abi.unwrap();
        //Find only the errors
        let errors: Vec<SolidityABI> = abi
            .into_iter()
            .filter(|x| x._type == "error")
            .collect::<Vec<SolidityABI>>();

        //For each name, add the abi input
        for error in errors.clone() {
            // let error_name = split_on_first_parenthesis(error.name.clone().unwrap());
            // let inputs = error.inputs.clone().unwrap();
            // error_name_to_inputs_hashmap.insert(error_name.to_string(), inputs);
            let _clone = error.name.clone().unwrap();
            let error_name = split_on_first_parenthesis(&_clone);
            let inputs = error.inputs.clone().unwrap();
            error_name_to_inputs_hashmap.insert(error_name.to_string(), inputs);
        }

        //A list of errors as strings
        let mut prepared_errors_from_file = errors
            .clone()
            .into_iter()
            .map(|x| prepare_error_from_abi_input(&x))
            .collect::<Vec<String>>();

        //A vector of all the selectors
        let mut selectors_from_file = prepared_errors_from_file
            .clone()
            .into_iter()
            .map(|x| find_selector::find_selector(&x))
            .collect::<Vec<String>>();

        //Add the selectors and errors to the main vectors
        selectors.append(&mut selectors_from_file);
        prepared_errors.append(&mut prepared_errors_from_file);
    }
    // //print prepared errors
    // println!("prepared_errors = {:?}", prepared_errors);
    // let

    //Return the selectors and the prepared errors
    return (selectors, prepared_errors);
}

fn split_on_first_parenthesis(input: &str) -> &str {
    return input.split("(").collect::<Vec<&str>>()[0];
    //collect
}
